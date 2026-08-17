---
status: pending
era: August Activity-Property-Trait
---

# Stochastic discovery and bad-luck protection

> **Role / side:** sourced research report / development side.
> **Authority:** records this report's question, sources, findings and implications.
> **Excludes:** product decisions and current implementation contracts; see `docs/game/`.

Date: 2026-08-09

Status: research; no Aicadia game-behaviour or implementation decision

Related research: [World time and sparse simulation](world-time-and-sparse-simulation.md)
separates an explicit action from elapsed time and background work. This report
narrows the question to resolving one accepted action.

## Question

For a Character-bound investigation that deliberately attempts something new and
resolves a hidden chance table into zero, one or sometimes several shared World
results, which chance models can invite interaction without turning the mechanic
into retry spam, grind optimisation or a visible progress meter?

The comparison covers independent weighted choices, independent multi-roll tables,
hard and rising-odds guarantees, entropy-like bounded sequences, sampling without
replacement and variable result counts. It also separates gameplay probability from
API retries and operational throttling.

## Evidence boundary

The evidence is limited to first-party documentation, owner-hosted developer posts
and project-owned source documentation pinned to the inspected revision where
possible. These systems solve different problems: Cataclysm: DDA documents actual
item-group resolution; Warframe and Hearthstone document live reward policies;
GameplayKit documents general game randomisation; Path of Exile documents an
entropy mechanism for combat rather than loot.

The Path of Exile mechanism is included as evidence for bounded streaks, not as
evidence that combat entropy transfers unchanged to rewards. Community-inferred
Diablo drop rates or pity rules were excluded because no sufficiently specific
owner source was established. Research describes mechanics and trade-offs; it does
not choose an Aicadia model.

## Terms that must remain separate

| Term | Meaning here |
|---|---|
| eligible attempt | One accepted, intentional gameplay action allowed to consume a new resolution |
| delivery retry | Re-delivery of the same command after a timeout or lost response |
| independent roll | A draw whose probability is unchanged by earlier outcomes |
| weighted choice | One selection where each candidate's chance is relative to the sum of eligible weights |
| multi-roll | Several candidate tests or count draws inside one attempt, allowing zero, one or multiple results |
| drought | Consecutive eligible attempts without the specified result |
| bad-luck protection | Gameplay state that changes or constrains later outcomes because of earlier eligible outcomes |
| hard pity | A rule that guarantees a specified result no later than a defined eligible attempt |
| soft pity | A rule that improves the probability after misses but need not guarantee a result unless it reaches 100% |
| reset scope | The Character, target, category, table version or cycle whose success or reset affects protection state |
| disclosure | Which probabilities, guarantees or accumulated state the player can inspect |

Three controls are orthogonal:

1. **Technical idempotency** makes a delivery retry return the already committed
   resolution. It prevents a timeout from becoming a free reroll.
2. **Operational rate limiting** caps request frequency or concurrent load. It can
   slow spam but does not make a sequence fairer.
3. **Gameplay bad-luck protection** changes the distribution of a later *new,
   eligible* attempt because earlier eligible outcomes occurred.

A system can require all three, but one cannot substitute for another. In
particular, a one-minute rate limit leaves an independent one-percent chance
mathematically unchanged and merely stretches the same grind over wall time.

## Main findings

1. Independent tables are the smallest state model, but they have no maximum
   drought. If a target has probability `p` per eligible attempt, the probability
   of still missing after `n` attempts is `(1 - p)^n`. Cheap repeatable attempts
   therefore make repetition the rational strategy.
2. A recurring hard or rising-odds pity rule is functionally a streak counter even
   when it is hidden or derived from history. Omitting a `pity_count` column changes
   storage shape, not the mechanic.
3. Bounded-drought systems move optimisation rather than erase it. Hard pity makes
   every eligible miss progress toward a guarantee; rising odds increase the value
   of continuing; a nearly exhausted shuffle bag makes later draws more predictable.
4. Sampling without replacement bounds composition over a cycle without counting
   failures against one named outcome. It requires durable sequence state and a
   precisely defined reset boundary.
5. Zero/one/many is not one probability choice. Independent candidate tests,
   choose-one distributions, count draws and uniqueness constraints produce
   materially different correlations and player incentives.
6. Per-Character protection and shared World results pull in different directions.
   Independent protected sequences are fair to individual Characters, but alternate
   Characters or accounts can multiply protected opportunities to create shared
   state.
7. Hiding a progress meter does not remove progress state. It changes what players
   can verify and may encourage external tracking or superstition, especially when
   reset scope is unclear.
