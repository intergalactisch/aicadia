---
status: complete
created_at: "2026-08-13T11:42:41+02:00"
updated_at: "2026-08-13T16:52:51+02:00"
accepted_at: "2026-08-13T15:34:38+02:00"
completed_at: "2026-08-13T16:52:51+02:00"
---

# Character-grounded Entity interaction and history

## Outcome

At one exact current Place, a User's Character can perform one deliberately selected,
explicitly confirmed directed interaction involving one or more existing co-present
Entities. World accepts it as one atomic action with one accountable actor, canonical
prose, exact Place and explicit directional participation for every involved Entity.
Repeated actions form rich many-to-many Entity history without authoring another
played Character's response.

Actor, explicitly directed Entity, witness and distant Character receive only the observation and
later history justified by their own Character context. A User cannot widen that
knowledge with global queries, ids, counts or prompt pressure. Player mode never
reveals whether another Entity is User-controlled, so the rat-character scenario can
be playful without false system claims, leaked provenance or lost identity.

This is the highest-value current edge because the present trail marker proves shared
objects but not inhabited social play. The evidence claim is one complete interaction
slice that creates attributable directional history and different justified views of
the same event through one `World` interface, HTTP and MCP contract.

## Non-goals

- a generic action, interaction, conversation, observation or relationship engine;
- one atomic action with several independently confirming acting authors;
- movement, routes, distance, geometry, general visibility or Place neighborhoods;
- autonomous NPCs, factions, clocks, background inference or server-authored prose;
- disguise rules, species taxonomy, reputation, affinity, relationship scores or
  automatic friendship/rivalry;
- open-ended Property-key/Trait introduction or mutation in this first interaction slice; the
  required follow-on remains explicit rather than entering through prose;
- a complete ripple system, private messaging or remote interaction;
- private block/ignore state, notifications or attention-pressure mechanics; target
  Users retain response ownership, and safety controls are revisited before movement
  or broader reach;
- building administrative authentication or an administrative meta-Agent; absolute
  inspection is retained as a separately authorized operational concern rather than
  a player capability.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence for this build |
| --- | --- | --- |
| `docs/game/README.md` | `submit_action` has one actor, one exact Place, one introduced Entity and immutable Activity/prose | Add one distinct Interaction operation behind the same deep `World` seam rather than disguise it as an Action consequence or create another service |
| `src/world.rs`, `activity`, `activity_entity` | Activity already supports one optional actor and many involved Entity rows, but roles are only `subject`, `destination`, `location` | Cardinality can reuse the seam; concrete participation and observation meanings still need a decision |
| `docs/game/agent-interface.md` | Agent reads current Place, offers three directions and obtains exact confirmation | Preserve private workshop and one irreversible commit |
| `docs/concept/10-discovery-and-world-context.md` | Shared state is not universal knowledge; participation, observation and knowledge can be asymmetric while control provenance stays outside player mode | Scope reads and outputs by Character; never rely on narration to hide over-broad data |
| `docs/research/dnd-rules-and-campaign-loops.md` | Durable play loops turn grounded situation and intent into authoritative consequence and a new decision | The first interaction must leave a truthful response opportunity, not a completed response |
| `docs/research/dnd-campaign-meta-and-storytelling.md` | Recurring actors, callbacks and remembered consequences create campaign continuity | Interaction history must support later grounded recurrence without campaign schemas |
| `docs/research/tabletop-narrative-meta-beyond-dnd.md` | Authority, stakes, observation and recap should not impersonate each other | Keep User intent, Agent composition, World result and Character knowledge separate |
| `.agents/backlog/capability-map.md` | Current and future User capabilities need one living index | Update status and links with every accepted scope or completion |
| User choices on 2026-08-13 | Many-to-many Entity interaction, local knowledge, delayed ripples and playful rat ambiguity are core | Treat them as invariants and stress tests, not optional flavor |

## Research traceability and play-experience contract

The three tabletop reports inform one coherent play direction. They do not authorize
all of their subjects in the first build. Every finding below is classified as:

- **Now** — a contract, Agent heuristic or evidence case for this interaction and
  knowledge outcome;
- **Later** — a binding design test for a named future capability, not current scope;
- **Lens** — a useful editorial scale or review question that earns no domain model;
- **Reject** — a pattern Aicadia deliberately does not import.

### D&D rules and durable campaign loops

| Research finding | Aicadia alignment | Status | Plan consequence |
| --- | --- | --- | --- |
| Scene → declared action → authoritative result → new decision | The Agent renders Character-grounded current facts, the User selects intent, World accepts one bounded consequence and Activity makes a truthful next response possible | Now | T1 freezes the exact loop; T4 implements it; T7 proves another User can answer without a pre-authored response |
| Resolution zoom changes only when decision density changes | Keep one atomic interaction at the existing exact Place; add no rounds, action points, clock or micro-movement | Now / Lens | T1 rejects unearned time granularity; movement later earns its own decision seam |
| Dice only resolve meaningful uncertainty | The first directed interaction remains deterministic unless T1 identifies a concrete uncertain outcome with distinct meaningful branches | Now / Later | No decorative roll; investigation must define positive, zero and next choice before randomness is designed |
| Social play joins free roleplay to bounded situated consequences | Free prose and steering express the approach; typed action, actor, Place and participation bound durable meaning; no directed Entity response is inferred | Now | T1/T3/T4/T5 preserve free expression plus exact consequence |
| Exploration rewards concrete attention to Place and trade-offs | Orientation and target selection use only Character-knowable exact-Place facts; generic “show me something interesting” grants no global knowledge | Now / Later | T4 scopes reads; investigation later needs a concrete inquiry and trade-off |
| Pressure and relief form one loop | Do not add stress, cooldown, attention or interaction capacity unless expenditure, consequence, recovery and history are designed together | Later / Reject-now | Repeated-following safety cannot become an unexplained meter or timer |
| Moment, situation, adventure and campaign are nested closure scales | Activity, recurring Entities and derived lenses can show local closure and longer threads without `scene`, `quest`, `arc` or `campaign` records | Lens | Capability map and recaps preserve scales editorially; no schema task |
| Random tables constrain improvisation before canon | Exactly three private Agent proposals create bounded variation; any later oracle constrains interpretation but never writes World truth itself | Now / Later | T5 keeps proposals private/non-exhaustive; investigation uses roll-first/author-second only when selected |
| Rewards make past play affect future play | Interaction history should create response opportunities, recognition and reusable context, never XP, level, currency or prose-quality reward | Now | T7 proves changed future play through a separate reply and grounded callback |

