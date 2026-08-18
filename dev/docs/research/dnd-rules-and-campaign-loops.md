---
status: pending
era: August Activity-Property-Trait
---

# D&D rules and durable campaign loops

> **Role / side:** sourced research report / development side.
> **Authority:** records this report's question, sources, findings and implications.
> **Excludes:** product decisions and current implementation contracts; see `game/docs/`.

Date: 2026-08-13

Status: research findings and candidate heuristics, not Aicadia product direction

## Question

Which concrete Dungeons & Dragons play loops and rules help campaign play remain
durable across scenes, encounters, adventures and sessions? What can Aicadia learn
from D&D's treatment of social interaction, exploration, combat, uncertain outcomes,
time scale, rests, resource pressure, random tables, advancement and rewards without
copying D&D's scores, levels, omnipotent Game Master or content model?

This report asks what transfers as a **design heuristic**. It makes no product or
contract decision.

## Aicadia boundary used for this research

Current Aicadia has one persistent `World`, durable Users and shared Entities, at
most one Character per User, and at most one entry Place. A Character can enter that
Place, inspect local Entities and Activity, and perform one confirmed trail-marker
action. That action atomically adds one placed Entity and immutable prose/history at
the Character's exact current Place. Before submission, the Agent grounds itself in
World reads, presents exactly three private proposals, incorporates steering, shows
the complete consequence and requires explicit User confirmation. The server is
deterministic and never interprets prose with an LLM.
([current build contract](../../../game/docs/README.md),
[Agent play contract](../../../game/docs/agent.md))

Movement, further Places, discovery, investigation rolls, scores, clocks, generic
actions, combat, nonplayer characters and autonomous World narration are absent or
deferred. The backlog lists a first investigation roll and first discovery commit as
later outcomes, not accepted behavior.
([development backlog](../../../dev/backlog/README.md))

That narrow boundary matters. A useful D&D pattern is not automatically a missing
Aicadia feature.

## Source and edition boundary