8. Time need not participate in probability. An explicit accepted attempt can
   resolve synchronously; its timestamp records when it happened rather than
   advancing odds, refreshing a bag or generating a background attempt.

## Comparative evidence

| System | Resolution model | Zero/one/many | Durable or sequence state | Bound and disclosure |
|---|---|---|---|---|
| Cataclysm: DDA item groups | `distribution` chooses one relative-weight entry; `collection` tests every entry independently | Distribution: one; collection: zero through many; `count` may repeat an item or draw a min/max count | No protection state is defined by the item-group format | No drought bound in the documented table model |
| Warframe Tauforged Shards | Base 20%, +20 percentage points after each miss for that Shard variant, capped at 100%; success resets it | One reward variant per eligible Hunt | Per-player, per-variant miss history or an equivalent durable level | Rising odds plus a hard cap; current percentage is shown |
| Hearthstone card packs | First-Legendary guarantee, duplicate protection and collection-dependent Catch-Up Pack sizing | Ordinary packs contain several cards; Catch-Up Packs contain 5–50 | Per-account set ownership/receipt history and guarantee eligibility | Guarantee and sizing rules are disclosed; no live streak meter is described in the cited pages |
| Apple `GKShuffledDistribution` | Uniform values in shuffled order, with no value repeated until every possible value was used | One value per call; a shuffled array can provide a subset | Random-source state or an equivalent permutation and cursor | Bounded repetition; the API documents the rule, but presentation is game-specific |
| Path of Exile evasion | Random initial entropy; each attack adds its hit chance and a threshold emits/subtracts 100 | Binary hit/miss per test | One entropy accumulator per entity during the active sequence | Removes long combat streaks; resets to a new random start after inactivity |

### Cataclysm: DDA: independent collections versus choose-one distributions

