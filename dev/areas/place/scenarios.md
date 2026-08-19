# Spatial scenario catalogue

> **Role / side:** Place Area scenario catalogue / development side.
> **Authority:** owns the fixed spatial narratives, known facts, unknowns and questions reused to pressure Place and related Area decisions.
> **Excludes:** accepted game behavior, expected outcomes, Movement and World Change decisions, the complete remote-button fixture and evidence verdicts; those remain in `game/docs/`, the [Movement Area](../movement/README.md), the [World Change Area](../world-change/README.md), [Multiplayer scenario S03](../multiplayer/scenarios.md#s03--a-button-detonates-a-remote-bomb) and `dev/lab/`.

Status: **active catalogue**. A scenario makes one difficult scene repeatable. It
does not authorize the scene as current gameplay, select a term or imply that one
kind of state must solve every question.

## How to use the catalogue

Work scenario-first. A candidate spatial model receives a temporary label such as
`Candidate A`; it does not enter canonical vocabulary merely because it appears in
an evaluation. For every selected scenario, record:

1. what the Agent can observe and submit;
2. which exact current facts World would store and validate;
3. what another eligible Agent can retrieve without learning hidden facts;
4. what changes when each participating Entity moves or changes;
5. the smallest mutation and Activity footprint;
6. the bounded reads, indexes and concurrency dependencies;
7. the deliberately hot-subject behavior at million-Character scale; and
8. which question the candidate cannot answer without another concept.

`Relation` is used below only as the working label from the current Place
exploration for open Agent-authored meaning between World subjects. The catalogue
does not accept a Relation table, endpoint shape, predicate field or mechanical
effect.

### Cross-scenario probes

Apply these unless a scenario explicitly makes one irrelevant:

- move each named Entity independently and state which other truth becomes stale;
- replace one exact observation with an Agent-authored description and keep the two distinguishable;
- hide one involved Entity or spatial fact from another Character;
- submit two conflicting changes and one unrelated change concurrently;
- retry after an accepted response is lost and read authoritative current state and Activity;
- make one subject extremely hot while quiet subjects continue ordinary work;
- retrieve one addressed Entity and one bounded nearby set without scanning the World;
- omit geometry, a coordinate or a named reference rather than allowing World to infer it from prose; and
- reject cycles, unbounded chains or fan-out when the candidate introduces them.

## Scenario index

| ID | Narrative | Primary pressure |
| --- | --- | --- |
| SP01 | A Character moves from Place A to Place B | Place, Position, Connection and Movement |
| SP02 | An Entity is in somebody's coat | Current arrangement, movement, knowledge and access |
| SP03 | A distant button activates a bomb | Spatial distance versus remote causality |
| SP04 | A cup is two centimetres above a table surface | Exact point, relative description and persistent movement |
| SP05 | A dog sleeps under a bridge | Authored meaning versus geometry-derived current truth |
| SP06 | A Character is one hundred metres from a hotel | Distance, destination and travel progress |
| SP07 | A birdhouse is placed on a table in a village | Nested spatial grounding without universal containment |
| SP08 | A Character crosses a forest edge into open heath | Terrain, Area, unnamed Position and incomplete boundaries |
| SP09 | A cabin, passengers and cargo travel on a ship | Moving Place, relative state and hot-carrier scale |
| SP10 | A remembered sword is now hidden in a backpack | Current knowledge, memory, privacy and mutation grounding |
| SP11 | Place A connects to B but not C | Direct topology, physical infrastructure and Route deferral |
| SP12 | Many Characters share one unnamed Position | Co-location, bounded reads and hot-point contention |
| SP13 | A city, forest and waterfall appear on the map | Entity, Place and Area qualification |

## SP01 — A Character moves from Place A to Place B

**Narrative.** Character M is at established Place A. Place B is known and may have
been discovered earlier without being entered. M's Agent now proposes that M travels
to and enters B.

**Known.** A, B and M retain stable identities. Discovery did not already move M.
An accepted move must leave one current Position and attributable Activity.

**Unknown.** Whether A and B require a direct Connection; whether M occupies
intermediate Positions; whether travel takes time; what terrain lies between them;
and whether another Character may meet M underway.

**Questions.** What exact input establishes a valid destination? Which state changes
once? Can M stop between A and B without creating a Place? What does a concurrent
move observe, conflict with or retry against?

## SP02 — An Entity is in somebody's coat

**Narrative.** Character Mara wears Coat C. Entity K is described as being in C.
Mara walks away while another Character can see Mara and the coat but has never seen K.

**Known.** Mara, C and K are distinct Entities. Seeing Mara or C need not reveal K.
No generic containment mechanic is assumed.

**Unknown.** Whether K has an exact Position; whether it moves with C or Mara;
whether wearing is stored; whether “in the coat” is authored meaning or structural
state; and who may inspect, remove or change K.

**Questions.** Which minimum facts preserve K's arrangement when Mara moves? Can an
eligible Agent retrieve Mara and C without learning that K exists? What changes when
C is removed, handed over, destroyed or left behind?

## SP03 — A distant button activates a bomb

**Narrative.** The complete shared fixture remains Multiplayer scenario S03: a
Character presses a button while its bomb is many miles away.

**Known.** Physical proximity, a Connection or a Route cannot by itself explain the
remote consequence. Prose saying “remote trigger” is not executable authority.

**Unknown.** The exact explicit structural basis, who may know it, who may invoke it
and whether pressing and detonation settle atomically or as two explicit Actions.

**Questions.** Can the spatial model remain irrelevant to causality while still
recording where both endpoints and Activities occur? Does hiding the bomb or link
prevent guessed-id access without preventing authorized remote use?

## SP04 — A cup is two centimetres above a table surface

**Narrative.** Cup C is currently described as floating exactly two centimetres
above the surface of Table T. An Agent may later move T, pick up C or deliberately
make the acting Character float with C.

**Known.** C and T are Entities. “Two centimetres above” is quantitative but does
not alone identify a horizontal point, surface shape, direction or movement rule.
The Agent must explicitly propose surprising multi-Entity consequences.

**Unknown.** Whether an exact Position exists; whether T has geometry; whether the
distance is authored or calculated; whether C moves when T translates or rotates;
and which Characters can observe or change the arrangement.

**Questions.** What can one read return without pretending unknown coordinates are
known? Which facts must change when T or C moves? Can two Agents describe the same
current arrangement differently while World retains one coherent mechanical state?

## SP05 — A dog sleeps under a bridge

**Narrative.** Dog D sleeps under Bridge B. D can wake and walk away; B can be
repaired, moved or replaced.

**Known.** D and the physical bridge are Entities. The sentence may be a current
observation, a durable authored description or a fact calculated from spatial state.
Those possibilities are not assumed equivalent.

**Unknown.** Whether D and B have geometry; what “under” means; when the statement
becomes stale; whether B is also involved in a Connection; and who observed it.

**Questions.** Must movement delete or supersede anything? Can World calculate the
statement without interpreting prose? What history remains after D leaves, and what
may a later arrival truthfully learn?

## SP06 — A Character is one hundred metres from a hotel

**Narrative.** While travelling, Character M's Agent says, “another one hundred
metres and I reach Hotel H.” M may approach by road, cross-country or a longer
accessible path.

**Known.** M and H have stable identities, and H is a Place only if World has
established that role. A remaining distance can change without either identity changing.

**Unknown.** Whether one hundred metres is straight-line, path length, authored
estimate or measured value; whether a Route exists; which Position or Area of H
counts as arrival; and whether H is visible or merely known.

**Questions.** Is this current state worth storing or better calculated on demand?
Which input and spatial basis make the number reproducible? How does the answer
remain bounded when many possible paths or changing terrain exist?

## SP07 — A birdhouse is placed on a table in a village

**Narrative.** Character A creates Birdhouse B and places it on Table T in Village V.
Another Character enters V and may discover B. T may later be moved to another Place.

**Known.** B, T and V are independently addressable Entities; V may bear the Place
role. B does not become a Place merely because it can be found spatially.

**Unknown.** Whether B and T each need exact Positions; how “on” is expressed;
whether B moves with T; whether V's Area matters; and which state makes B eligible
for a bounded local read.

**Questions.** Can B be grounded through T without copying every ancestor? What
single-subject change moves T and B coherently? What happens when B is lifted while
another Agent concurrently moves T?

## SP08 — A Character crosses a forest edge into open heath

**Narrative.** Character M leaves a forest, pauses at its irregular edge and enters
open heath without arriving at a city, building or other named destination.

**Known.** M must retain a Position between established Places. The forest or heath
may have incomplete boundaries and neither needs a Place at every stopping point.

**Unknown.** Whether forest and heath are Places, Areas, terrain description or a
combination; whether the edge has geometry; and what observation establishes that M
crossed it.

**Questions.** Can the model describe gradual and uncertain terrain without filling
the World with Places? What exact current fact supports local play at the edge? How
do overlapping, disputed or only partially discovered boundaries behave?

## SP09 — A cabin, passengers and cargo travel on a ship

**Narrative.** Ship S moves through the World while Cabin C, active Characters and
thousands of cargo Entities remain aboard. Play inside C continues during travel.

**Known.** S and every passenger or cargo subject retain stable identities. Moving
S may not require one unbounded atomic rewrite of every subject aboard.

**Unknown.** Whether C is a Place, whether Positions are relative, what causes
movement with S, how deep nested arrangements may be and which interior Actions
depend on S's external Position.

**Questions.** Can one addressed Entity's exact current point be resolved with a
bounded read? Can local cabin play avoid conflicting with every ship movement? What
index finds eligible nearby or aboard subjects without scanning all descendants?

## SP10 — A remembered sword is now hidden in a backpack

**Narrative.** Character B once saw Sword S. Later S is described as being inside
Backpack P carried by another Character. B can still see P but not S.

**Known.** B's prior observation may remain memory but is not a fresh read. Guessing
S's identity cannot reveal its current state or authorize direct mutation.

**Unknown.** Whether S has an exact Position; which hidden current fact relates it
to P; whether it moves with P; and what investigation could make S knowable again.

**Questions.** Can the same spatial model return P while omitting S and even the
hidden fact's existence? How does an authorized holder retrieve contents without a
shared unbounded list? What prevents memory from becoming current authority?

## SP11 — Place A connects to B but not C

**Narrative.** Place A has direct travel to Place B. Place C is geographically near
but has no established direct travel from A. A physical road, door or bridge may be
involved between A and B.

**Known.** A physical road, door or bridge is an Entity; the direct topology is not.
No Connection is inferred from distance, overlapping Area or visible infrastructure.

**Unknown.** Whether `Connection` remains the final name, whether open terrain needs
one, current access, travel cost and whether a later named Route combines several
direct steps.

**Questions.** Can an Agent list bounded direct options without treating C as
reachable? What changes when the physical Entity closes or breaks? Which truth
authorizes travel and which merely explains it?

## SP12 — Many Characters share one unnamed Position

**Narrative.** Two Characters meet and leave an Entity at one unnamed point between
Places. Later, a festival draws a million Characters and Entities to that same point.

**Known.** Co-location does not mint a Place, expose every hidden fact or justify a
global lock. Each subject retains its own identity and attributable history.

**Unknown.** Exact equality or proximity semantics, local visibility, occupancy
representation, admission behavior and how an Agent pages the currently relevant set.

**Questions.** Can one Character move without updating a shared Position owner? Can
reads stay bounded at the hot point while quiet Positions remain responsive? Which
conflicts are real when many subjects share coordinates but change unrelated facts?

## SP13 — A city, forest and waterfall appear on the map

**Narrative.** Explorers establish Moss City, North Forest and Silver Waterfall as
named World subjects. Agents want to map, revisit and describe all three.

**Known.** Each may be an Entity when independently addressable. A Place additionally
requires Position and must be established as an independent map, discovery,
navigation or spatial reference. Area remains optional Place coverage.

**Unknown.** Which subjects earn the Place role; whether the forest has an Area;
whether the waterfall is merely an Entity at a Position; and what “map” may reveal
to a Character that has not discovered them.

**Questions.** Which gameplay fails if each candidate is or is not a Place? Can a
map list Places without enumerating all Entities or disclosing undiscovered ones?
Does Area describe extent without deciding travel, ownership or visibility?