The ten resulting D&D review tests remain explicit: close the smallest loop; zoom
only for a decision; roll only meaningful uncertainty; keep expression free and
consequence typed; make exploration targeted; pair pressure with relief; nest
closure rather than schemas; use variation before canon; reward changed future play;
and preserve visible authorship.

### D&D campaign meta-layers and storytelling

| Research finding | Aicadia alignment | Status | Plan consequence |
| --- | --- | --- | --- |
| Authority is asymmetric while protagonist agency is explicit | User owns intent and confirmation; Agent owns private framing; World owns acceptance; each User owns only their Character's response | Now | Cross-cutting invariant in T1–T7 |
| Campaign is a rhythm of scales, not one escalating plot | Recurring local concerns and Entities may be rendered as threads; no global plot or mandatory convergence | Lens | Agent recaps may connect scales without storing campaigns |
| Hooks expose an actionable situation rather than prescribe a plot | Proposals name an observed subject, an open pressure/question and a concrete attempt; they never pre-author the outcome | Now | T5 guidance and T6 proposal evidence |
| Preparation makes improvisation grounded | Agent “prep” is fresh authorized World reading and Activity callbacks, never a hidden campaign bible or invented backstory | Now | T4 makes facts available; T5 requires grounded proposal provenance |
| Factions and rivals create independence through recurrence | The first transferable primitive is stable Entity identity plus directional Activity history; independent actor authority is required before factions/NPCs | Now / Later | Interaction history supports recurrence; autonomous factions remain absent |
| Failure matters when it changes the next situation | Invalid/rejected calls still write nothing; a future valid uncertain attempt must change the next grounded choice rather than invite identical retry | Later | Investigation/discovery completion gate, not an ad-hoc interaction complication |
| Secrets are delayed evidenced information | Unknown stays honest; a mystery needs stored/derivable truth, fair evidence and a reveal rule. Control provenance is not a fake in-world secret | Now / Later | T1 decides private meta-awareness; T4 never leaks hidden truth; mystery behavior deferred |
| Chronicle preserves callbacks; recap is a lens | Activity/prose remains canonical; recaps select one recent change, one grounded older callback and one present affordance without becoming state | Now | T5/T6 verify grounded selective recap |
| Downtime alternates intensity | Proposals may be quiet, hospitable, observational or humorous; elapsed offline time never creates Character history | Now / Later | T5 tone heuristic; no background mutation |
| Spotlight is social allocation, not power reward | Interactions create invitations and response opportunities without canon weight, rank, priority or relationship scores | Now / Later | T7 checks independent reply; wider attention/ripple policy remains later |
| Organized play preserves a small portable core | One provider-neutral World/HTTP/MCP semantic contract carries identity, Activity and scoped knowledge across Agents | Now | T5 parity and clean-room evidence |
| Table culture becomes fiction through deliberate callbacks | Occurrence, recognition and incorporation remain separate; only later confirmed reuse makes a motif culturally consequential | Now / Lens | T6/T7 prove a callback uses accepted Entity/Activity rather than model memory |

### Tabletop patterns beyond D&D

| Research finding | Aicadia alignment | Status | Plan consequence |
| --- | --- | --- | --- |
| Assign final authority per question | Action, participation, observation, Character knowledge and recap each have an explicit owner; control provenance remains operational and absent from player mode | Now | T1 scenario table records owner/finality for every statement |
| State goal, approach, exposure and effect before resolution | The User sees what the Character attempts and the exact bounded World consequence; any risk must come from typed rules, not Agent judgment | Now | T1/T5 preview contract; no universal position/effect matrix |
| Fail forward where absence would stall play | Interaction creates an honest response opportunity; investigation zero must alter understanding or the next choice | Now / Later | T7 interaction evidence; future investigation gate |
| Oracle result is constrained input, not established truth | World signal, private interpretation, User confirmation and committed fact remain four stages | Later | Investigation/discovery plan dependency; never server-authored lore |
| Roll first, author second | Resolve investigation eligibility/shape before spending Agent creativity, then preview one candidate | Later | Retained investigation → discovery sequence in capability map |
| Frame clearly, then cut to the decision | Orient with who, exact Place and what changed/open; avoid exhaustive planning or a `Scene` record | Now | T5 natural presentation and T6 output checks |
| Pressure advances through causal signs, not autonomous ticks | Any future pressure names source, impulse, observable sign, accepted cause and end condition | Later / Lens | No current clock/counter; future ripple/world-pressure design test |
| Preserve canonical residue; derive recollection | Entity and Activity outlive the speaker; recap, interaction history and thematic views are Character-scoped derivations | Now | T4 read model and T6 authority tests |

### Required Agent storytelling heuristics

The following are part of the intended interaction contract and must be represented
in T5 guidance and T6/T7 evidence, while World data remains the actual enforcement
source:

