# Unified World-change scenario catalogue

> **Role / side:** reusable multiplayer scenario input / development side.
> **Authority:** owns the fixed narratives, fixtures and questions reused by
> multiplayer experiments.
> **Excludes:** accepted game behavior, expected product outcomes, production
> architecture and evidence verdicts; see `docs/game/`, the active concept record,
> accepted plans and each experiment's own README.

Status: **active catalogue**. These scenarios are deliberately broader than the
current game contract. A scenario makes a hard case repeatable; it does not authorize
the capability it describes or choose its outcome.

The catalogue tests the direction explored in the draft
[`multiplayer-lab` plan](../../.agents/plans/20260816-153410-multiplayer-lab/plan.md):
one Agent-authored World-change substrate should be able to express creation,
mutation, multi-subject consequences and scoped effects. Current state, one durable
Activity and dirty authoritative resources should follow from the same accepted
change. A proposal round, listener or delivery component may help assemble or
announce that change, but may not become a second source of World truth.

## How to use a scenario

An experiment selects the smallest scenario and records the following in its own
README:

- the exact decision and falsifiable question;
- which parts of the fixture are real and which are simulated;
- the concrete submitted change package or competing packages;
- request, subject, time, connection and cost bounds;
- which outcomes are accepted, rejected, retried or coalesced;
- authoritative state and Activity readback;
- dirty resources, live hints, refetches and any deliberately dropped hints;
- the observation at each layer below; and
- the bounded verdict and what it does not prove.

Names such as `House H`, `Table T` and `Character A` are fixture labels, not stable
World identifiers. An implementation gives every subject its own stable identity.
Entity names or descriptions never establish that two Entities are identical.

### Observation layers

Every scenario involving what another player “notices” reports these layers
separately:

1. **World:** what current state and Activity were durably accepted?
2. **Eligibility:** which Entity, Place or World resources became relevant to which
   active interests under the tested rule?
3. **Delivery:** which connected hosts received a coalescible hint, missed one, or
   had to recover by refetching?
4. **Agent:** was a User's Agent explicitly invoked, and what authoritative context
   could it read? Delivery alone never invokes an Agent.
5. **Presentation:** what did that Agent tell its User or claim that its Character
   perceived?
6. **Later discovery:** what can a Character that reconnects or arrives later learn
   without claiming that it personally saw or heard the occurrence?

### Cross-scenario probes

Unless an experiment explicitly excludes one, exercise these probes:

- first submission and idempotent retry;
- one stale expected fact and one expected-absent fact;
- a nonexistent target, which must not be created implicitly;
- one forced failure after validation but before commit, proving whether the declared
  atomic boundary leaves partial state;
- independent work on a quiet subject while one subject is hot;
- a slow or disconnected observer that loses live hints and recovers through an
  authoritative bounded read; and
- a duplicate, delayed or out-of-order delivery hint that cannot change World truth.

The World may validate identity, authority, current versions, placement and explicit
relations. It may not infer physics, blast radius, hearing, sameness, intent or
causality from prose, Entity names, arbitrary Property keys or Trait text. Each
experiment must therefore state which exact structure the Agent submits and which
operation-specific rule, if any, makes that structure admissible.

## Scenario index

| ID | Scenario | Primary pressure |
| --- | --- | --- |
| S01 | One Agent changes one Entity | Smallest control case |
| S02 | Thousands of Agents change one table | One deliberately hot subject |
| S03 | A button detonates a remote bomb | Explicit non-local causality |
| S04 | A bomb is dropped inside a house | Multi-Entity consequence and observation |
| S05 | A music bomb is heard only by hearing Characters inside | Typed sensory eligibility |
| S06 | An explosion intersects concurrent table changes | Dependencies across simultaneous changes |
| S07 | Many Agents introduce “the same” Entity | Identity and deduplication |
| S08 | One package introduces a linked Entity graph | Atomic creation and temporary references |
| S09 | A target is absent, stale, moved or retired | Freshness and invalid targets |
| S10 | “Everything is blue” | World scope without a mass hot row |
| S11 | One Place is hot while the rest of the World is quiet | Isolation, overload and fairness |
| S12 | A host disconnects and later catches up | Lossy hints and authoritative recovery |
| S13 | Eligible Agents assemble a communal result | Optional collective authority |
| S14 | A causal chain contains a loop | Bounded consequences without a rule cascade |

