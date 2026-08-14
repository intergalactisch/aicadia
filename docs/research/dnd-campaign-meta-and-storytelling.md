# D&D campaign meta-layers and storytelling for Aicadia

Date: 2026-08-13

Status: research implications and candidate heuristics, not accepted Aicadia
direction

## Question

Which campaign-level practices make Dungeons & Dragons stories interesting across
multiple sessions, and what can Aicadia learn from them without importing a Dungeon
Master, a hidden server-side narrator, or player-independent authorship into its
shared World?

The requested scope is the meta-layer around play rather than a catalogue of combat
mechanics: authority between DM and players, session and campaign framing, hooks
versus plots, preparation and improvisation, factions and recurring rivals,
consequences and failure, secrets and unequal information, recaps and chronicles,
downtime, spotlight and social contract, open-table play, and the way a table's
habits become part of its fiction.

## Scope and source boundary

This note uses three levels of first-party evidence:

1. the current public 2024 Basic Rules for the normative player/DM loop;
2. Wizards/D&D Beyond's own summary of the 2024 *Dungeon Master's Guide* and the
   current 2026 D&D Adventurers League guides for campaign and public-play
   guidance; and
3. D&D Beyond-owned designer/editorial guidance and official campaign examples
   where the core rules do not prescribe a storytelling method.

The third level is practice, not a rule. In particular, “fail forward,” particular
scene-building recipes, and the rival-party interpretation are useful official-site
guidance or product examples, not mandatory D&D procedure. The full 2024 *Dungeon
Master's Guide* text is not freely available as one public primary-source document,
so claims about its campaign chapter are limited to Wizards' own chapter summary.

No fan guides, actual-play opinions, Reddit posts, unofficial blogs, or Powered by
the Apocalypse “fronts” material are used. “Front” is therefore not treated as an
official D&D term or as a proposed Aicadia domain noun.

## Aicadia's current boundary

Aicadia currently has one persistent `World`, durable `User` and shared `Entity`
records, one optional current `Place` per Character, and one accepted trail-marker
action that atomically adds an Entity and canonical prose at the Character's exact
Place. Every accepted mutation appends immutable normalized `Activity`; current
state is authoritative and is not replayed from that history
([current build contract](../game/README.md)).

The current player experience deliberately splits powers that D&D gives to one DM:

- `World` assigns identity, checks typed commands and current state, and commits or
  rejects atomically. It performs no inference and invokes no Agent.
- The player's Agent is a grounded guide. It may translate and frame only typed
  World results, must keep implementation details out of player mode, and cannot
  turn its prose into an additional fact.
- The User steers privately and must explicitly confirm the complete public package
  before `submit_action`.
- Exactly three grounded proposals are a private workshop device, not three
  server-authored branches. Drafts, steering, rejection and confirmation are not
  World state.
- Accepted Activity/prose is the shared chronicle. Conversation and Agent reasoning
  are private and transient.