1. **Orient before proposing:** render the current named Place, one relevant recent
   accepted change, one grounded older callback when useful and what remains
   observably open.
2. **Offer doors, not endings:** proposals are concrete attempts, never completed
   responses from directed Entities or resolved plots.
3. **Treat three as assistance, not exhaustiveness:** the User may combine, reject or
   freely redirect within actual World affordances.
4. **Use a strong noun and verb:** name the Character-knowable person, creature,
   thing, place or event and the intended action; avoid “advance the story.”
5. **Ground every callback:** recurring material resolves to accepted Entity,
   Activity, personal history, observation, transmission or ripple evidence—not
   Agent memory alone.
6. **Recap selectively:** never dump the World or reduce continuity to only the
   latest item.
7. **Preserve negative space:** unknown means the Character lacks a justified answer;
   it is not permission to invent a secret or query globally.
8. **Let tone breathe:** humor, ordinary attention, hospitality and small acts may be
   valuable without escalation, provided their typed consequence is honest.
9. **Keep meta clarity private and fiction grounded:** selection and safety controls
   stay outside Character narration; control provenance is not exposed in player
   mode.
10. **Canon starts at accepted confirmation:** repetition, fluency, model confidence
    and private banter never substitute for preview, confirmation and World
    acceptance.

The rat case exercises all ten: a fresh local frame makes the creature and Mara
available; proposals describe the rat's own attempts; Mara's response is left open;
control provenance stays private; humor is allowed to remain small; and later
recognition must cite accepted encounters rather than model recollection.

### Future capability alignment retained by this plan

| Future capability | Research-derived contract that must survive later planning |
| --- | --- |
| Investigation roll | Concrete Character-grounded inquiry; meaningful uncertainty; zero changes understanding/next choice; retry is not reroll; no skill/score ladder |
| Discovery commit | World signal first, Agent authors second, User confirms exact candidate, World commits typed result/provenance; oracle prompt is not fact |
| Ripple knowledge | Causal path, source provenance, bounded loss/alteration of detail and Character-specific arrival; no global feed or instantaneous omniscience |
| Entity Properties | Separate bounded-text/integer key/value state model; Agent-authored Actions may atomically affect World-eligible Entities while World protects other played Characters; Activity preserves history; no RPG attributes or global knowledge |
| Entity Traits | Separate statement-form model; development appends a linked immutable version and current lens selects latest; Traits ground Agent framing but natural language is never executable World logic |
| Recurring actors/rivals | Stable identity, independent action authority, traceable recurrence and Activity; no Agent puppet, renown meter or faction turn |
| Mysteries/secrets | Authoritative truth or explicit unresolved evidence, fair reveal rule and knowledge scope; no invented hidden answer |
| Place expansion/movement | Zoom only around meaningful route/destination/risk choices; every transition leaves history; no universal turn clock |
| Downtime/world pressure | Explicit authored cause and relief/end rule; quiet play allowed; no elapsed-time authorship, hidden tick or absence punishment |
| Culture/institutions | Occurrence → independent recognition → confirmed incorporation; no automatic promotion from repetition or global popularity score |
| Long-form arcs | Derived lenses over recurring Entity/Activity history with local closure; no campaign spine, quest status or predetermined ending |

### Explicit non-import matrix

The plan rejects the following even where their source games use them successfully:

- one omniscient Agent/GM that invents missing World facts, NPC responses, secrets or
  off-screen consequences;
- D&D abilities, skills, d20/DC ladders, classes, levels, XP, Challenge Rating,
  treasure value, rests, combat rounds and encounter budgets;
- Fate points, Blades stress/coin/rep, progress clocks, relationship points,
  reputation/renown meters, ranks, currencies and disguised milestone progression;
- one universal strong/weak/miss, position/effect, social-influence or generic action
  engine whose semantic result depends on Agent taste;
- hidden GM fronts, secret solutions without stored authority, generated lore,
  autonomous faction/downtime turns, global calendars and background inference;
- flashbacks or Microscope-style insertion that retroactively establishes earlier
  World history;
- Brindlewood-style theory success that makes unconstrained prose true without a
  deterministic domain contract;
- global turns, one synchronous scene owner, campaign/quest/chapter schemas and a
  privileged plot spine;
- global Entity/social graphs, universal Character dossiers, control labels or
  aggregate counts in player mode; and
- automatic culture, relationship or truth promotion from frequency, summaries or
  repeated language.

## Alignment

### Strategic

This outcome advances Aicadia from a shared ledger of introduced things toward an
inhabited discovery and settlement World. Users can create humor, curiosity,
recognition and recurring social texture through each other's real decisions while
the server remains dumb and strict. The next concrete risk is choosing an
interaction so generic that it becomes an unrestricted mutation language, or so
thin that it stores prose without usable future history.

### Tactical

The smallest complete slice uses two or more entered Characters at the existing
entry Place. One actor's Agent reads only authorized current context, identifies an
allowed co-present directed-Entity set, offers three grounded attempts, receives steering,
previews one complete interaction and obtains explicit confirmation. World derives
the actor and Place, validates directed-Entity existence/co-presence and a fresh Place
revision, then atomically stores one Activity, canonical prose and exact directional
participation. It never stores or narrates a response from a directed Entity.

The actor can later recall its own Interaction. Each target Character can recall the
outward behavior through its Activity `target` participation; a non-target bystander
receives nothing automatically in this slice. A later response is a new Interaction
with a new actor and Activity. Several such Interactions establish many-to-one, one-
to-many and many-to-many history without a joint-author session.