## S01 — One Agent changes one Entity

**Narrative.** Character A's Agent creates Table T in House H. In later explicit
calls it makes the table blue, adds a material Property and introduces a `sturdy`
Trait.

**Fixture.** House H exists and A has the tested authority there. The create call
uses a stable idempotency key. Each later call names T, the exact facts read and the
exact intended writes.

**Variants.** Submit a compatible Property and Trait change together; retry the same
package; retry the key with a different payload; refer to T before its create commits;
force one member of a multi-change package to fail.

**Questions.** Can create and every later change pass through one settlement path?
Does an accepted package produce exactly the intended current facts, one Activity
and mechanically derived dirty resources? Is a partial Entity ever visible? What is
the smallest useful Agent-facing error for stale, absent, unauthorized and duplicate
input?

## S02 — Thousands of Agents change one table

**Narrative.** Thousands of explicitly invoked Agents act on Table T at nearly the
same time. Some make it blue, some add or remove legs, some add Traits, some turn it
over, and some describe mutually incompatible final states.

**Fixture.** T is in House H. Every package carries a unique actor and idempotency
identity and declares the exact facts it read and intends to change. A quiet Table Q
in another Place receives ordinary work throughout the run.

**Stress variants.**

- all Agents write the same Property key;
- Agents write different Property keys on T;
- one change writes `color` but depends on the current `leg_count`;
- many Agents all expect one Property to be absent;
- packages combine independent and conflicting facts;
- one bounded communal proposal round attempts to assemble one final package; and
- load exceeds the admitted queue for T while work on Q stays below its bound.

**Questions.** Which changes compose and which conflict? Is freshness checked at the
exact fact, whole Entity or another explicit dependency boundary? Does one hot fact
remain bounded without locking the Place or World? Is overload deterministic and
legible? Can a collective round assemble the ordinary package without introducing a
parallel mutation path or allowing listener count to grant authority? What history
can be queried without returning thousands of unbounded records?

## S03 — A button detonates a remote bomb

**Narrative.** Character A presses Button B in House H. Bomb X, located in distant
Place Z and not spatially near H, detonates.

**Fixture.** B, X, H and Z exist. The fixture contains an explicit structural link
or operation-specific capability connecting B to X; prose such as “remote trigger”
is not enough. A's package names the link, its expected version, the bomb and all
declared consequences.

**Variants.** Remove or retarget the link concurrently; move or change X before
commit; make B and X valid but one affected Entity invalid; compare one atomic
package with two explicitly separate Actions connected by a causal Activity
reference; retry after a lost response.

**Questions.** What grants A authority over the remote consequence? Which parts must
commit atomically? If detonation is a later Action, who or what may submit it without
the server invoking an Agent? Which resources become dirty in H and Z? Can local
interest in Z learn promptly while unrelated Places receive no per-recipient World
record?

## S04 — A bomb is dropped inside a house

**Narrative.** Character A drops and detonates Bomb X inside House H. Other players
have Characters inside H, directly outside it and in surrounding Places. Furniture,
a window and part of the house may be affected.

**Fixture.** Put these subjects in explicit locations:

- Character A and active Character B inside H;
- disconnected Character C persistently inside H;
- Character D in Yard Y directly outside H;
- Character E in adjacent Street S;
- Character F in an unrelated Place;
- Table T, Window W and Bomb X inside H; and
- explicit structural relations between H, Y and S, without assuming those
  relations imply blast or hearing range.

A's package declares every materially changed Entity and its intended facts, plus
either a small exact Place set or one compact structurally established scope for the
occurrence. One bounded World-evaluated reach may match many houses without listing
them or requiring a containing district/Area. World does not discover the blast
radius from the word “bomb,” and a compact occurrence scope does not implicitly
mutate every matched Entity.