The main rules source is Wizards of the Coast's current **SRD 5.2.1** and the current
D&D Beyond Basic Rules derived from it. Wizards identifies SRD 5.2.1 as the current
open rules foundation and publishes it under CC BY 4.0.
([official SRD page](https://www.dndbeyond.com/srd),
[SRD 5.2.1 PDF](https://media.dndbeyond.com/compendium-images/srd/5.2/SRD_CC_v5.2.1.pdf))

The 2024 *Dungeon Master's Guide* is not freely readable in full. Its official table
of contents and official D&D Beyond explanatory material are used only for the
campaign/adventure structures they directly expose. The historical 2014 Basic Rules
are used once, explicitly as older-edition evidence about the coupling between
encounter load and rests. Wizards states that the 2024 *Dungeon Master's Guide*
replaces the 2014 book when the updated rules are used, so the older adventuring-day
numbers are not treated as current universal guidance.
([2024 DMG contents](https://www.dndbeyond.com/sources/dnd/dmg-2024),
[official 2024 DMG update](https://www.dndbeyond.com/posts/1916-updates-in-the-dungeon-masters-guide-2024))

## Findings

### 1. The smallest D&D loop is scene, declared action, resolved result, new scene

**Observable D&D pattern.** The current SRD names social interaction, exploration
and combat as the three main pillars, but all three run on one smaller rhythm: the GM
describes a scene, players describe what their characters do, and the GM narrates the
result. That result normally creates another decision point and returns play to the
first step. Combat adds stricter turns, but does not replace this basic loop.
([SRD 5.2.1, “Rhythm of Play,” pp. 5–6](https://media.dndbeyond.com/compendium-images/srd/5.2/SRD_CC_v5.2.1.pdf))

**Transferable heuristic.** A durable roleplaying loop needs four legible boundaries:

1. what is presently true;
2. what the actor intends to do;
3. what authoritative process resolves it; and
4. what is newly true and actionable afterward.

The continuation comes from the changed situation, not from an abstract “next turn”
button or an inexhaustible narrator.

**Possible Aicadia application.** The existing trail-marker flow already contains
most of this shape: exact-Place reads establish the scene, private proposals and
steering establish intent, `World` validates one typed consequence, and the accepted
Entity plus Activity become the next shared scene. A future slice could be evaluated
by whether its accepted result creates a truthful next decision for this or another
Character. This is an evaluation lens, not a proposal to add a universal action
engine.

**Does not fit.** Aicadia should not copy the GM as a hidden, omniscient authority who
may invent any missing fact. `World` owns deterministic state resolution; the User's
Agent owns private reasoning and presentation; accepted World records remain the
only live-game facts. Nor do the three D&D pillars justify `social`, `exploration` and
`combat` tables or mode fields. They can remain design lenses until concrete
Aicadia behavior needs a model.

### 2. D&D changes resolution zoom when the density and consequence of decisions change

**Observable D&D pattern.** Current travel rules explicitly allow the GM either to
summarize trips lasting hours or days or to use travel-pace rules. When every second
matters, play switches to combat movement. Combat then uses roughly six-second rounds,
initiative and one turn per participant. Even within exploration, time pressure makes
object interaction more granular. D&D therefore does not run one clock or one action
granularity for every situation.
([SRD 5.2.1, “Travel” and “Combat,” pp. 12–14](https://media.dndbeyond.com/compendium-images/srd/5.2/SRD_CC_v5.2.1.pdf))

**Transferable heuristic.** Spend resolution detail where choices are close together,
contested or costly. Compress uneventful continuity. A time scale is useful only when
it distinguishes materially different choices or consequences.

**Possible Aicadia application.** The current marker action is correctly resolved as
one atomic package rather than as micro-turns for reaching, writing and placing. If
future travel or investigation is selected, this pattern suggests first identifying
the actual decision boundary—destination, route, pace, risk, target, or another
concrete choice—before adding elapsed time. The activity history can preserve the
accepted result without pretending every conversational exchange is game time.

**Does not fit.** A universal round system, real-time tick, action-point budget or
always-running World clock would add machinery without a current choice to resolve.
D&D's six-second combat round is a local procedure, not evidence that a persistent
world should simulate continuously.

### 3. Dice are gated by meaningful uncertainty, not used as a universal creativity source

**Observable D&D pattern.** SRD 5.2.1 says an ability check is used when a non-attack
task has a chance of meaningful failure, and dice determine the result when the
outcome is both uncertain and narratively interesting. Straightforward actions can
simply succeed in the rhythm-of-play example. A d20 test then has an explicit target
number, modifiers and binary comparison; the kind of check follows from the actual
challenge.
([SRD 5.2.1, “D20 Tests” and “Ability Checks,” pp. 6–7](https://media.dndbeyond.com/compendium-images/srd/5.2/SRD_CC_v5.2.1.pdf))

**Transferable heuristic.** Before adding a roll, state:

- the concrete attempted outcome;
- why success is uncertain;
- what meaningful result follows success;
- what meaningful result follows failure; and
- which established actor or situation changes the odds.

If failure changes nothing, blocks the only path, or merely asks the player to repeat
the request, randomness is not earning its place.

**Possible Aicadia application.** This is a strong test for the backlog's deferred
investigation roll. A retry-stable zero or volatile positive result would only become
play if zero and positive establish distinct, grounded next situations and if the
roll's scope is authoritative and observable. The present marker needs no roll:
`World` can deterministically validate whether its typed package is allowed.

**Does not fit.** D&D's six abilities, skills, proficiency bonus, DC ladder, advantage,
disadvantage and d20 are a complete character-competence model. Importing them would
introduce scores and progression that Aicadia explicitly excludes. The heuristic is
“roll only meaningful uncertainty,” not “use a d20.” Random prose generation is also
not an ability check: a language model's variability cannot silently decide World
truth.

### 4. Social play joins free roleplay to bounded, situated consequences

**Observable D&D pattern.** Current D&D social interaction progresses through both
roleplaying and ability checks. The player chooses how the Character approaches the
conversation; an NPC's attitude, personality, goals, fears and sympathies shape the
reaction; a check is used only where chance should still affect the response. The
rules therefore do not reduce social play to either pure improvisation or one generic
persuasion roll.
([SRD 5.2.1, “Social Interaction,” pp. 10–11](https://media.dndbeyond.com/compendium-images/srd/5.2/SRD_CC_v5.2.1.pdf))

**Transferable heuristic.** Free expression becomes game play when it selects an
approach to a concrete other actor or situation, while authoritative resolution stays
bounded by known state and supported consequences.

**Possible Aicadia application.** The current Agent flow already separates expressive
prose from the typed `introduce_entity` consequence. If Aicadia later gains a concrete
social actor, a useful candidate test is whether the actor has independently
established facts or interests that make different approaches meaningful. Until then,
the Agent can phrase choices richly but cannot claim an unseen NPC response.

**Does not fit.** A general “influence” field, hidden NPC attitude score or Agent-made
NPC interiority is not justified. D&D's GM may portray and decide for all NPCs;
Aicadia has no accepted omnipotent author role and cannot let a player Agent privately
manufacture another durable subject's will.

### 5. Exploration rewards concrete attention to place, tools and trade-offs

**Observable D&D pattern.** D&D exploration is interaction with dangerous, mysterious
places. Equipment changes what characters can perceive or reach. Searching must be
directed near the hidden object; a high check cannot compensate for searching in the
wrong place. Travel pace trades speed against perception, survival and stealth.
([SRD 5.2.1, “Exploration” and “Travel,” pp. 11–12](https://media.dndbeyond.com/compendium-images/srd/5.2/SRD_CC_v5.2.1.pdf))

**Transferable heuristic.** Exploration becomes play when a player can form a grounded
hypothesis about a place, choose where or how to investigate, and accept a trade-off.
Purely requesting “discover something” delegates the interesting decision to the
resolver.

**Possible Aicadia application.** Exact-Place Entity and Activity reads are already a
minimal evidence surface for grounded attention. A future investigation slice could
distinguish concrete targets or approaches drawn from those reads before it needs
terrain simulation, inventory or a universal skill system. A positive result should
attach to the existing Place and history rather than appear as context-free loot.

**Does not fit.** D&D's equipment catalog, metric distances, marching order, light
levels, hazards and travel pace are interconnected content and rules. None should be
introduced as a generic exploration ontology before a selected Aicadia outcome
requires it.

### 6. Resource pressure works because expenditure, danger and recovery form one loop

**Observable D&D pattern.** Current D&D has hour-long Short Rests and eight-hour Long
Rests. Short Rests let characters spend a finite recovery resource and recharge some
features; Long Rests restore broad resources, reduce Exhaustion and cannot be started
again immediately. Interruptions can remove or delay benefits. Many class features
explicitly recover on one of these boundaries.
([SRD 5.2.1, “Short Rest” and “Long Rest,” pp. 185–187](https://media.dndbeyond.com/compendium-images/srd/5.2/SRD_CC_v5.2.1.pdf))

The older 2014 Basic Rules made the coupling especially visible: its encounter
guidance estimated six to eight medium or hard encounters in an adventuring day and
two Short Rests, and expressed daily encounter capacity in XP. That is historical
5e guidance, not a current Aicadia target or a rule to copy.
([2014 Basic Rules, “The Adventuring Day”](https://www.dndbeyond.com/sources/dnd/basic-rules-2014/building-combat-encounters))

**Transferable heuristic.** Pressure is not created by a counter alone. It needs a
closed loop:

`finite useful capacity -> informed expenditure -> accumulating constraint -> chosen
relief boundary -> restored or changed capacity`.

The cadence of challenges and the cadence of recovery must be designed together.

**Possible Aicadia application.** None is warranted now. A future concrete activity
might create a scarce opportunity, commitment or recoverable condition; if so, its
relief rule should be designed in the same slice and its history should show what
happened. This pattern can also be used negatively: if there is no interesting choice
about spending or relief, do not add a resource meter.

**Does not fit.** Hit Points, spell slots, Exhaustion levels, encounter budgets,
adventuring-day XP, daily resets and rest spam all presuppose combat attrition and a
clock. They conflict with Aicadia's current no-score boundary and could punish absence
in a persistent asynchronous World.

### 7. Encounter, adventure and campaign are nested closure scales, not one giant plot

**Observable D&D pattern.** The 2024 *Dungeon Master's Guide* explicitly separates
running social, exploration and combat encounters; creating an adventure from a
premise, player draw, planned encounters, ending and rewards; and creating a campaign
from premise, characters, conflicts, start, linked adventures and ending. An official
D&D Beyond guide summarizes the intended nesting: encounters combine into an
adventure, and two or more adventures combine into a campaign. It also distinguishes
episodic from serialized connections.
([2024 DMG contents](https://www.dndbeyond.com/sources/dnd/dmg-2024),
[official campaign-creation guide](https://www.dndbeyond.com/posts/1850-creating-your-first-campaign-using-the-2024))

**Transferable heuristic.** Long-form durability can be composed from loops with
different closure horizons:

- a **moment** ends with a resolved action and changed situation;
- a **local situation** ends when its concrete tension or question is answered;
- a **larger thread** links several local situations through remembered consequences;
- a **campaign-scale arc** earns an ending rather than merely stopping.

Each layer needs enough closure to make progress legible while leaving consequences
that can seed another layer.

**Possible Aicadia application.** Activity already supplies durable memory across
otherwise stateless calls, and a placed marker can become a later Character's input.
That supports emergent linkage without a pre-authored campaign. Candidate future
behavior can be inspected for a smallest complete local question and for which
accepted result makes later play possible.

**Does not fit.** `encounter`, `quest`, `adventure`, `campaign`, `plot`, `chapter` and
`story beat` need not become server records or statuses. Aicadia's shared World has
many Users and asynchronous histories; one privileged GM-authored campaign spine
would conflict with equal player provenance and the dumb, strict server.

### 8. Random tables constrain improvisation; they do not have to author canon

**Observable D&D pattern.** Official examples use 2024 DMG tables to select or combine
an adventure situation, dungeon quirk and patron hook while a human DM still frames
the actual adventure. In an official interview, rules designer Jeremy Crawford
describes random encounter tables as optional aids that include combat, noncombat and
story-hook results and can be rolled on **or read for inspiration**, especially when
players go somewhere unexpected. The value claimed is reduced preparation and escape
from the first stereotypical idea, not autonomous campaign authorship.
([official 2024 DMG one-shot example](https://www.dndbeyond.com/posts/1846-lets-build-a-halloween-one-shot-with-the-2024),
[official designer interview on DM tools](https://www.dndbeyond.com/posts/109-dungeon-master-tools-in-d-ds-xanathars-guide-to))

**Transferable heuristic.** A good random table is a bounded prompt surface:

- its entries share a meaningful context;
- variation changes the situation rather than only its wording;
- a human can choose, combine or reject results; and
- only a deliberately accepted result enters play.

Tables are most useful at the edge of prepared knowledge, where they create a
specific prompt without pretending to know the whole World.

**Possible Aicadia application.** The existing exactly-three-proposal workshop is
already a nonrandom form of bounded divergence followed by User selection, steering
and confirmation. Private Agent guidance or future design tools could use curated
prompt structures without writing any table result to World. If World later owns an
investigation roll, its typed outcome and retry behavior must remain authoritative;
the Agent may interpret that result but not reroll privately for a preferred story.

**Does not fit.** Server-side random lore generation, random NPCs, random biomes, a
generic encounter table or an LLM “director” would ship a hidden content ontology and
an unaccountable author. A random result cannot bypass the same actor, Place,
freshness, confirmation and history boundaries as any other mutation.

### 9. D&D rewards both future capability and campaign continuity, but its numeric ladder is not the lesson

**Observable D&D pattern.** Current Basic Rules make advancement explicit and
cumulative: characters earn XP, cross level thresholds, gain class features and
increase proficiency at defined levels. The 2024 DMG separately organizes character
advancement, adventure rewards, treasure, marks of prestige, renown and Bastions,
showing that D&D uses several reward channels to make past play affect future play.
([Basic Rules, “Level Advancement”](https://www.dndbeyond.com/sources/dnd/br-2024/creating-a-character/),
[2024 DMG contents](https://www.dndbeyond.com/sources/dnd/dmg-2024),
[official 2024 DMG update](https://www.dndbeyond.com/posts/1916-updates-in-the-dungeon-masters-guide-2024))

**Transferable heuristic.** A durable campaign needs accepted actions to matter later.
That mattering can take at least three forms:

- **changed capability:** a new action becomes possible;
- **changed situation:** the shared place or relationships now support different
  choices; or
- **changed meaning:** later participants recognize, use or respond to the recorded
  history.

The common requirement is durable consequence, not a number that always rises.

**Possible Aicadia application.** The trail marker demonstrates changed situation and
changed meaning: another Character can find the same Entity and prose at the Place.
Future slices can ask which new grounded affordance or social reliance an accepted
result creates. That is more compatible with a shared settlement game than rewarding
the originating User with an isolated private score.

**Does not fit.** XP, levels, proficiency bonuses, Challenge Rating, treasure value,
renown counters, ranks and currency directly conflict with Aicadia's no-score rule or
presuppose unaccepted institutions. “Milestone” advancement would still be a level
system even if it hid XP. Nor should the server reward prose quality; it cannot
interpret private conversation or creative merit deterministically.

## Candidate heuristics for Aicadia review

These are research-derived tests, not accepted rules:

1. **Close the smallest loop.** A capability should turn grounded current facts plus
   one conscious intent into one authoritative changed situation that supports a
   next decision.
2. **Zoom only for a decision.** Add time, turns or finer actions only where their
   granularity changes a meaningful choice or consequence.
3. **Roll only meaningful uncertainty.** Define success, failure, scope and odds
   inputs before selecting any random mechanism.
4. **Keep expression free and consequence typed.** Narrative approach may be rich;
   durable state changes remain bounded, inspectable and attributable.
5. **Make exploration targeted.** Reward attention to concrete Place facts and
   chosen trade-offs, not generic requests for novelty.
6. **Pair pressure with relief.** Never add a scarce resource without its expenditure,
   recovery and history loop in the same design.
7. **Nest closure, not schemas.** Moment, local situation and long thread can be
   editorial scales without becoming domain types.
8. **Use variation before canon.** Tables or Agent divergence may supply private
   candidate material; only selected, confirmed and validated results become World
   truth.
9. **Reward by changed future play.** Prefer new affordances, shared consequences and
   remembered reliance over points or personal progression ladders.
10. **Preserve visible authorship.** D&D's GM can invisibly fill gaps; Aicadia should
    keep User/Character action, Agent formulation and World acceptance distinct.

## What this suggests for Aicadia now

The highest-leverage lesson at Aicadia's current boundary is not to add D&D's breadth.
It is to demand that the first investigation/discovery behavior, if selected, forms
the next complete loop:

`read exact shared situation -> choose a concrete line of inquiry -> resolve only
meaningful uncertainty -> establish a bounded result -> store provenance and local
history -> expose a truthful next choice`.

That framing highlights several open questions without answering them:

- What exactly can a Character investigate at the sole entry Place?
- What does a zero result change or teach, so it is not a retry prompt?
- Which positive result is temporary context, and which later confirmed act makes a
  durable shared Entity or Place fact?
- What trade-off makes one investigation approach meaningfully different from
  another without importing skills or scores?
- What new choice becomes available to the same or another Character after the first
  accepted discovery?

Until those answers exist, D&D's rests, travel, equipment, combat rounds, classes,
NPC attitudes, XP and campaign structures are useful contrast, not backlog items.

## Uncertainties and limits

- The full 2024 *Dungeon Master's Guide* chapters are purchase-gated. This report
  relies on its official public contents and first-party explanatory articles and
  does not claim inaccessible wording or procedures.
- The encounter-adventure-campaign nesting is official design guidance, not a
  deterministic rule enforced by the D&D system.
- The six-to-eight-encounter adventuring day and two-Short-Rest cadence come from the
  2014 Basic Rules. They are included only to show that resource pressure and recovery
  were co-designed; Wizards says the updated 2024 DMG replaces the 2014 DMG rules.
- The random-table evidence includes an official designer interview about the 2017
  *Xanathar's Guide to Everything*. It establishes intended optional use—roll or read
  for inspiration—not the statistical quality or campaign effect of any table.
- D&D assumes a small synchronous party and a human GM. Aicadia is a persistent,
  asynchronous, shared World mediated by per-User Agents. Every transfer above is
  therefore an inference and is labeled as such.

## Raw primary sources

- Wizards of the Coast, [System Reference Document 5.2.1 PDF](https://media.dndbeyond.com/compendium-images/srd/5.2/SRD_CC_v5.2.1.pdf),
  especially “Rhythm of Play,” “D20 Tests,” “Social Interaction,” “Exploration,”
  “Travel,” “Combat,” “Short Rest,” and “Long Rest.”
- Wizards of the Coast, [official SRD releases](https://www.dndbeyond.com/srd).
- Wizards of the Coast / D&D Beyond, [2024 Basic Rules: Playing the Game](https://www.dndbeyond.com/sources/dnd/br-2024/playing-the-game).
- Wizards of the Coast / D&D Beyond, [2024 Basic Rules: Creating a Character](https://www.dndbeyond.com/sources/dnd/br-2024/creating-a-character/).
- Wizards of the Coast / D&D Beyond, [2024 Dungeon Master's Guide contents](https://www.dndbeyond.com/sources/dnd/dmg-2024).
- D&D Staff, [Updates in the Dungeon Master's Guide (2024)](https://www.dndbeyond.com/posts/1916-updates-in-the-dungeon-masters-guide-2024).
- D&D Beyond, [Creating Your First Campaign Using the 2024 Dungeon Master's Guide](https://www.dndbeyond.com/posts/1850-creating-your-first-campaign-using-the-2024).
- D&D Beyond, [Let's Build a Halloween One-Shot With the 2024 Dungeon Master's Guide](https://www.dndbeyond.com/posts/1846-lets-build-a-halloween-one-shot-with-the-2024).
- D&D Beyond interview with Jeremy Crawford, [Dungeon Master Tools in *Xanathar's Guide to Everything*](https://www.dndbeyond.com/posts/109-dungeon-master-tools-in-d-ds-xanathars-guide-to).
- Wizards of the Coast / D&D Beyond, [2014 Basic Rules: Building Combat Encounters](https://www.dndbeyond.com/sources/dnd/basic-rules-2014/building-combat-encounters),
  used only as explicitly historical edition evidence.