These boundaries are specified in the
[Agent interface](../game/agent-interface.md#agent-guidance-and-player-facing-communication)
and its
[private-workshop action flow](../game/agent-interface.md#required-private-workshop-action-flow).
Every request stands alone; there is no durable domain or transport session and no
server-side narrator. Separate Agents can nevertheless encounter the same accepted
Entity and Activity at a Place, making the World persistent and operationally
asynchronous.

The comparison must therefore preserve one distinction throughout this note:

> D&D makes one human DM the live authority who can prepare hidden truth, adjudicate
> uncertainty and narrate results. Aicadia distributes those functions among a
> strict World, a non-authoritative Agent presentation layer, a confirming User and
> an immutable public chronicle.

## Central finding: the interesting part is carried between layers

D&D's current rules describe play as a repeating exchange: the DM establishes a
scene, players decide what their characters do, and the DM determines and narrates
the result, which creates the next decision point. Players are protagonists; the DM
prepares adventures and adjudicates but is not their adversary. Everyone narrates,
but they do not have interchangeable authority
([2024 Basic Rules, “Player or DM?” and “Rhythm of Play”](https://www.dndbeyond.com/sources/dnd/br-2024/playing-the-game)).

The campaign becomes interesting across sessions when each layer carries something
forward:

| Layer | What D&D carries forward | Aicadia's current or possible analogue | What cannot transfer unchanged |
|---|---|---|---|
| Social contract | tone, boundaries, rules expectations, desired play style | global Agent contract plus private User preferences | one table can renegotiate together; a global asynchronous World cannot silently assume unanimous taste |
| Campaign frame | premise, recurring concerns, serialized or episodic links | an editorial lens derived from accepted World history | a hidden prewritten plot with an authoritative ending |
| Scene | a concrete situation and legible things to act on | grounded Agent orientation and three non-exhaustive proposals | invented facts or closed branch menus |
| Adjudication | DM interprets rules, uncertainty and fictional consequences | deterministic `World` validation and future accepted game rules | Agent taste becoming World truth |
| Private preparation | hidden locations, NPC motives, clocks, clues | presently no direct equivalent | secret server-authored fiction or autonomous simulation |
| Consequence | the scene changes and creates another decision | immutable Activity plus authoritative current state | prose-only consequences that the model pretends are state |
| Chronicle | notes, character logs, story awards and callbacks | canonical Activity/prose and read-only ledger | treating a mutable recap as a second authority |
| Table culture | repeated jokes, NPC affection, rituals and callbacks | patterns that players deliberately bring back through confirmed action | private conversational repetition becoming canon automatically |
| Open-table contract | portable characters, common rules, bounded DM variation | one provider-neutral MCP contract and shared World semantics | flattening every local situation into interchangeable episodes |

The main lesson is thus not “Aicadia needs a DM.” It is that a satisfying long-form
story repeatedly links **orientation → meaningful choice → authoritative result →
remembered consequence → new orientation**. Aicadia already has the authority and
commit boundaries for the middle of that chain. Its open storytelling question is
how Agents can expose meaningful unfinished situations and remembered consequences
without taking over authorship.

### Priority synthesis: nine strongest patterns

1. **Split the DM function; do not recreate the DM role.** Let the Agent foreground
   and phrase, the User intend and confirm, and World adjudicate and remember.
2. **Frame a situation, not a plot.** A good hook exposes a named pressure and an
   actionable object while leaving method and outcome open.
3. **Prepare causes and reusable material, not endings.** In Aicadia that means
   fresh typed reads and accepted history, never an Agent's private campaign bible.
4. **Use recurrence to make the World feel independent.** People, places, factions
   or rivals become interesting when they return changed by attributable action;
   they do not need renown or relationship scores.
5. **Make consequences create the next decision.** A meaningful result changes the
   situation. Decorative prose or identical null retries do not create continuity.
6. **Build mysteries from evidence and reveal rules.** An Agent may notice a gap but
   cannot privately possess the true answer unless World has an accepted authority
   and visibility contract for it.
7. **Keep the chronicle authoritative and the recap editorial.** Activity/prose is
   durable ground; every summary is a selective, fallible lens and must remain so.
8. **Alternate intensity and distribute attention without timers or scores.** D&D's
   downtime and spotlight solve pacing and social-allocation problems, but Aicadia
   must solve them through explicit acts and open invitations rather than offline
   progress, quotas or rankings.
9. **Make culture portable only through confirmed callbacks.** A strict semantic
   contract lets different Agents share one World, while jokes, customs and motifs
   become fiction only when players deliberately reintroduce them across accepted
   Activity.

## Findings

### 1. Authority is asymmetric, but protagonist agency is explicit

The 2024 Basic Rules give the DM authority to build adventures, describe the
environment, determine uncertain results and oversee rules. They give players
authority over what their characters attempt and what they choose to explore. The DM
is explicitly a guide rather than an adversary, and the DM is asked to ensure every
Character has a chance to act outside combat
([2024 Basic Rules](https://www.dndbeyond.com/sources/dnd/br-2024/playing-the-game)).
The current Adventurers League DM guide sharpens the service role: the DM is a
“facilitator of fun,” should help each player and Character shine, collaborates while
challenging, and may tailor minor details while retaining the adventure's general
story and prominent NPCs
([AL DM's Guide 2026.2](https://wizardsprod.a.bigcontent.io/v1/static/D%26D%20Adventurers%20League%20Dungeon%20Masters%20Guide%20v2026.2)).

This asymmetry works because the boundaries are legible at the table. A player can
declare intent without also deciding that the door was unlocked or that an NPC
agreed. The DM can determine the door and NPC response without deciding what the
Character intended.

**Aicadia implication.** The same clarity matters more than copying the role. The
Agent can own *presentation, questions and options*; the User owns *intent and final
confirmation*; World owns *acceptance and stored outcome*. A proposal should never
sound as though the Agent already knows a future result, and a rejection should not
be narratively rewritten as success. This is already aligned with the current
contract.

**Failure mode.** Calling the Agent a “DM” would collapse these powers. A plausible
model response could then be mistaken for an accepted World fact, an NPC's choice,
or an outcome that World never stored.

### 2. A campaign is a rhythm of scales, not one endlessly escalating plot

Wizards' 2024 DMG overview distinguishes episodic adventures from serialized ones
and describes the campaign chapter as guidance for linking adventures. It also
recommends an explicit start, occasional lighter break sessions, and a deliberate
ending; player investment can come from a community, a compelling antagonist, a
home base or player contributions to worldbuilding
([2024 DMG campaign overview](https://www.dndbeyond.com/posts/1850-creating-your-first-campaign-using-the-2024)).

This creates at least four temporal scales:

- the **turn**, one declaration and result;
- the **scene or encounter**, a situation with a local question;
- the **adventure or arc**, several situations with a recognizable concern; and
- the **campaign**, the remembered sequence and changing relationships among arcs.

The official overview's episodic/serialized distinction matters because continuity
does not require one master plot. Recurring people, places, questions and
consequences can link otherwise independent episodes.

**Aicadia implication.** Aicadia must not introduce durable `session` or `campaign`
objects merely to imitate these labels. Its calls are stateless and its World is
shared asynchronously. The useful transfer is presentational: an Agent can render
the next private workshop at several scales—what is true here now, what from prior
Activity remains relevant, and which larger concern this action might touch—while
the stored facts remain Entity, Place, current state and Activity.

**Failure mode.** If every contribution is framed as the next beat in a universal
plot, late or infrequent players become supporting cast in somebody else's story.
If every contribution is isolated, the World becomes a feed of polished but
weightless entries. Long-form interest needs selective recurrence, not mandatory
plot convergence.

### 3. Hooks should expose an actionable situation, not prescribe a plot

D&D's scene loop begins with the DM describing the meaningful environment, not the
final outcome. Official scene-design guidance treats an encounter as one scene and
recommends a concrete objective made from an actor or object plus an action, an
interactive location, a visible obstacle, and clear information about why the
situation matters. It explicitly preserves multiple methods: negotiation, threat,
bribery and stealth can all address the same objective
([D&D Beyond, “How to Create Engaging Scenes”](https://www.dndbeyond.com/posts/1271-how-to-create-engaging-scenes-for-your-players)).
The same guidance suggests two immediate handles followed by an open alternative,
so options reduce blank-page anxiety without claiming to be exhaustive.

The official campaign-writing guidance draws a parallel distinction. A
story-focused campaign may present an obvious task but must remain responsive to
how players solve it. A sandbox instead supplies NPCs, motivations and changing
events while the players decide what matters
([D&D Beyond, “How to Write a D&D Campaign”](https://www.dndbeyond.com/posts/1671-how-to-write-a-d-d-campaign)).

**Aicadia implication.** A grounded proposal is strongest when it names:

1. one observed person, place, thing or prior event;
2. one unresolved pressure, opportunity or question; and
3. one concrete kind of attempt the Character could make.

The current three-proposal workshop can function like “doors in the situation,” not
like a branching narrative menu. The User must remain free to combine, reject or
steer outside them, subject to the accepted action surface.

**Failure mode.** Three polished outcomes that already resolve the interesting
uncertainty are disguised plots. They move authorship from the User to the Agent and
encourage confirmation of prose rather than choice of action.

### 4. Effective preparation makes improvisation grounded

The official 2024 campaign-writing guidance advises starting with a small area and
a problem, preparing concrete NPCs, encounters and environments rather than an
exhaustive setting, watching what players actually pursue, and revising future
preparation around that new direction. Its author prepares enough motives and
setting knowledge to improvise when players leave the expected route, then records
the newly encountered people, places and things for later preparation
([campaign-writing guidance](https://www.dndbeyond.com/posts/1671-how-to-write-a-d-d-campaign)).

This is “prepare causes and materials, not a transcript.” The prepared elements are
recombinable; improvisation answers player action; notes make improvisation available
for future callbacks.

**Aicadia implication.** The Agent's equivalent of preparation is fresh grounded
reading, not invented backstory. It can identify concrete reusable material in
Entity and Activity—the named marker, who acted, where, what changed, and what is not
established—and use those facts to compose proposals. Future richer World models
could add motives or ongoing processes only through an accepted behavior decision;
the Agent cannot fill the present gap with plausible fiction.

**Failure mode.** An Agent with a richly prepared “campaign bible” but no World
authority will produce coherent hallucinations. Conversely, an Agent that merely
lists records without arranging them around a pressure or affordance will be correct
but dramatically inert.

### 5. Factions and rivals make the world feel independent through recurrence

Official D&D guidance for sandbox campaigns asks which factions are prominent,
what they want and how their plans affect the realm. The emphasis is motive and
effect, not a quest list
([campaign-writing guidance](https://www.dndbeyond.com/posts/1671-how-to-write-a-d-d-campaign)).
The current AL Player's Guide allows a Character to associate with one setting
faction while explicitly removing faction-renown rewards from current AL play. This
shows that faction identity and relationship can exist without a progress score
([AL Player's Guide 2026.4](https://wizardsprod.a.bigcontent.io/v1/static/D%26D%20Adventurers%20League%20Players%20Guide%20v2026.4)).

*Critical Role: Call of the Netherdeep*, an official D&D campaign product, provides
a more concrete recurring-rival pattern. A second adventuring party pursues the same
broad destination, grows over the campaign, and changes between friendship,
competition and enmity in response to table choices. Its co-lead designer described
the party as a clock and as evidence that the protagonists are not the only active
adventurers
([official D&D Beyond rival overview](https://www.dndbeyond.com/posts/1177-face-off-against-rival-npcs-in-call-of-the)).
This is product-specific evidence, not a core D&D subsystem.

The shared storytelling function is clear: a recurring actor carries memory. The
actor returns changed by prior contact, has an intention not reducible to the
protagonist's convenience, and makes past choices visible now.

**Aicadia implication.** The most compatible current analogue is another User's
accepted Character action or a shared Entity that multiple players deliberately
revisit. A future faction or non-player actor would require an explicit World model,
ownership rules, action authority and Activity footprint. It should not be smuggled
in as Agent-authored prose. A faction need not—and under Aicadia's “No Score
Anywhere” rule must not—become renown points.

**Failure modes.** A faction that exists only as lore is scenery. A faction represented
only by a meter becomes a scoreboard. A rival whom the Agent can move at will is a
puppet and gives the Agent authority over another subject. A recurring actor is
interesting only when its change is traceable to accepted action or an explicitly
authorized future World process.

### 6. Failure is interesting when it changes the next situation

The 2024 rules call for an ability check only when failure would be meaningful and
the outcome is uncertain and narratively interesting. Dice can surprise both table
and DM; players and DM interpret the unexpected result in the fiction
([2024 Basic Rules, D20 Tests](https://www.dndbeyond.com/sources/dnd/br-2024/playing-the-game)).
The scene rhythm then returns from narrated result to a new decision point.

D&D Beyond's source-owned “failing forward” guidance distinguishes defeat from
campaign termination. It proposes three possible continuities: the story advances
with a complication, the Character changes, or an alternative plot path opens. It
also argues that failure should follow action or inaction, affect something the
Characters care about, and remain possible to answer later
([“Failing Forward: Losing Without Ending the Campaign”](https://www.dndbeyond.com/posts/408-failing-forward-losing-without-ending-the-campaign)).
This is editorial guidance, not a rule.

**Aicadia implication.** A negative outcome should not mean “nothing was written,
try the same thing again” whenever the future accepted mechanic can safely produce a
durable result. The interesting shape is: the attempt matters, the intended result
does not fully occur, a concrete cost/revelation/changed opportunity is stored, and
the Character gets a different next decision. Exactly which failures deserve World
history is a future domain choice; rejected invalid requests remain correctly absent
from Activity under the current contract.

**Failure modes.** Pure null failure invites retry spam and makes history forget the
attempt. Guaranteed success with decorative complications makes uncertainty fake.
Punishment unrelated to the Character's choice feels arbitrary. Allowing the Agent
to invent a “failure consequence” after World rejected a call creates unauthoritative
canon.

### 7. Secrets work as delayed, evidenced information—not arbitrary withholding

D&D gives the DM private preparation and a live reveal channel. The current AL DM
guide even lists a screen that hides the DM's notes and rolls, while its facilitation
guidance tells the DM to provide hints and small victories when Characters stall on
clues
([AL DM's Guide 2026.2](https://wizardsprod.a.bigcontent.io/v1/static/D%26D%20Adventurers%20League%20Dungeon%20Masters%20Guide%20v2026.2)).
Official D&D Beyond guidance on hidden VTT information treats clues, loot, hazards
and secret doors as discoverable map elements visible to the DM before the players
find them
([“Prep Made Easy”](https://www.dndbeyond.com/posts/1856-prep-made-easy-getting-ready-for-your-online)).

The official analysis of *Strixhaven: A Curriculum of Chaos* describes recurring
story seeds—NPCs, objects, places, organizations, creatures and events—introduced
early, repeated with variation, and revealed at a pace that lets players connect
them. It warns that players can miss foreshadowing, so a crucial thread should not
depend on one delicate clue
([D&D Beyond, “What Strixhaven Teaches Us About Good Storytelling”](https://www.dndbeyond.com/posts/1207-what-strixhaven-a-curriculum-of-chaos-teaches-us)).
Again, this is an official campaign example, not universal rules text.

**Aicadia implication.** Aicadia currently has no authorized hidden-fiction store
and no DM who can privately know a true answer. Therefore a present Agent may create
*questions from absences* but not *secrets from imagined answers*. A future mystery
would need one of three explicit contracts:

- accepted World truth that exists but is not yet visible to this Character;
- accepted evidence whose full relation has not yet been established; or
- a deterministic future result that World creates only when the investigation is
  accepted.

Until one is chosen, the honest storytelling pattern is “something here is not yet
known,” grounded in observed incompleteness, not “the Agent knows but will not tell.”

**Failure modes.** A secret without a stored authority is a hallucination waiting to
be retconned. A single mandatory clue turns one failed read into a stopped story.
Global visibility of every fact destroys discovery. Hidden state with no fair trace
becomes arbitrary author power.

### 8. A chronicle preserves callbacks; a recap is only a lens

Current organized play requires a legible Character log containing adventure
identity, date, DM, level and inventory changes, downtime, significant play details
and Character changes. Story awards can have future significance and are checked
when later adventures need them
([AL Player's Guide 2026.4](https://wizardsprod.a.bigcontent.io/v1/static/D%26D%20Adventurers%20League%20Players%20Guide%20v2026.4)).
This is how selective consequences travel between tables without every table sharing
one live campaign database.

D&D Beyond's 2026 Journals announcement makes the storytelling case more directly:
captured clues, decisions, bargains, secrets, jokes and reveals support precise later
callbacks; the resulting history helps a mystery remain coherent and makes the
world feel as though it remembers
([“Never Lose the Story Again”](https://www.dndbeyond.com/posts/2175-never-lose-the-story-again-introducing-journals-on)).
Journals are a mutable play aid, not D&D canon authority.

**Aicadia implication.** Aicadia already has a stronger canonical substrate than
ordinary table notes: immutable Activity with explicit actor, Place and involved
Entity roles plus one canonical prose value. A recap should be a fresh, grounded
selection from that substrate, never a second stored truth. A useful orientation is
likely selective rather than exhaustive: one recent change, one callback involving
the current Character or Place, and one still-open affordance. This is a candidate
presentation heuristic, not accepted behavior.

**Failure modes.** Summarizing only the latest item makes the World amnesiac.
Summarizing the whole ledger every turn buries salience. Saving an Agent-generated
summary as authority allows compression errors to become facts. Treating prose as a
complete state projection invents implications that structured World state does not
support.

### 9. Downtime alternates intensity and lets Characters own continuity

D&D's campaign overview recommends occasional lighter or relaxing sessions to
prevent an ongoing difficult story from becoming exhausting
([2024 DMG campaign overview](https://www.dndbeyond.com/posts/1850-creating-your-first-campaign-using-the-2024)).
Adventurers League formalizes between-session continuity through Character-owned
downtime days and logged activities; current AL also lets Bastion orders resolve
across future sessions while keeping their use bounded by explicit rules and records
([AL Player's Guide 2026.4](https://wizardsprod.a.bigcontent.io/v1/static/D%26D%20Adventurers%20League%20Players%20Guide%20v2026.4)).

The narrative value is not merely rest. Downtime changes scale: relationships,
craft, home, recovery and preparation can matter without every contribution being a
climax. It also creates contrast, making danger feel dangerous again.

**Aicadia implication.** Real elapsed time or a gap between Agent conversations must
not silently produce World history; Aicadia has no durable session and no autonomous
server authorship. A quieter act can already be told in prose only to the extent its
structured consequence is current and valid. Richer downtime would require explicit
actions, ownership, time semantics and Activity. The transferable heuristic is
tonal: not every proposed action should escalate. Maintenance, observation,
hospitality, recovery and small acts of place-making may be more valuable connective
tissue once the World has concrete behavior for them.

**Failure modes.** Treating offline time as automatic narrative progress burns
agency and creates invisible advantage. Treating every return as crisis produces
fatigue. Calling flavor-only prose “downtime progress” overstates current state.

### 10. Spotlight is a social allocation problem, not a power reward

The current AL Player's Guide asks participants to follow the group's conduct rules,
share attention, pay attention, prepare for their turn and protect others'
enjoyment. The current DM guide asks the DM to ensure every player and Character can
shine, and prescribes pre- and post-game discussion, content warnings, listening,
decompression and ways to address disruptions
([AL Player's Guide 2026.4](https://wizardsprod.a.bigcontent.io/v1/static/D%26D%20Adventurers%20League%20Players%20Guide%20v2026.4),
[AL DM's Guide 2026.2](https://wizardsprod.a.bigcontent.io/v1/static/D%26D%20Adventurers%20League%20Dungeon%20Masters%20Guide%20v2026.2)).

Session zero is the campaign's explicit meta-contract. Official D&D Beyond guidance
uses it to align story-versus-player-driven play, lore density, genre, emotional
intensity, roleplay style, rules, PvP, boundaries and safety; it also says this
calibration can recur during a campaign
([“How to Run a Session 0,” updated 2024](https://www.dndbeyond.com/posts/929-how-to-run-a-session-0-for-your-d-d-game)).

**Aicadia implication.** A single shared asynchronous World cannot convene one
session zero whose preferences govern everyone. The permanent Agent contract can
set universal safety and authority boundaries; private User-Agent conversation can
calibrate language, intensity and desired kind of prompt; World-wide tone or content
policy would require an explicit product decision. Spotlight cannot be allocated by
a DM, points or turn quotas without changing Aicadia's model. It can instead emerge
through *addressable opportunities*: Activities that name other Characters or
shared Entities only within ownership and consent rules, invitations that leave the
other Character's response open, and Agents that deliberately surface underused
local connections. These are future considerations, not current capabilities.

**Failure modes.** Attention becomes power when frequent contributors can decide
other Characters' responses or monopolize all consequential hooks. A global tone
inferred from the loudest local history excludes quieter play styles. Private safety
preferences must not be published as World fiction.

### 11. Organized play shows how continuity survives changing tables

The current Adventurers League is official public play with a shared corpus, current
rules and portable Characters. Its DM guide allows adventures of different lengths,
one-off chapters, players who may play other adventures between sessions, and
tailoring of minor details; it still preserves the setting, general story and
prominent NPCs so players receive a recognizable adventure. Its player guide carries
significant continuity in the Character log, selected story awards, inventory and
downtime
([AL DM's Guide 2026.2](https://wizardsprod.a.bigcontent.io/v1/static/D%26D%20Adventurers%20League%20Dungeon%20Masters%20Guide%20v2026.2),
[AL Player's Guide 2026.4](https://wizardsprod.a.bigcontent.io/v1/static/D%26D%20Adventurers%20League%20Players%20Guide%20v2026.4)).

This is selective continuity: not every improvised table detail travels, but the
portable contract says which Character state and story marks do. The current AL
Adaptation Guide then provides explicit exceptions needed to make individual
published adventures compatible with that public-play contract
([AL Adaptation Guide 2026.2](https://wizardsprod.a.bigcontent.io/v1/static/D%26D%20Adventurers%20League%20Adaptation%20Guide%20v2026.2)).

**Aicadia implication.** This is D&D's closest operational analogue to provider-
neutral Agent play. Aicadia's semantic World/MCP contract, rather than DM memory,
makes a Character and accepted history legible across hosts. The useful heuristic is
to keep the portable core small and explicit while allowing natural-language
presentation to vary. Aicadia goes further than AL because every accepted shared
mutation enters one live World, rather than only selected Character awards moving
between otherwise separate tables.

**Failure modes.** Too much portability policy turns play into compliance ceremony.
Too little makes every new Agent reinterpret state differently. Preserving prose but
not structured roles creates evocative yet non-actionable history; preserving only
state without prose loses why players care.

### 12. Table culture becomes fiction through deliberate callbacks

D&D's rules say the action is narrated together, but role authority remains split
between DM and players. The 2024 DMG overview explicitly encourages asking players
questions that let them contribute to worldbuilding
([2024 DMG campaign overview](https://www.dndbeyond.com/posts/1850-creating-your-first-campaign-using-the-2024)).
The Journals announcement describes how a minor NPC, a promise, or even a joke can
gain campaign texture when remembered and called back later
([D&D Beyond Journals](https://www.dndbeyond.com/posts/2175-never-lose-the-story-again-introducing-journals-on)).

This suggests a useful distinction:

- **occurrence**: something was said or happened once at the table;
- **recognition**: the group remembered and treated it as meaningful;
- **incorporation**: a later scene made it consequential in the fiction.

Ordinary D&D tables can blur these stages because the same people remember and the
DM can ratify details conversationally. Aicadia cannot. Its equivalent stages must
respect the explicit commit boundary.

**Aicadia implication.** Private banter, rejected drafts and Agent phrasing remain
non-canonical occurrence. A User can deliberately bring a motif back in a future
confirmed package; other players can recognize it by independently referencing the
same Entity or Activity. Culture then accretes from repeated, attributable, accepted
attention. The server need not declare a meme, custom, reputation or tradition. If a
future game behavior wants one of those as queryable current state, it must define
its own actor, acceptance and history semantics.

**Failure modes.** Auto-promoting frequent words or model summaries to canon turns
statistical repetition into authorial power. Treating no repeated pattern as
meaningful until a server taxonomy exists makes culture sterile. The safe middle is
human-confirmed reuse grounded in durable history.

## Candidate storytelling heuristics for Aicadia

These are research-derived heuristics for later discussion and playtesting. They do
not alter `docs/game/`, authorize new capabilities, or settle future domain models.

### Heuristics compatible with the current authority boundary

1. **Orient before proposing.** Render the current named Place, the most relevant
   recent accepted change, and what remains observably open before suggesting an
   act.
2. **Offer doors, not endings.** Each proposal should identify a concrete object of
   attention and an attempt; it should not narrate the uncertain result as complete.
3. **Treat three as assistance, never exhaustiveness.** The three workshop options
   reduce blank-page friction. The User can combine, reject or redirect them.
4. **Use a strong noun and verb.** “Examine the cedar marker,” “ask Mara about the
   crossing,” or “reinforce the workbench” is more playable than “explore the lore”
   or “advance the story,” provided the action exists in the current game surface.
5. **Make every callback cite its ground.** A recurring name, promise or consequence
   must come from typed current state or Activity, never model memory alone.
6. **Recap selectively.** Prefer one recent change, one older relevant callback and
   one present affordance over either a full ledger dump or an amnesiac latest-item
   summary.
7. **Preserve negative space.** Say honestly what is not established. An unknown is
   an invitation to investigate later, not permission to invent its answer now.
8. **Let tone breathe.** Not every proposal needs to escalate danger or world stakes;
   the current action still has to fit its accepted concrete consequence.
9. **Keep meta clarity private and fiction grounded.** The Agent can ask the User
   above the World whether a proposal is clear or welcome, then return to in-world
   language for the preview. That conversation does not become Activity.
10. **Canon begins only at accepted confirmation.** Repetition, eloquence, model
    confidence and emotional intensity never substitute for the explicit preview,
    User confirmation and World acceptance boundary.

### Heuristics that require future product or domain choices

11. **Failure should alter the next decision.** A valid uncertain attempt should, if
    the future mechanic permits, create a durable changed situation rather than only
    an invitation to repeat the same roll.
12. **Recurring actors need independent, attributable continuity.** Factions, rivals
    or World actors need explicit identity, motives or state, action authority and
    Activity; they cannot be free-moving Agent characters.
13. **Secrets need a truth and a reveal rule.** No mystery should depend on facts
    that exist only in an Agent's hidden reasoning. Evidence, visibility and
    discovery must have a World-owned contract.
14. **Downtime must be authored, not inferred.** Offline time, conversation gaps or
    server clocks do not create Character history by themselves.
15. **Spotlight should create invitations, not scores.** A later social system can
    expose relationships and unanswered calls to action without renown, rank,
    priority or canon-weight counters.
16. **Campaign arcs should remain derived lenses.** A recurring local concern may be
    rendered as an arc from Activity and current state; storing a universal plot or
    predetermined ending would be a separate, consequential design choice.

## Failure-mode checklist

| Failure mode | Observable symptom | Boundary it threatens |
|---|---|---|
| Hidden Agent-DM | the Agent reveals motives, outcomes or off-screen changes absent from World | dumb/strict server and grounded guide |
| Three-choice railroad | every option is a complete authored outcome and free steering feels invalid | protected User agency |
| Chronicle-as-state | a recap adds ownership, relationship, placement or consequence not present in typed state | one authoritative World |
| Prose-only consequence | the story says a lasting change occurred but only a marker Entity was stored | structured consequence authority |
| Null-loop failure | repeated valid attempts leave no trace and invite identical retries | meaningful persistent history |
| Fake secret | later prose changes an answer that was never stored or deterministically generated | stable shared truth |
| Omniscient discovery | every Character sees every future-relevant fact immediately | discovery and information locality |
| Autonomous downtime | World changes because a player was absent or time passed, without an explicit action | no unconscious token burn and player agency |
| Faction meter | a scalar substitutes for concrete relationships and acts | no score anywhere |
| Rival puppet | an Agent authors another durable subject's choice to create drama | ownership and one-subject identity |
| Global plot gravity | all local contributions become support for one privileged arc | equal access to the shared World |
| Canon by repetition | frequent language or model output is promoted without confirmation | explicit public commit boundary |
| Recap flood | every return begins with an exhaustive history dump | salience and playability |
| Open-table flattening | provider neutrality removes local detail, prose or Character consequence | meaningful continuity |

## Concrete questions for later experiments

These are test questions, not a selected next build:

1. Given the same exact Place Entity and Activity pages, can a clean-room Agent
   produce a brief orientation containing one fact, one callback and three genuinely
   non-exhaustive attempts without inventing state?
2. Do players choose and steer more deliberately when proposals describe attempts
   rather than polished outcomes?
3. Can another Agent distinguish canonical Activity prose from the first Agent's
   recap language and refuse to treat the recap as World fact?
4. When a read returns an absence, can the Agent make that absence intriguing without
   implying a hidden answer?
5. Can recurring reference to one accepted Entity create a sense of culture and
   history before any faction, reputation or tradition type exists?
6. For the planned investigation edge, what is the smallest negative result that is
   both durable and meaningfully changes the next choice without becoming a score or
   authoring an unsupported World event?

## Overall implication

D&D campaigns stay interesting not because a DM has a secret plot, but because a
human table repeatedly turns remembered consequences into new, legible choices.
The DM normally supplies continuity by combining four functions: choosing what to
foreground, keeping some information back, adjudicating uncertain results, and
bringing changed people and problems back later.

Aicadia should keep those functions separated. Its most promising D&D-derived
storytelling shape is:

1. the Agent reads fresh authoritative local history;
2. it foregrounds one concrete changed or unfinished situation;
3. it offers several open attempts without claiming their outcome;
4. the User chooses, steers and confirms the exact public package;
5. World alone accepts and stores current state plus immutable Activity; and
6. later Agents earn continuity by grounding callbacks in that same accepted record.

This produces campaign texture without a server-side narrator. Factions, rivals,
secrets, downtime and richer failure could strengthen the loop later, but only after
their independent actors, state, ownership, visibility, acceptance and history are
explicitly designed. D&D shows the storytelling value of those functions; it does
not justify bypassing Aicadia's authority boundaries to obtain them.

## Raw primary and first-party sources

- Wizards of the Coast, [2024 Basic Rules: Playing the Game](https://www.dndbeyond.com/sources/dnd/br-2024/playing-the-game).
  Normative current role split, scene rhythm, D20 tests and meaningful uncertainty.
- Wizards/D&D Beyond, [Creating Your First Campaign Using the 2024 Dungeon Master's Guide](https://www.dndbeyond.com/posts/1850-creating-your-first-campaign-using-the-2024).
  First-party summary of the current DMG's campaign chapter; used where the full
  chapter is not publicly readable.
- D&D Beyond, [How to Write a D&D Campaign](https://www.dndbeyond.com/posts/1671-how-to-write-a-d-d-campaign).
  Source-owned 2024 editorial guidance on story-focused versus sandbox campaigns,
  small preparation and improvisation.
- D&D Beyond, [How to Create Engaging Scenes for Your Players](https://www.dndbeyond.com/posts/1271-how-to-create-engaging-scenes-for-your-players).
  Source-owned scene and hook guidance; not normative rules.
- D&D Beyond, [How to Run a Session 0 for Your D&D Game](https://www.dndbeyond.com/posts/929-how-to-run-a-session-0-for-your-d-d-game).
  Source-owned guidance updated in 2024, grounded in *Tasha's Cauldron of
  Everything*; used for social-contract practices.
- Wizards of the Coast, [D&D Adventurers League Player's Guide v2026.4](https://wizardsprod.a.bigcontent.io/v1/static/D%26D%20Adventurers%20League%20Players%20Guide%20v2026.4).
  Current official public-play rules, released 2026-07-02.
- Wizards of the Coast, [D&D Adventurers League DM's Guide v2026.2](https://wizardsprod.a.bigcontent.io/v1/static/D%26D%20Adventurers%20League%20Dungeon%20Masters%20Guide%20v2026.2).
  Current official facilitation, adjudication, adaptation and safety guidance,
  released 2025-11-25 and still linked as current in August 2026.
- Wizards of the Coast, [D&D Adventurers League Adaptation Guide v2026.2](https://wizardsprod.a.bigcontent.io/v1/static/D%26D%20Adventurers%20League%20Adaptation%20Guide%20v2026.2).
  Current official compatibility layer for published adventures.
- D&D Beyond, [Never Lose the Story Again: Introducing Journals on D&D Beyond](https://www.dndbeyond.com/posts/2175-never-lose-the-story-again-introducing-journals-on).
  First-party 2026 product evidence about notes, callbacks and campaign memory; not a
  canon rule.
- D&D Beyond, [Prep Made Easy: Getting Ready for Your Online Session](https://www.dndbeyond.com/posts/1856-prep-made-easy-getting-ready-for-your-online).
  First-party official-VTT guidance on hidden discoverable information.
- D&D Beyond, [What *Strixhaven: A Curriculum of Chaos* Teaches Us About Good Storytelling](https://www.dndbeyond.com/posts/1207-what-strixhaven-a-curriculum-of-chaos-teaches-us).
  Source-owned reading of an official campaign's story-seeding practice; not a core
  rule.
- D&D Beyond, [Face Off Against Rival NPCs in *Call of the Netherdeep*](https://www.dndbeyond.com/posts/1177-face-off-against-rival-npcs-in-call-of-the).
  Official product/designer evidence for one recurring-rival implementation; not a
  general D&D subsystem.
- D&D Beyond, [Failing Forward: Losing Without Ending the Campaign](https://www.dndbeyond.com/posts/408-failing-forward-losing-without-ending-the-campaign).
  Source-owned designer/editorial guidance by an official D&D adventure co-author;
  deliberately treated as practice rather than rules.

## Uncertainties and nonclaims

- This note does not claim the 2024 DMG mandates a single campaign structure. Its
  public first-party overview explicitly presents options and prompts.
- D&D has many official setting-specific faction, patron, relationship and rival
  systems. The examples here establish useful functions, not one canonical generic
  data model.
- “Fronts” is a useful term in other tabletop traditions but was not established as
  a current generic D&D 5e structure by the official sources reviewed here.
- Ordinary home-table canon has no universal D&D database or ratification protocol.
  Claims about table culture are therefore process observations grounded in
  first-party guidance about shared narration, player worldbuilding and callbacks,
  not a formal rule.
- Organized-play portability is selective Character continuity, not one mutable
  shared World. Its comparison to Aicadia is structural, not equivalence.
- No claim is made that Aicadia currently supports factions, NPC agency, secrets,
  downtime, campaign arcs, rich failure consequences or social spotlight mechanics.
  Each would require a separate accepted contract before implementation.