Interaction expression stays free within its closed participation result and target
cardinality is one-to-many. The material first-slice choices are resolved below.
The User explicitly accepted this plan on 2026-08-13; implementation may proceed
through its dependency-ordered task graph.

### Technical

`World` remains the single deep game-behavior seam. It owns contextual actor/Place
derivation, directed-Entity authorization, participation semantics, observation or
knowledge eligibility, freshness, idempotency and the atomic Activity transaction.
PostgreSQL stores only the minimum history/evidence earned by the selected behavior.
HTTP and MCP are thin adapters with one semantic schema and error contract. Agent
tool descriptions enforce natural, Character-grounded presentation but are defense
in depth; unauthorized data is absent before inference.

Interaction is a distinct `World` operation and player capability because existing
directed Entities, directional participation and response ownership are
not Action semantics. It does not create a parallel service or second authority:
`World` still owns both operations and Activity remembers both. No standalone
Knowledge or Observation public module is introduced. Private helper modules are
earned only if they deepen `World` without widening its interface. Tests cross the
same `World`, HTTP and MCP seams as callers.

### First-slice relational shape

No new `interaction` root table or social graph is earned. One existing Activity is
the accepted Interaction identity and immutable history envelope:

```text
activity
  id
  operation = 'submit_interaction'
  requested_by_user_id
  actor_character_entity_id       -- conceptual role: actor
  context_place_entity_id         -- conceptual role: location
  prose
  request_id + request_fingerprint
  occurred_at

activity_entity
  activity_id
  entity_id
  role = 'target' | 'location'    -- one or more targets; existing Place row
  primary key (activity_id, entity_id, role)
```

The request carries one fresh `request_id`, the unchanged current Place revision,
bounded canonical prose and 1–100 distinct target Entity ids chosen from the
authorized exact-current-Place read. It carries no User, actor or Place id. World
derives those, locks the User and Place through the existing ordering, validates all
targets with one set query and writes Activity plus target/location rows atomically.
The eligible set is the current Place Entity itself, Characters currently entered
there and ordinary Entities explicitly located there, excluding the actor. A target
id that is absent, duplicated, the actor itself or no longer co-present produces one
neutral rejection and no history; the result does not reveal whether an unseen id
exists elsewhere.

The upper bound matches the existing maximum contextual page size. It is request
protection, not a fictional score or population rule. One `INSERT ... SELECT`/array
insert writes the targets. Existing actor chronology and
`activity_entity(entity_id, activity_id)` indexes serve actor and target history;
the existing `entity_location(place_entity_id, entity_id)` index serves ordinary
Entity eligibility. Characters need one matching partial index:

```sql
CREATE INDEX character_current_place_entity_id_entity_id_index
    ON character (current_place_entity_id, entity_id)
    WHERE current_place_entity_id IS NOT NULL;
```

No pairwise relationship row is updated, so write cost follows this Interaction's
named targets rather than accumulated World history.

The Property follow-on uses a different normalized relation because it is current
Entity state, not participation: reusable immutable `property_key` rows define key
meaning/type; `entity_property(entity_id, property_key_id)` owns the current value;
and `entity_property_history(entity_id, property_key_id, activity_id)` retains each
accepted change. Agent proposals and deterministic Action/Interaction consequences
both become accepted Activity before those two Property writes; there is no
polymorphic “changed by agent or event” source column. Full schema rationale and
typed checks live in `docs/concept/11-entity-traits-and-change.md`.

### Database change inventory

The accepted Interaction slice and the retained Entity-state follow-ons must not be
mistaken for one migration:

| Delivery boundary | Table | Database change |
| --- | --- | --- |
| Interaction slice | `activity` | No column change. Extend the operation check with `submit_interaction`; generalize the existing prose/request provenance check so both confirmed Action and Interaction operations require prose, request id and fingerprint. |
| Interaction slice | `activity_entity` | No column change. Extend the role check with `target`; retain the existing `(activity_id, entity_id, role)` primary key and reverse Entity/activity index. |
| Interaction slice | `character` | No column change. Add the partial `(current_place_entity_id, entity_id)` index required to find entered Characters at one Place without a table scan. |
| Interaction slice | `place`, `entity_location`, `entity`, `user` | No schema change. Reuse Place revision, ordinary-Entity placement and stable Entity/User identity. |
| Property follow-on | `property_key` | New compact shared typed-key vocabulary table. |
| Property follow-on | `entity_property_history` | New append-only typed history per Entity/key/Activity. |
| Property follow-on | `entity_property` | New direct current typed value per Entity/key. |
| Trait follow-on | `entity_trait`, `entity_trait_current` | Proposed new append-only Trait-version and current-lens tables; exact first Trait migration still requires its own accepted plan. |
| Trait retirement, only when selected | `entity_trait_retirement` | Possible later append-only retirement evidence; absent unless retirement ships in that concrete Trait slice. |

The Interaction migration therefore creates **zero tables**, changes checks on two
existing tables and adds one index to `character`. The three Property tables and the
Trait tables are ordered follow-ons, not hidden scope in T2–T7. There is deliberately
no `interaction`, `observation`, `knowledge`, `relationship`, `signal`, Property-per-
key or universal event table.

## Decisions, scenarios and assumptions

### Confirmed decisions

- Entity interaction history is many-to-many across Activity, while the first
  Activity retains one accountable actor — supports one-to-many and many-to-one
  without multi-author confirmation machinery — recorded in the concept record and
  backlog item.
- The first Interaction accepts one or more explicit, distinct, co-present Entity
  directed Entities of any Entity role — this is the powerful Terry boundary; implicit
  witnesses, joint actors, movement and state mutation do not hitchhike with it.