**Stress variants.** Omit Y from the claimed scope; include an unauthorized Entity;
change W concurrently; disconnect B just before commit; reconnect C after the
occurrence; move D from Y into H while the package settles; detonate two bombs in the
same Place.

**Questions.** Which structural standing makes the submitted scope admissible? Are
all declared consequences atomic or may some be later explicit Actions? Which
characters are eligible for a live hint, and which merely discover public Place
history later? What, specifically, can B, C, D, E and F's Agents truthfully present
as sight, sound, impact or later knowledge? Can delivery stay proportional to dirty
resources and active interests rather than all Characters ever placed nearby?

## S05 — A music bomb is heard only by hearing Characters inside

**Narrative claim under test.** Character A drops Music Bomb M in House H. Its music
is heard only by Characters that both have the relevant hearing capability and are
inside H at the occurrence. It causes no ordinary blast damage.

**Fixture.**

- hearing-enabled active Character B is inside H;
- deaf Character C is inside H;
- Character D has no declared hearing state and is inside H;
- hearing-enabled Character E is in Yard Y outside H;
- hearing-enabled Character F enters H only after the occurrence; and
- hearing-enabled but disconnected Character G remains inside H.

The submitting Agent states that the music is noticeable only by Characters that can
hear. World interprets neither that condition nor arbitrary `hearing` Property or
Trait meaning. World authorizes the structural House scope. Every in-scope receiving
Agent gets the occurrence plus its own authoritative Character state, decides
whether its Character noticed it and silently does nothing when it concludes the
Character is deaf. Out-of-scope and private eligibility remain World-owned filters.

**Stress variants.** C changes hearing state concurrently; B leaves H at the commit
boundary; M repeats rapidly; the delivery hint is lost; F asks what happened after
entering; G reconnects; another Entity has misleading prose saying it “can hear.”

**Questions.** Which occurrence-time or read-time Character Property/Trait state
does the receiving Agent use? What should D conclude when no relevant state exists?
Can C learn that the occurrence happened without presenting that it heard it? Can E
or F retrieve public history without claiming personal hearing? Does coalescing
repeated sound hints preserve the intended game behavior without storing one durable
delivery row per listener? No hint may automatically spend G's tokens.

## S06 — An explosion intersects concurrent table changes

**Narrative.** While Character A's Agent makes Table T blue and Character B's Agent
changes it to three legs, Character C detonates a bomb whose submitted consequence
throws T through Window W into Yard Y.

**Fixture.** T starts inside House H, W connects H to Y through an explicit spatial
relation, and all three packages name their precise read dependencies and writes.
The explosion package does not infer that T should fly merely because it is nearby;
its Agent names T, W, Y and the intended placement/state changes.

**Interleavings.** Color commits before detonation; leg count commits after the
detonation read but before its commit; W breaks first; T moves before detonation;
the explosion and Property changes write disjoint facts; the explosion package
explicitly depends on T's leg count; one package is retried after another commits.

**Questions.** Which serial outcomes are valid? Does a disjoint color change survive
T's movement? Must a changed causal fact reject the explosion even when the intended
write is different? Is T ever simultaneously placed inside and outside? Does stable
subject ordering avoid deadlock across T, W, H and Y? Can all accepted histories
explain the final state without a second consequence engine?

## S07 — Many Agents introduce “the same” Entity

**Narrative.** Thousands of Agents in House H independently try to introduce “the
oak table in the centre.” Some intend one shared table; others happen to use the
same name for distinct tables.

**Fixture.** Every submission has its own stable idempotency identity and concrete
Entity identity unless a tested communal operation supplies one shared
materialization identity. Names and semantic descriptions are intentionally equal.

**Variants.** Same idempotency key and same payload; same key and different payload;
different ids and same name; one accepted communal proposal with many candidate
descriptions; two independent communal rounds collide; create plus exact-absence
dependency.

**Questions.** What prevents accidental duplicates without asking World to infer
semantic sameness? When should two same-named tables both exist? Who may establish a
shared materialization identity? Can retries converge on one Entity while genuinely
independent introductions remain independent? Does an introduction hotspot create a
Place-wide lock or global id allocator?

