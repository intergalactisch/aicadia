# Time and turns

> **Concept log** — we're still discovering. Exploration, not specification.

## The daily turn — current direction

- **One canon-eligible scene per player per rolling day.** A scene credit accrues
  every 24 hours; you hold at most 3 (absence is forgiven mechanically, not
  socially). No global clock needed — the cap is fairness, not ceremony.
- **Gestures**: a small daily handful. A letter, rumor, song or witnessed detail
  appends its own accepted claims and provenance; it never changes an earlier claim.
  A gesture may update a current projection only when that projection's explicit
  authority, locality, time and replacement rules permit it. Repeating a report is
  not a direct observation.
- **Reading is free and unlimited.**
- The ritual is ~15 minutes: reading and gestures are the bulk; the scene is the
  punctuation, not the point. (Debate finding: a cap of one with nothing else to do is a
  90-second session, and 90 seconds builds no habit. The reading surface carries the
  ritual.)

## Realtime, not turns — user direction (2026-07-25)

The synchronized daily dawn is dropped. On reflection it was a shipped institution —
exactly what principle 8 forbids. The world runs in realtime, like a real place:

- Scenes and gestures process **on arrival**. No global resolution moment, no batch.
  If you and I are both in the tavern now, my scene appears for you now — things
  happen before your eyes, not in hindsight.
- The "morning report" is dead as a ceremony. What remains is the **catch-up**: a
  plain query, answered when *you* show up — the ripples that touched your character
  since you last looked (technically: your inbox diff, see doc 09). Your agent reads
  the rows and tells you what they mean; the server composes nothing.
- Fork deadlines are real clocks (the caravan leaves when it leaves); witness windows
  roll. If the world ever wants shared synchronized moments, it will develop markets
  and festivals itself.

### The ripple pipeline (deterministic, no LLM, per event)

1. Extract facts from a scene on arrival.
2. Route **ripples** to whom they plausibly touch — by location, relationships,
   possessions, weather. The butterfly effect as infrastructure: the miller left on a
   journey → days later the baker's catch-up mentions the flour supply.
3. **Bad news travels faster** — idea, see doc 05: scenes of need, loss and failure
   ripple further and land in more catch-ups. Vulnerability buys attention and
   incoming care — never currency. This is the anti-niceness-deadlock mechanism.
4. A character's **catch-up** is largely a *place* report: what moved in the places
   and institutions of your life since you last looked.

## The absence model — idea from the debate (the big one)

Characters have **no autonomous volition**. What moves while a player is away:

- **Places have momentum; people don't.** The world's clockwork lives in places and
  institutions: the weir silts up, the pass closes, the orchard fruits, the flour runs
  short. Absent characters are simply off-stage — unrendered, unharmed, unauthored.
- **Standing orders.** A player may pre-author their character's routine once ("she
  opens the bakery at dawn; she checks the weir after rain"). The server may state the
  routine as fact — it executes a commitment, it never invents one. (Prior art:
  correspondence-chess conditional moves; Diplomacy's civil-disorder hold.)
- **Circumstances still happen to anyone** (leefregel 3): storms, gossip, windfalls.
- **Forks expire; they never resolve.** Nobody — no resolver, no observer, no referee
  model — ever authors an absent character's choice. The caravan leaves at dawn and she
  is still standing at the gate; the apprenticeship goes to someone else; the offer
  lapses. Missed chances are better fiction than arbitrary ones, cost zero tokens, and
  keep consent line 1 absolute.
- **No visible "hesitation" labels.** Marking a character as "hesitating for three days"
  leaks out-of-character absence into fiction and is a guilt mechanic in costume.
  Banned. Absence must be indistinguishable from an undramatic day.

## Punctuated equilibrium — idea from the debate

Daily scenes are texture and free to miss. **Irreversible change crystallizes at
communal moments** — a market, an assembly, a festival, as the world develops them
(never system-scheduled) — and what changes is **computed from aggregate play, not
voted**: the dish cooked most often
becomes the festival dish; the path walked most becomes the road; the name used most
sticks. Deterministic, no LLM, and retention comes from a known payoff date rather than
daily FOMO: missing a Tuesday is genuinely free, the week is irresistible.

Cozy retention is seasonal, not continuous (Animal Crossing's two-month cliff). Design
for return waves — recurring festivals and seasons as the world discovers them — not
for DAU.

## The world steward — user direction (2026-07-26)

The first version has one deliberately small, administrator-only way to introduce
shared external pressure. It is not a background simulation and it does not give every
player a meta-author role:

1. The administrator consciously invokes the world-steward skill.
2. The steward queries a bounded meta-briefing of current canon and asks for more
   world detail where needed.
3. It presents **exactly three** relevant `world move` proposals, plus the explicit
   option to do nothing. The proposals have no fixed categories; the steward derives
   whatever three interventions appear most interesting from the current world.
4. The administrator chooses one direction or nothing. Nothing creates no package and
   changes no state. Unchosen proposals and a do-nothing result remain private
   workshop material and are not retained as world or meta-admin history.
5. For a chosen direction, the steward drafts the exact public source package.
6. The administrator inspects and explicitly confirms that complete package before it
   enters canon, like every other public commit.

One invocation can therefore produce at most one `world move`. A `world move` adds new
history through the ordinary append-only scene path; it never edits existing history
or mutates current state directly. Its provenance permanently identifies it as a
meta-admin change for authorised administrators. Player-facing clients and player
agents receive only the fictional history, not its administrative origin. It may
introduce an impersonal circumstance or unresolved pressure. It may not write a played
character's voice, choice, inner life or ending; award wealth, rank, mastery or
institutional status; or decide how the circumstance resolves.

The intelligence and token spend live in the administrator's consciously started
agent session. The server remains the dumb chronicle-keeper. Frequency and reach beyond
this minimum remain open until real use proves they are needed.

## Emergent time — user direction (2026-07-25)

The server's only time is the timestamp — time simply flows, in realtime. No shipped calendar, no named seasons,
no epochs, no planned ending — the world meanders and develops ("alles kabbelt voort").

- Weather and seasons enter canon when first *written*, and recur because canon says
  they do. The first winter is a founding cultural event, not a config value.
- Named time units (a week, a month, a year) are culture: they crystallize when many
  people use them, via the same aggregate mechanism as everything else.
- If the world ever feels the need for a telling, an ending-rite, an era — it may
  develop one, together. The system never imposes it, and one player can never author
  such a layer alone.
- Institution test: **an institution is a habit the world noticed.** Anyone can start
  a practice; only repetition by others makes it real.

## Letters and social debt — idea from the debate

Play-by-post communities die of owed replies, not boredom. Therefore:

- A letter must always be answerable with **one gesture** — it never obligates a scene.
- Social debt **expires visibly**: after N days a letter is marked as no longer
  expecting a reply. A statute of limitations on obligation is the anti-dark-pattern.