- The accepted first roles are `actor`, `target` and `location`; `counterpart` and
  `actee` are rejected. A target Character can know the outward behavior without
  thereby consenting, understanding or responding; non-target bystanders receive
  nothing automatically.
- A later Interaction may carry independently validated typed World consequences in
  its same atomic Activity. The first slice remains participation-only so it does not
  import Property, relation or area-effect mechanics.
- Participation is directional and distinct from observation, knowledge,
  relationship and recap — preserves asymmetric truth and avoids a generic
  `active/passive` flag.
- Another played Character's response, belief, feeling and intent are never authored
  by the acting User — preserves protected volition.
- Action and Interaction are separate game capabilities — an Action establishes or
  changes World state, while an Interaction is an act from one existing Entity
  toward one or more other existing Entities. They share `World` authority and
  Activity history, not one operation or consequence type.
- `Signal` is not a system or operation kind — speech, sound, gesture and circling
  feet are freely worded Interaction expressions.
- Property and Trait are separate future state models and may use separate tables
  and interfaces; both preserve Activity provenance. Traits ground Agent framing but
  never become executable World logic from their natural-language statement alone.
- Shared persistence does not grant global player queryability — World capabilities
  enforce Character-grounded knowledge before Agent inference.
- Character-grounded interaction and knowledge is the selected next edge before
  investigation and discovery — inhabited shared play now has higher leverage than
  another solo resolution mechanic.
- Player mode never discloses whether another Entity is controlled by a User — Users
  interact with Entities, not control categories; a future reveal would require a
  separate explicitly accepted product decision.
- Global Entity reads are removed or narrowed in player mode but retained for a
  separately authorized administrative/operator view — shared persistence remains
  inspectable without granting omniscience to a Character.
- Private attention controls are deferred from this first slice — targets own every
  response and no background Agent or notification is triggered, but repeated
  targeting can enter history and must be addressed before movement or reach expands.
- One stable Entity identity survives playful appearance ambiguity — no duplicate
  rat/NPC identity or hidden role id.
- `World` remains the public game seam and Activity remains immutable history — no
  parallel interaction service or event-sourcing model.
- The complete research alignment above governs scope and evidence — transferable
  tabletop functions are preserved while their scores, omniscient roles, content
  ontologies and synchronous assumptions are not imported.

### Scenario decision table

| Scenario | World decision and stored result | Character-grounded result |
| --- | --- | --- |
| Pip the rat circles Mara's feet at their exact Place | Accept one `submit_interaction`: Pip is derived actor, Mara is one target, current Place is location and outward behavior is canonical prose | Pip and Mara may later receive that outward behavior; neither receives a User/NPC label or the other's thoughts |
| Pip acts toward Mara and a food bowl | Accept one Activity with two distinct target rows; the bowl remains the same Entity and gains no invented mind | Mara may know the behavior; the bowl's involvement is history, not fictional knowledge or a response |
| Mara and Eno separately approach Pip | Accept two Activities with their own actors and confirmations | Together they form many-to-one history; no joint author, consensus or merged intent is inferred |
| Mara responds to Pip later | Accept a new Activity with Mara as actor and Pip as target | The earlier Interaction is not edited and no response was pre-authored for Mara |
| A third Character is co-present but not targeted | Store no witness/observer role in the first slice | That Character receives no automatic event history merely from co-presence; a later sensory behavior may add Observation evidence |
| A distant roof collapses | The remote Activity, if accepted by a future mechanic, is not selected by this Character's player reads | The Character cannot ask through prompt pressure; only a later accepted local carrier/sign could make a bounded ripple knowable |
| The User asks for all Entities, buildings, Characters or totals | No player capability supplies a global query or aggregate | Agent orients to available local facts and honest unknowns without exposing database, permissions or hidden counts; an authorized operator remains out-of-world |
| Mara moves or context changes between preview and commit | Place revision/co-presence validation rejects the whole request | Agent re-orients naturally and seeks fresh confirmation; no partial Activity or targets exist |
| Agent submits an unseen, remote or fabricated target id | One neutral invalid-target result; no existence distinction | User learns only that the attempted Interaction is not grounded in the present situation |
| Pip repeats the valid Interaction many times | Each separately confirmed valid call may leave its own Activity; nothing triggers Mara's Agent or a notification | Mara owns every response; missing attention control is explicit and must be solved before movement, notifications or broader reach |
| Agent proposes `hair_colour = red` | Not accepted through the first Interaction result | A later Property Action may atomically append Property history and update Pip's current Entity-owned value |
| A future bounded blast changes many eligible Entities | Not inferred from prose and not part of this slice | A later typed mechanic derives the complete bounded set, writes one Activity and bulk Property consequences, or rejects atomically |

### Reversible assumptions

- The first evidence scenario uses Characters already co-present at the sole entry
  Place — exercises interaction without prematurely designing movement.
- Existing request UUID and exact-Place revision patterns are reused if the selected
  action has the same retry and freshness needs — verify during interface design.
- Interaction history pages reuse `(occurred_at, activity_id)` ordering and cursor
  semantics — valid unless the selected knowledge rule requires a distinct time of
  observation or receipt.
- The existing read-only ledger can remain an operator view while player reads are
  narrowed; a future administrative meta-Agent needs a separately authorized
  operational interface rather than player MCP reuse.
- The existing exact-current-Place Entity read is deepened to return all eligible
  ordinary Entity and Character targets without role/control metadata; exact Place
  is first-slice eligibility, not universal visibility.
- Target Character recall derives from immutable Activity `target` participation; no
  Observation table is added until a witness or sensory behavior needs it.

### Closed boundary and retained later decisions