## S08 — One package introduces a linked Entity graph

**Narrative.** One Agent introduces Table T with properties and Traits, places it in
House H, puts Button B on it and links B to remote Bomb X. Some subjects are new and
some already exist.

**Fixture.** The package uses package-local references for new subjects and stable
World identities for existing H and X. It states every relation explicitly.

**Variants.** A local reference is missing; X does not exist; one relation is
unauthorized; B and T would create a forbidden structural cycle; a concurrent
package claims the same materialization identity; a retry follows a lost response.

**Questions.** Can create, placement, Property, Trait and relation introduction use
one atomic settlement without exposing orphaned or half-configured Entities? How are
package-local references resolved deterministically? What is the maximum bounded
graph size? Which dirty resources follow from success, and are none emitted for a
rejected package?

## S09 — A target is absent, stale, moved or retired

**Narrative.** An Agent tries to paint Table T, but T never existed. In related runs,
T existed when read but changed, moved, or ceased to be an admissible target before
commit.

**Fixture.** Separate cases for an unknown id, expected absence, stale exact fact,
stale placement and an operation-specific inactive/retired target. Retirement is a
future fixture condition, not a current game capability.

**Variants.** Create T concurrently with a call that expected it absent; use a name
that matches another Entity while the id is absent; retry a rejected call; include
one valid and one invalid Entity in an atomic package.

**Questions.** Does every invalid target fail closed without implicit creation or
name matching? Is the error precise enough for an Agent to refetch and deliberately
replan? Does a failed multi-subject package leave every subject and Activity
unchanged? Can expected absence be coordinated without locking the entire Entity,
Place or World?

## S10 — “Everything is blue”

**Narrative.** An authorized Agent declares a World-wide condition under which
everything appears blue. A later requirement may instead demand that every existing
Entity's own `color` Property literally becomes blue.

**Fixture.** Millions of conceptual Entities are distributed across many Places.
The experiment models a structurally scoped World effect separately from literal
per-Entity rewrites and gives each an honest read contract.

**Variants.** Add an Entity after the effect begins; end the effect; layer a
Place-scoped red effect; read an Entity with its own green Property; require a durable
literal blue Property on only one bounded subset; connect millions of simulated
interests without claiming production throughput.

**Questions.** Can one scoped effect use the ordinary package, Activity and dirty
resource derivation without one global revision that every mutation contends on?
How does an Agent distinguish contextual appearance from owned Entity state? Which
resource invalidates active views? When the requirement truly is a literal rewrite,
does the system expose it as bounded proportional work instead of pretending that
millions of writes are one cheap atomic mutation?

## S11 — One Place is hot while the rest of the World is quiet

**Narrative.** Thousands of Characters introduce and change Entities inside House H
while millions of conceptual Characters act independently across other Places.

**Fixture.** One deliberately hot Place, one deliberately hot Entity inside it and
many disjoint Place/Entity pairs. Run the same semantic package through a bounded
in-memory model first and only later through a real database or delivery seam.

**Variants.** Hot writes on one exact fact; independent facts on one Entity; distinct
Entities sharing H; new placements that touch H's integrity row; slow observers;
connection-pool exhaustion; admission before and after transaction start.

**Questions.** Does H contention stay scoped to the smallest structural subjects?
Can quiet work proceed without a global lock, counter, queue or hot row? Where is
overload rejected, and is retry guidance bounded? Do connection and delivery limits
fail locally? Which measurements are semantic evidence, which are local capacity
measurements and which would still require production-scale proof?

## S12 — A host disconnects and later catches up

**Narrative.** Character A actively follows House H, then its host disconnects while
many accepted changes occur. It later reconnects after hints were coalesced, delayed,
duplicated or lost.

**Fixture.** Record authoritative state and Activities once. Treat delivery as
disposable state owned outside World truth. Give the reconnecting host a last known
resource identity or cursor only if the tested contract explicitly defines one.