Cataclysm: DDA's item-group documentation makes the zero/one/many distinction
explicit. In a `collection`, every entry is selected independently with an absolute
0–100 percent probability. In a `distribution`, exactly one entry is selected and
each probability is a relative weight. Its worked example gives two entries with
weights 30 and 20: the distribution selects only A or only B at 60/40, whereas the
collection can select neither, either one or both. Entries may nest other
collections or distributions.
[Cataclysm: DDA item spawn system, inspected commit](https://github.com/CleverRaven/Cataclysm-DDA/blob/c92176491b0494a2520cacb62de4c9938fe63681/doc/JSON/ITEM_SPAWN.md#collection-or-distribution)

The same format lets `count` repeat item creation and lets a two-value count array
draw an inclusive minimum and maximum. Each repeated item's damage is rolled
separately in the documented example. This shows that result count, candidate
selection and per-result properties are independent design dimensions rather than
one generic “loot roll.”
[Cataclysm: DDA entry fields, inspected commit](https://github.com/CleverRaven/Cataclysm-DDA/blob/c92176491b0494a2520cacb62de4c9938fe63681/doc/JSON/ITEM_SPAWN.md#entries-array)

Neither table subtype carries bad-luck state. Repeating a collection can therefore
produce arbitrarily many empty resolutions, while repeating a distribution always
produces one eligible entry. An explicit “nothing” entry can make a distribution
produce a semantic zero, but it remains a choose-one draw. That modelling choice
changes correlations: two rare discoveries in an independent collection can appear
together, while two sibling entries in one distribution cannot.

### Warframe: rising odds that terminate in a hard guarantee

Warframe's 2023 Tauforged Archon Shard change assigns each player their own
probability for each Amber, Azure and Crimson variant. The base chance is 20%; a
miss adds 20 percentage points for the next Hunt of that variant, up to 100%.
Receiving that variant resets only its chance to 20% and leaves other variants
unchanged. The current chance is displayed in the Hunt panel.
[Warframe Update 32.3](https://www.warframe.com/en/patch-notes/pc/32-3-0#tauforged-archon-shard-probability-change)

This is both soft and hard pity: odds rise after each miss, and the cap eventually
makes success certain. It is also unambiguously a per-player, per-variant failure
streak in semantic terms. The owner source does not reveal backend storage. The
server could store a level from 0 through 4 or derive the same level from durable
Hunt outcomes, but either implementation must preserve the equivalent state across
weeks and resets.

The example exposes why reset scope matters. An Amber success does not erase an
Azure drought. A broader “any discovery succeeded” reset would be a different
mechanic, as would carrying accumulated odds into a revised reward table. Showing
the percentage improves inspectability but also creates exactly the visible
progress signal that the Aicadia question seeks to avoid.

### Hearthstone: scoped guarantees, collection exclusion and variable counts

Blizzard's card-pack change guarantees a Legendary within the first ten packs of a
new set. When a Legendary is opened it comes from that same set and is one the
account does not own until all Legendaries in the set are owned. The same change
also prevents one pack from containing more copies of a card than can be used in a
deck.
[Hearthstone card-pack changes](https://hearthstone.blizzard.com/en-us/news/20852959/hearthstone-update-upcoming-card-pack-changes)

This is a hard *initial* guarantee scoped to account and set, not evidence for a
recurring hard-pity interval. It requires a number of eligible pack openings or an
equivalent fulfilled/not-fulfilled state, but it is not a failure-streak counter
that resets after every later success. Duplicate protection instead depends on
collection membership: it is sampling from an eligible remainder, not rising odds.

Catch-Up Packs show that one action can use several count and guarantee rules. Each
pack takes 1–10 cards from each of five fixed sets, for 5–50 total, with the count
per set based on the percentage of that set previously received. At least 20% of
the pack is Rare or better, and the first 50 cards from each included expansion
contain a Legendary. Cards once received or crafted continue to count even if later
disenchanted.
[Hearthstone Catch-Up Packs](https://hearthstone.blizzard.com/en-us/news/24008690/introducing-catch-up-packs)

The history rule prevents disposal from reopening a protected pool. It also shows
the cost of collection-relative counts: the output amount is directly coupled to
an account progress measure. For a system that rejects scores or counters in its
schema and API, that is a materially different fit from a fixed hidden table.

No ongoing “Legendary every N normal packs” claim is made here because the inspected
owner pages do not establish one. Community measurements are not promoted to a
first-party rule.

### GameplayKit: a shuffle bag bounds repetition, not effort

Apple describes `GKShuffledDistribution` as roughly uniform across many samples
while preventing any value from repeating until every possible value has been used.
With a six-sided die, a rolled value cannot recur for at least five more rolls.
Apple presents this as “fair” randomisation because truly random sequences can
contain long lucky or unlucky streaks.
[Apple `GKShuffledDistribution`](https://developer.apple.com/documentation/gameplaykit/gkshuffleddistribution)

GameplayKit random sources are deterministic and independent. Apple documents
preserving a source by archiving it; restoring the same archived state reproduces
the same subsequent sequence. It also warns that unrelated game randomisation
should use independent sources so one visible stream cannot reveal another. These
properties matter for multiplayer replay and for preventing flavour draws from
advancing discovery outcomes.
[Apple GameplayKit randomisation guide](https://developer.apple.com/library/archive/documentation/General/Conceptual/GameplayKit_Guide/RandomSources.html)

A shuffle bag is not a pity streak counter. Its state is the remaining permutation
or an equivalent seed plus cursor. A weighted bag can be constructed by placing
multiple copies of outcomes in a finite multiset, but that extension introduces
weight granularity and is an inference, not behaviour promised by Apple's class.

The bounded sequence also creates a different exploit surface. If a player can
infer which results have already appeared, the shrinking remainder becomes more
predictable. Resetting on restart would restore reroll opportunities; resetting on
Character, table version, target or success would each define a different game.
Durability is therefore part of the guarantee, not merely an implementation detail.

### Path of Exile: entropy spreads outcomes and constrains state transfer

Grinding Gear Games developer Mark_GGG documented Path of Exile evasion as an
entropy accumulator rather than independent rolls. Every entity starts with a
random value from 1 to 100; an incoming attack adds its chance to hit; crossing 100
causes a hit and subtracts 100. At a stable 25% chance, for example, every fourth
attack hits after the random starting phase. The stated purpose is to eliminate
damaging lucky or unlucky streaks while retaining the long-run chance.
[Path of Exile developer explanation](https://www.pathofexile.com/forum/view-thread/11707/page/106#p251314)

The same explanation deliberately resets the initial value after a short inactive
period so a player cannot prepare favourable entropy on a weak enemy and transfer it
to a boss. A later staff answer identifies the inactivity interval as about three
seconds and confirms that the value is used whenever an enemy attacks, whether the
result hits or misses.
[Path of Exile entropy reset clarification](https://www.pathofexile.com/forum/view-thread/324357#p2638067)

Entropy is not a failure-streak counter: misses and hits both transform one numeric
accumulator, and changing hit chance changes the increment. It nevertheless is a
counter-like piece of state. Porting it to persistent discovery would require a new
answer for reset scope; Path of Exile's real-time inactivity reset is specifically
an anti-transfer combat rule and would make elapsed time affect outcomes if copied
literally.

## State accounting: what is actually a counter?

| Model | Is it a pity streak counter? | Minimum equivalent state if protection survives requests | Can it be derived? |
|---|---|---|---|
| independent weighted/collection rolls | No | none beyond the committed attempt/result | not applicable |
| recurring hard pity after failures | Yes | consecutive eligible failures in the reset scope | yes, from a complete ordered attempt history |
| rising odds after failures | Yes | failure streak or current odds tier in the reset scope | yes, from a complete ordered attempt history |
| first-N initial guarantee | No failure streak, but it counts eligibility until fulfilled | eligible openings plus fulfilled state | yes, from complete opening history |
| entropy accumulator | No pity streak; it is still a numeric accumulator | current accumulator and its reset context | sometimes, if every ordered input and reset is retained |
| shuffle bag | No | remaining multiset/permutation or stable seed, cycle and cursor | yes, from stable seed plus ordered consumption history |
| duplicate protection | No | historical eligibility/receipt set | yes, from immutable acquisition history |

Deriving state avoids a dedicated counter field but does not make the mechanic
stateless. A query that counts consecutive misses on every attempt still implements
a streak counter. Conversely, snapshotting a derived value is an optimisation only
if the authoritative history and reset rules remain clear.

This distinction is material under Aicadia's current prohibition on counters in
schema and API. Independent rolls need no progression state. Recurring hard pity
and rising odds are functionally counter mechanics even if hidden. Shuffle and
entropy replace the failure streak with other durable sequence state; they do not
eliminate state. Research alone cannot decide whether such internal state is an
acceptable evolution of that rule.

## Multi-result resolution choices

For a hidden table with candidates `A`, `B` and `C`, “zero, one or several” still
requires an explicit composition rule:

| Rule | Possible outcomes | Important correlation |
|---|---|---|
| test every candidate independently | any subset, including none and all | one success does not consume or exclude another |
| choose one weighted entry including `nothing` | exactly one table entry | candidates compete for the same probability mass |
| draw `k`, then sample without replacement | exactly `k` distinct results | count is controlled; duplicates within the attempt are impossible |
| draw `k`, then sample with replacement | exactly `k`, duplicates possible | a rare result can appear more than once |
| guarantee one after zero, then roll extras | at least one after the guarantee triggers | protection may change result count as well as content |
| replace one failed slot with the guaranteed result | original count preserved | guarantee displaces another possible result |

Bad-luck state also needs a success predicate. If one attempt returns two ordinary
results and misses the protected rare result, does the drought continue? If it
returns two rare results, does that consume one guarantee, start a new cycle, or
create two shared facts? There is no answer implicit in “pity.”

For shared World results, materialisation must additionally be atomic. The attempt,
all zero/one/many outcomes, any protection-state transition and any new shared
records must commit together. Otherwise a crash after writing one result but before
advancing sequence state can duplicate or reroll the outcome on retry.

## Behavioural and abuse trade-offs

### Retry spam and grind optimisation

- **Independent rolls:** every valid attempt has the same expected value. If attempts
  are cheap and repeatable, maximal repetition is rational; a hidden low rate makes
  the required effort especially opaque.
- **Hard pity:** bounds the worst drought, but each miss becomes guaranteed progress.
  This can make exhaustive repetition more, not less, legible.
- **Rising odds:** reduces droughts while strengthening “one more attempt” pressure.
  Showing the current percentage makes that pressure explicit; hiding it removes
  the meter but not the incentive.
- **Entropy:** regularises spacing, but a known accumulator can invite setup and
  transfer exploits. Reset rules intended to stop those exploits can themselves
  become reroll strategies.
- **Shuffle bag:** bounds cycle composition, but observed draws leak information
  about the remainder. A large or secret bag reduces predictability but also makes
  the guarantee less intelligible.
- **Multiple results per attempt:** can reduce the number of empty experiences, but
  also concentrates World creation into optimised attempts and makes protection
  interaction more complex.

No chance model by itself makes an attempt meaningful. The strongest anti-spam
boundary is semantic eligibility: the player must do something that changes the
investigation context enough to constitute a genuinely new attempt. A cooldown can
protect capacity, but if waiting alone recreates eligibility it turns wall time into
the price of another roll.

### Character, account and cooperation scope

A per-Character sequence means two Characters can stand at the same place with
different odds or bag state. Warframe demonstrates that individual reward odds can
coexist in shared cooperative activity, but Aicadia's proposed outputs are shared
World facts rather than merely private loot.

That creates several neutral choices with different consequences:

- one shared investigation roll gives collaborators one consistent outcome but
  does not use independent per-Character protection;
- one roll per Character preserves individual sequences but can create conflicting
  or duplicate shared outcomes;
- consolidating several Character rolls needs a deterministic rule for zero, one
  or multiple accepted World results;
- newly created Characters reset per-Character state by definition, so alternate
  Characters become extra protected sequences;
- moving protection to a User narrows that loophole but contradicts the stated
  Character-bound premise, while multiple accounts remain an operational concern.

Uniqueness constraints can prevent duplicate shared records, but they do not answer
whether a duplicate roll counts as success, failure, refund or no-op for a
Character's sequence. That must be part of the probability contract.

### Disclosure and auditability

Disclosure has at least three independent levels:

1. reveal the eligible outcomes but not their weights;
2. reveal the existence and reset scope of protection but not accumulated state;
3. reveal current odds, remaining bag composition or distance to guarantee.

Warframe uses the third level for its rising odds; Hearthstone discloses guarantee
and collection rules without the cited pages describing a live pity meter. A fully
hidden rule avoids an in-product progress display, but players can still reconstruct
or speculate about it. Server-side audit data is also needed to diagnose disputed
outcomes even when the Agent-facing API exposes no counter.

Auditability does not require publishing a seed that permits future prediction. A
committed attempt can retain its stable identity, table version, eligible context,
resolved results and enough protected server evidence to replay or investigate the
decision without exposing the next draw.

## Compatibility with the stated Aicadia constraints

The following are implications, not decisions:

- **Dumb, strict server:** every compared model can be resolved by ordinary
  deterministic validation and pseudorandom computation. None requires an LLM.
- **No server-side Agent runs:** resolution occurs only inside an explicit accepted
  investigation call. A miss schedules no follow-up and spends no future tokens.
- **Time as metadata:** `occurred_at` can record the attempt. Odds, eligibility and
  sequence state need not advance with wall clock, world time or a background tick.
- **Multiplayer consistency:** the server must be authoritative. Separate random
  sources or stable domains prevent unrelated random features from consuming the
  discovery sequence; committed outcomes survive restart and retry.
- **No score/counter in schema or API:** recurring hard pity and rising odds are
  semantically streak counters whether stored or derived. Entropy and shuffle bags
  require different sequence state. Independent rolls are the only compared family
  with no cross-attempt progression state.
- **Shared World result:** resolution and materialisation need one atomic boundary;
  per-Character chance state cannot silently override uniqueness or conflict rules
  for shared facts.
- **Hidden chance table:** hiding weights is compatible with all models, but hidden
  reset rules weaken player understanding and operational auditability. Internal
  evidence and player-facing disclosure are separate choices.

The idempotent request boundary is invariant across all options: a client-supplied
or server-issued attempt identity denotes one gameplay attempt. A retry with that
identity returns the same zero/one/many result and cannot consume another draw. A
new identity is not sufficient by itself to prove a new attempt; eligibility remains
a World rule.

## Neutral trade-off summary

| Model | Main benefit | Main cost | Character-bound shared-world risk |
|---|---|---|---|
| independent weighted or collection rolls | smallest model; no cross-attempt state | unbounded drought and constant incentive to repeat | many Characters multiply independent opportunities |
| hard pity | explicit maximum drought | functionally a counter; misses become progress | reset farming and guarantee multiplication |
| rising odds | smoother drought distribution | counter state and “keep going” pressure | different hidden odds for collaborators |
| entropy accumulator | preserves average while spacing outcomes | state reacts to ordering and reset context | setup/transfer exploits; varying tables complicate increments |
| shuffle bag | bounded composition without a failure streak | durable bag state and end-of-cycle predictability | each Character or account owns another protected bag |
| without-replacement eligibility | avoids duplicate known results | requires durable historical membership | shared discoveries rapidly alter every Character's eligible pool |
| multi-roll or variable count | naturally permits several results | correlations and pity interaction must be specified | one optimised action can materialise many shared facts |

The research does not identify a model that simultaneously supplies a bounded
drought, zero cross-attempt state, no grind incentive and no information leak. Those
properties conflict. The meaningful comparison begins only after the game defines
what earns another eligible attempt and what a “success” means when results are
shared.

## Smallest question still to decide

**What exact change in Character, Place or investigation context makes a request a
new eligible attempt, rather than a delivery retry or repetition of the same search?**

That one boundary determines whether any chance model invites exploration or merely
prices repetition. Only after it is concrete can a later decision compare an
independent draw, a bounded sequence or no random miss at all without accidentally
designing retry spam. It must also specify whether an already-existing shared result
counts as success; no pity or table model can infer that from probability alone.