The separate Property/Traits frontier is now sufficiently aligned for this plan: a
Property belongs to one Entity and has natural identity `(entity_id,
property_key_id)`. Many Entity Properties may reference one reusable typed Property
key while retaining independent values. A new key may be introduced atomically with
its first Property. The direct current row and immutable Activity-backed history are
separate for fast reads; Trait keeps its distinct append-only lineage/current lens.
Exact Property/Trait builds remain follow-on plans and do not enter this Interaction
build.

Administrative and ripple principles are no longer blockers: a meta-Agent is a
separately authorized out-of-world operator, and a distant fact becomes Character-
knowable only when a new accepted causal carrier reaches that Character's context.
The Character receives the carried sign or report, not direct access to its source
event. Detailed operator delivery and ripple mechanics remain later plans.

Structured Entity Properties and Traits are also retained as separate required
follow-ons in `docs/concept/11-entity-traits-and-change.md`. Their current relational
direction, authority and ordering are resolved far enough to bound this build; their
first concrete mutation operations still require follow-on accepted plans. The order
is Interaction/knowledge, then Properties, then Traits, then investigation/discovery.

No material first-slice design question remains open. The User confirmed shared
understanding and explicitly accepted the plan on 2026-08-13, so it is `active`.

## Implementation map

| Surface | Current state | Intended change | Invariants |
| --- | --- | --- | --- |
| `docs/concept/10-discovery-and-world-context.md`, `CONTEXT.md`, concept log | Knowledge, free-expression Interaction and roles explored and resolved | Preserve exact first-slice language and supersede stale alternatives | Concepts remain distinct; `docs/game/` remains current truth until build |
| `.agents/backlog/README.md`, capability map, interaction item | Proposed current edge and full horizon | Keep one current status and exact evidence links | Backlog never becomes game authority |
| `docs/game/README.md`, `docs/game/agent-interface.md` | Thirteen capabilities, global Entity reads, marker-only action | Specify the accepted directed interaction and Character-scoped reads; narrow/reclassify global reads | Agent capability parity; natural player presentation; current-only contract |
| `migration/` | One actor Activity; many Entity roles limited to subject/destination/location; ordinary Entity Place index but no Character Place index; no observation/knowledge state | Extend Activity operation/provenance and target role checks; add the partial Character-at-Place index; create no Interaction table | Append-only history; one subject identity; FK integrity; no scores or generic payload |
| `src/world.rs` | One deep World seam; contextual exact-Place reads and marker commit | Add contextual directed-Entity selection/read, interaction acceptance and scoped history/knowledge | World derives User/Character/Place; atomicity; idempotency; freshness; no LLM |
| `src/wire.rs` | Current request/results expose global and exact-Place data | Encode one strict semantic contract with no User/control leakage | Deny unknown fields; no caller-supplied User/actor/Place |
| `src/server.rs`, `src/agent_contract.rs`, `src/agent-play-contract.txt` | HTTP/MCP parity and private workshop for marker action | Ship matching interaction/read tools and grounded natural heuristics | Thin adapters; provider-neutral; unauthorized facts absent before model |
| `tests/world.rs`, `tests/server.rs`, catalog/OpenAPI fixtures | Marker, exact-Place, retry/freshness and observer evidence | Add cardinality, asymmetric view, rollback, privacy and parity scenarios | Existing evidence retained; no test-only authority |
| local playtest/ledger evidence | One marker observed by second Agent | Demonstrate rat/target/witness/distant views under selected policy | No paid model run without separate authorization; ledger is not player authority |

## Execution contract

Root owns outcome, scope, plan state, integration and the final evidence claim. A
delegated Agent receives this plan path and one dependency-ready task id, re-reads
the live repository, changes only its owned surfaces, runs focused evidence and
returns raw results. Delegation is optional. Run tasks in parallel only when the
table marks them safe, write surfaces do not overlap and results verify
independently.

## Task graph

Allowed states are `pending`, `in_progress`, `completed` and `blocked`.

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | completed | — | no | Freeze the resolved first Interaction, target knowledge and explicit deferred-safety boundary; obtain plan acceptance | concept record, glossary, concept log, plan, backlog item/map | Concrete rat, one-to-many, many-to-one, witness and distant scenarios have one unambiguous result each |
| T2 | completed | T1 | no | Publish the accepted current game contract and capability correction before implementation | `docs/game/`, backlog status/map | Twelve-capability accepted player target, loopback operator-read reclassification and explicit implementation-pending boundary agree; relative links resolve and `git diff --check` passes |
| T3 | completed | T2 | no | Implement the minimum immutable participation persistence required by the accepted contract | `migration/`, storage model in `src/world.rs`, database tests | Migration 0006 and 44 passing World tests prove roles/provenance/context, one-to-many cardinality, duplicate rejection, immutability, exact index shape and transaction rollback |
| T4 | completed | T3 | no | Implement the deep `World` interaction and Character-grounded read behavior | `src/world.rs`, domain types, World tests | Seven focused Interaction tests plus all 48 World tests prove contextual derivation, knowledge scope, history ordering, retry, freshness, concurrency and rollback; formatter, Clippy with warnings denied and diff check pass |
| T5 | completed | T4 | yes | Ship strict HTTP and MCP parity plus natural Agent guidance | `src/server.rs`, `src/wire.rs`, `src/agent_contract.rs`, player contract, catalog/OpenAPI fixtures | Eleven server tests plus all 66 Rust tests prove HTTP/MCP Interaction success/error parity, strict schemas, safe scoped output, exact twelve-tool catalog, removed global MCP calls and preserved loopback ledger reads; formatter, strict Clippy and diff checks pass |
| T6 | completed | T4 | yes | Prove adversarial and multi-Character outcome scenarios without global leakage | test helpers, `tests/world.rs`, `tests/server.rs` outside T5 fixtures | Seven focused World Interaction tests, three focused server tests and all 66 Rust tests prove safe rat context, asymmetric views, cardinality/reply, neutral zero-write rejection, retry identity, global-tool absence and the explicitly deferred repeated-targeting boundary; formatter, strict Clippy, exact catalog and diff checks pass |
| T7 | completed | T5,T6 | no | Align current docs, capability map, backlog and exact bounded playtest evidence | `docs/game/`, `.agents/backlog/`, concept log, plan, approved harness surfaces | Full validation ladder and the deterministic rat outcome support exactly the claim; no paid model run was used |