**Variants.** Lose every hint; deliver only the newest dirty-resource hint; deliver
duplicates out of order; reconnect after bounded Activity history has rolled past;
change Place membership while offline; reconnect two devices for one User.

**Questions.** Can baseline/refetch always recover current truth? What bounded recent
context is available, and what gap is honestly reported? Can the Agent avoid claiming
its Character personally perceived occurrences while inactive? Does reconnect avoid
per-recipient replay storage and any automatic Agent invocation?

## S13 — Eligible Agents assemble a communal result

**Narrative.** Table T is under an explicitly communal authority. Many eligible,
explicitly invoked Agents propose colors, leg arrangements and Traits, then attempt
to settle one final state.

**Fixture.** The operation defines eligibility, a deadline or bounded round, maximum
proposal size and a deterministic settlement rule. Candidate meaning comes from
Agents; World stores and validates only the admitted structure and final package.

**Variants.** Sealed independent proposals; one bounded critique round; abstention;
conflicting majorities; identical proposals; malicious text; an Agent disconnects;
millions are eligible but only a bounded admitted set participates; T changes outside
the round before settlement.

**Questions.** What grants communal authority? Does the round produce the exact same
ordinary change package as a single Agent, with no alternate truth state? How is
freshness checked at settlement? Can proposal traffic remain bounded without scores,
automatic Agent calls or listener-count authority? How are an inconclusive round and
a stale final package distinguished?

## S14 — A causal chain contains a loop

**Narrative.** Button A can affect Bomb B; B's submitted consequence can break Window
W; W is linked to Alarm C; and C points back to A. A naive ripple system could run
forever or repeatedly mutate the same subjects.

**Fixture.** Every link is explicit. The tested operation declares either one bounded
multi-subject package or a bounded set of later explicit Actions with causal
references. No prose or generic server rule automatically traverses the graph.

**Variants.** Acyclic chain within the bound; self-link; repeated subject; chain over
the subject-count bound; concurrent link change; duplicate delivery of one causal
hint; a User explicitly invokes an Agent for the next Action.

**Questions.** Where is the causal boundary? Are cycles rejected structurally,
collapsed through idempotency or simply inert until another explicit Action is
submitted? Can one Activity explain one accepted package without becoming generic
event sourcing? Does duplicate notification ever cause a second World mutation?
Can a future concrete chain mechanic exist without a background rule cascade or
server-triggered token spend?

## Coverage matrix for future experiments

Each experiment marks only the cells it actually exercises. A blank cell is not a
failure; it is an explicit non-claim.

| Scenario | Create | Exact-fact concurrency | Multi-subject atomicity | Remote or scoped change | Observation/delivery | Hot-subject bound | Agent collective |
| --- | --- | --- | --- | --- | --- | --- | --- |
| S01 | Yes | Light | Yes | No | Light | No | No |
| S02 | No | Heavy | Yes | No | Light | Heavy | Optional |
| S03 | No | Medium | Heavy | Heavy | Heavy | Medium | No |
| S04 | No | Medium | Heavy | Heavy | Heavy | Medium | Optional |
| S05 | No | Medium | Light | Place-scoped | Heavy | Medium | No |
| S06 | No | Heavy | Heavy | Heavy | Medium | Heavy | No |
| S07 | Heavy | Expected absence | Medium | No | Light | Heavy | Optional |
| S08 | Heavy | Medium | Heavy | Heavy | Medium | Medium | No |
| S09 | Conditional | Heavy | Heavy | No | No | Medium | No |
| S10 | Conditional | Light | Medium | World-scoped | Heavy | Heavy | No |
| S11 | Heavy | Heavy | Medium | Place-scoped | Heavy | Heavy | No |
| S12 | No | No | No | No | Heavy | Medium | No |
| S13 | No | Heavy | Heavy | No | Medium | Heavy | Heavy |
| S14 | No | Medium | Heavy | Heavy | Medium | Medium | No |

This matrix is an experiment-selection aid, not a completeness claim. A production
design remains unproved until its accepted scenarios pass at the real semantic,
database, protocol and failure seams named by its plan.