## Task details

### T1 — Freeze the domain and player contract

**Objective:** Every material action, identity, target-knowledge and deferred-safety choice has one
concrete accepted answer before code or schema changes.

**Actions:**

1. Verify the resolved Property-key model, then verify the rat, group-
   target, shared-object, witness, closed-door, later-reply, distant-event and global-
   count scenarios against all confirmed answers.
2. Use the conventional `submit_interaction` operation and choose the shortest
   sufficient participation vocabulary.
3. Verify the minimum observable/remembered facts and prove that control provenance
   is absent from player results.
4. Confirm each research-matrix row remains `Now`, `Later`, `Lens` or `Reject` after
   the concrete interaction decisions.
5. Revise this plan, governing concept record, glossary, backlog item and capability
   map; request explicit plan acceptance.

**Invariants:**

- No generic interaction engine, authored target response or global knowledge.
- Many-to-many history remains possible without forcing multi-actor commit.
- Player creativity and the exact current safety boundary are both expressed as
  implementable allowed/rejected examples; deferred controls are not claimed present.

**Evidence:**

- Scenario decision table in the revised plan — every actor, input, state change,
  observation and rejection is explicit.

**Stop conditions:**

- Stop while any scenario changes the public contract, identity, privacy, safety or
  evidence claim.

### T2 — Publish the accepted contract

**Objective:** Current authorities describe the exact accepted capability and no
stale global-player claim remains.

**Actions:**

1. Update `docs/game/` with the new operation/read contract, errors, Activity roles,
   privacy boundary and explicit deferrals.
2. Update backlog and capability map from Proposed to Ready only when the executable
   contract and plan are accepted.

**Invariants:**

- Documentation does not imply implementation completion.
- Operator/public-ledger scope remains separate from player capability.

**Evidence:**

- Focused authority diff — every new claim maps to an accepted plan decision.

**Stop conditions:**

- Stop if the documented contract requires an unplanned schema, actor or operation.

### T3 — Store the minimum durable history

**Objective:** PostgreSQL can atomically retain exact directional participation and
only the observation/receipt evidence the accepted behavior requires.

**Actions:**

1. Extend or replace existing Activity roles only as the accepted Interaction
   participation requires.
2. Add immutable Character-specific evidence only if T1 proves it cannot be derived.
3. Add FK, uniqueness, check, ordering and immutable-history constraints for current
   reads; add no speculative indexes.

**Invariants:**

- Activity remains immutable and current state is not rebuilt by replay.
- One Entity keeps one id; one Activity has one accountable actor in this slice.
- Partial action, participation or observation writes cannot commit.

**Evidence:**

- Focused migration/database tests — one/many participant rows, duplicate roles,
  invalid target, rollback and update/delete rejection.

**Stop conditions:**

- Stop if the schema begins modeling universal relationships, beliefs, conversations
  or future action types.

### T4 — Deepen World with interaction and scoped reads

**Objective:** Callers exercise all contextual validation, acceptance and knowledge
rules through a small `World` interface.

**Actions:**

1. Add the accepted command and minimum Character-scoped read(s).
2. Derive actor, current Place and eligible directed Entities from User context; validate
   freshness and retry identity.
3. Return only the accepted result and Character-authorized facts/history.

**Invariants:**

- Agent input supplies no User, actor or Place id.
- No global query occurs behind a presentation filter.
- Another Character's response or personal state is never returned.

**Evidence:**

- World tests — rat action, reverse reply, one-to-many, many-to-one,
  witness/distant asymmetry, stale Place and exact retry.

**Stop conditions:**

- Stop if one interface must expose storage roles, control provenance or programmable
  queries to function.

### T5 — Ship adapter parity and natural guidance

**Objective:** HTTP and MCP expose identical strict capabilities, while the Agent
describes only observable in-world facts.

**Actions:**

1. Add matching wire schemas, endpoints/tools, annotations and error mappings.
2. Update the one global player contract and tool descriptions for private proposals,
   complete preview, confirmation and asymmetric rendering.
3. Encode the ten required Agent storytelling heuristics without treating prompt
   compliance as authorization or semantic validation.
4. Remove or reclassify global Entity tools exactly as T1 decided.

**Invariants:**

- Tool descriptions are defense in depth, not the privacy mechanism.
- User-control provenance, ids and record categories remain absent from in-world
  narration and ordinary player World reads. Separately authorized administrative
  inspection is an operational surface, not a private player meta signal.

**Evidence:**

- Catalog/OpenAPI parity and raw adapter tests — exact schemas, errors, cache metadata
  and unauthorized-data absence.

**Stop conditions:**

- Stop if one adapter needs different game semantics or leaks a superset of data.

### T6 — Prove adversarial and social scenarios

**Objective:** The accepted capability remains correct under cardinality, concurrent
change and prompt pressure while the repeated-targeting boundary remains explicit.

**Actions:**

1. Test one-to-many/many-to-one boundaries, reverse actions and ordering.
2. Test target, witness, distant and unknown views plus global count/id probes.
3. Test proposals as grounded non-exhaustive attempts, selective recap/callback
   provenance, honest unknowns and no pre-authored target response.
4. Verify Interaction triggers no background Agent work or notifications and document
   that repeated accepted targeting remains possible until later attention control.

**Invariants:**

- No test grants a Character omniscient setup knowledge.
- No test or prose claims a block/ignore mechanism exists in the first slice.

**Evidence:**

- `DATABASE_URL='postgres://localhost/postgres' cargo test --test world interaction_`
  — 7 passed; proves safe rat context, actor/target/distant/bystander views,
  one-to-many/many-to-one/reply ordering, no target-state mutation, repeated
  targeting as a deferred boundary, every rejection class, retry/conflict,
  concurrency and atomic rollback.
- Three focused `tests/server.rs` runs — catalog, invalid MCP framing and Interaction
  adapter parity each passed; proves safe typed output, exact twelve-tool catalog,
  direct global/count probe rejection, operator HTTP exclusion from MCP authority and
  the structural Agent guidance contract. This does not claim stochastic model
  obedience.
- `DATABASE_URL='postgres://localhost/postgres' cargo test` — all 66 Rust tests
  passed: 5 library, 2 playtest-database, 11 server and 48 World tests.
- `cargo fmt --all -- --check`,
  `cargo clippy --all-targets --all-features -- -D warnings`, exact catalog fixture
  equality inside the catalog test and `git diff --check` passed. No production code,
  paid model invocation, background call or notification was introduced.

**Stop conditions:**

- Stop if expected output requires Agent semantic interpretation rather than typed
  World evidence.

### T7 — Demonstrate and align the outcome

**Objective:** All authorities and evidence support exactly one completed interaction
and knowledge claim.

**Actions:**

1. Run focused, contract and full integrity validation.
2. Demonstrate the rat scenario through deterministic Agents/fakes first; request
   separate authorization before any paid live Agent run.
3. Audit every research-matrix row against the delivered capability or retained
   future/non-import status.
4. Update `docs/game/`, capability map, backlog item, concept log and plan status to
   final truth.

**Invariants:**

- Do not promote partial candidates or narrative plausibility to evidence.
- Keep the read-only ledger and operator facts distinct from player knowledge.

**Evidence:**

- One continuous result proves actor, participants, Place, Activity/prose, scoped
  views, reverse reply independence and zero leaked global/control facts.
- `DATABASE_URL=postgres://localhost/postgres cargo test` — 66 passed: 5 library,
  2 playtest-database, 11 server and 48 World tests. The named rat World scenario
  proves Pip's three-target Interaction, Mara-only target recall, Eno bystander and
  distant-Lysa exclusion, repeat boundary, separately authored Mara reply and
  many-to-one Eno action; the adapter scenario proves safe Pip/Mara data and
  HTTP/MCP canonical parity.
- `tests/agent-playtest.sh` — passed with fake Codex/curl/cargo only after aligning
  its catalog/OpenAPI fixture to the exact current twelve player tools; it invokes no
  paid model or real database.
- `cargo fmt --all -- --check`, strict all-target/all-feature Clippy, Bash syntax,
  exact catalog assertions, all relative Markdown links and `git diff --check`
  passed.
- `DATABASE_URL=postgres://localhost/postgres tools/agent-playtest preflight` passed
  without `codex exec` after its explicit command, resolved-system-path and version
  pin were updated to the inspected current `codex-cli 0.147.0`; the real disposable
  database ownership create/tag/read/drop probe passed.
- `PATH=/opt/homebrew/Cellar/postgresql@17/17.8/bin:$PATH
  DATABASE_URL=postgres://localhost/postgres tests/aicadia-local.sh` passed using the
  installed versioned PostgreSQL clients. It proved one stable User across restart,
  every fail-closed launcher boundary, isolated Agent handoff and
  `codex_invoked=false`; follow-up inspection found zero remaining test database.
  The harness now explicitly removes inherited `AICADIA_USER_ID` so its missing-
  context case is deterministic. No paid run was attempted.

**Stop conditions:**

- Stop if any required authority, parity check, cleanup or evidence layer fails.

## Validation ladder

1. **Focused:** migration and `World` tests for directional cardinality, exact
   observation/knowledge scopes, retry/freshness, immutability and rollback.
2. **Contract:** HTTP/MCP parity, strict schemas, errors, complete catalog and
   absence of global/control facts in player results.
3. **Outcome:** two or more same-Place Characters prove the rat interaction, a
   separately authored response, asymmetric knowledge and no distant/global leak;
   include the accepted one-to-many boundary.
4. **Integrity:** formatter, strict Clippy, complete Rust and local lifecycle suites,
   `git diff --check`, focused diff review and confirmation that unrelated User
   changes and all governing authorities remain intact.

## Change control

Refine paths, task order and stronger evidence in place while the accepted outcome
and contract remain unchanged. Stop implementation, keep or return `status: draft`,
revise and request explicit re-acceptance when new evidence changes the outcome,
public behavior, domain meaning, non-goals, irreversible state, privacy/safety,
external authority, material cost or evidence claim.

## Completion conditions

- every required task is `completed` and the validation ladder passes;
- the exact strategic outcome and evidence claim are demonstrated;
- current behavior, concept choices, vocabulary, capability map and backlog agree;
- no known-stale authority, material open question or accidental unrelated change
  remains;
- `status: complete` and `completed_at` are recorded only after these conditions.

All seven tasks and the complete validation ladder passed. The delivered contract,
implementation, current authorities and Done backlog state agree. Independent
review found two presentation-only issues; their focused re-review confirmed both
resolved with no regression. No required work remains in this accepted Interaction
slice, and its Property and Trait follow-ons remain separate unstarted work.
