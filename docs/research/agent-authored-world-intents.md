---
status: pending
era: August Activity-Property-Trait
---

# Agent-authored bounded World intents

> **Role / side:** sourced research report / development side.
> **Authority:** records this report's question, primary evidence, inferences and candidate implications.
> **Excludes:** product decisions and current implementation contracts; see `docs/game/`.

Date: 2026-08-16

Status: research; no proposal, discussion, voting, cooldown or multi-Place Action
below is accepted Aicadia behavior

## Question

How can intelligent, potentially untrusted Agents compose the meaning of a World
change while a dumb authoritative World remains deterministic and strict?

The concrete case is an Agent concluding:

> There is an explosion. It affects Places A and B, and these are the exact
> consequences for the involved Entities.

The Agent should be able to submit that conclusion in a bounded machine-readable
form. World must not infer blast meaning from prose or require every Entity to carry
ceremonial lifecycle Properties such as `form`, `exists` or `shape`. At the same
time, one Agent must not gain authority to name arbitrary Places or overwrite
arbitrary facts merely by claiming that they are affected.

The research therefore asks:

- what established systems let clients declare explicit targets, dependencies or
  read/write scope;
- what the authority independently validates;
- how exact actual facts and expected absence can support optimistic concurrency;
- how spatial or multi-region scope can be structurally verified without natural-
  language interpretation;
- what subscriptions and interest management contribute;
- how Agents can propose, discuss and settle a bounded choice;
- what happens when many requests converge on one hot subject; and
- which trust limits no protocol can remove.

## Evidence boundary

This report uses primary sources only: official protocol and engine documentation,
standards, first-party system documentation and original papers. The examined
systems solve different problems. A blockchain transaction, agent communication
standard or game-engine replication feature is evidence for a particular mechanism,
not proof that its complete architecture belongs in Aicadia.

The MCP `2026-07-28` material cited below is the protocol's published, versioned
specification. Its subscription design is relevant to current Aicadia protocol
research, but protocol support does not establish how a particular host presents a
notification, keeps a subscription open or invokes a model.

Each section separates:

- **Evidence:** what the source directly establishes;
- **Inference:** what follows for this problem but is not asserted by the source; and
- **Candidate implication:** an unaccepted Aicadia direction.

## Core result

The researched patterns support a sharp split:

| Concern | Intelligent owner | Deterministic owner |
| --- | --- | --- |
| What the occurrence means | Agent, under User direction | none |
| Which actual World facts informed that meaning | Agent declares them | World rechecks them |
| Which facts should change | Agent composes an exact bounded manifest | World validates and writes only that manifest |
| Whether claimed Places form a structurally allowed scope | Agent supplies a scope and, when possible, a witness | World checks generic stored relations, bounds or authority |
| Whether an unverifiable semantic scope is acceptable | eligible Agents may deliberate | World enforces eligibility and the chosen deterministic settlement rule |
| Whether prose is persuasive or physically correct | Agents | never World |
| Whether the request is current, authorized, bounded and atomic | none | World |

The hard limit is equally important:

> If World has no authoritative relation, geometry, rule, capability or collective
> ratification policy against which to check “A and B,” then it cannot verify that
> scope without interpreting the explosion. In that case the list is merely one
> untrusted Agent's assertion.

Collective agreement can make that assertion legitimate game authorship. It does
not turn it into independently proven physical truth. Cryptographic authorization
can prove who may make the assertion. It also does not prove blast semantics.

## 1. Client-composed target data is compatible with server authority

### Evidence

Unreal Engine separates generic target data from authoritative effect application.
`FGameplayAbilityTargetData` can carry Actor references, positions, directions and
origins. Epic lists area-of-effect producers that collect all Actors in a radius and
consumers that apply an effect to every Actor in the target data. The Actor-array
variant carries a source location and multiple target Actors specifically for
area-of-effect use.
[Unreal target data](https://dev.epicgames.com/documentation/en-us/unreal-engine/API/Plugins/GameplayAbilities/FGameplayAbilityTargetData),
[Unreal Actor-array target data](https://dev.epicgames.com/documentation/en-us/unreal-engine/API/Plugins/GameplayAbilities/FGameplayAbilityTargetData_Actor-)

The Ability System exposes `ServerSetReplicatedTargetData` as a reliable server RPC
with validation. More generally, Unreal's server-RPC validation runs before the RPC
implementation and is intended to verify that client parameters satisfy game rules
and constraints. A locally predicted ability may start immediately on a client, but
Epic states that the server has the final word and may override its impact.
[Unreal Ability System target-data RPC](https://dev.epicgames.com/documentation/unreal-engine/API/Plugins/GameplayAbilities/UAbilitySystemComponent),
[Unreal server RPC validation](https://dev.epicgames.com/documentation/unreal-engine/remote-procedure-calls-in-unreal-engine),
[Unreal Gameplay Ability execution policies](https://dev.epicgames.com/documentation/unreal-engine/using-gameplay-abilities-in-unreal-engine)

Unreal's own radial-damage operation takes an origin, radius, damage type, ignored
Actors and an occlusion channel, and applies damage only to locally authoritative
Actors. That function is marked authority-only and uses engine-known radius and
trace semantics.
[Unreal `ApplyRadialDamage`](https://dev.epicgames.com/documentation/en-us/unreal-engine/API/Runtime/Engine/UGameplayStatics/ApplyRadialDamage)

Solana transactions provide a more data-oriented example. A signed transaction
contains an atomic sequence of instructions and an explicit list of every account
used by those instructions, partitioned into signer/non-signer and writable/read-
only groups. Programs define the business logic, only an account's owner program can
modify its data, and the runtime rejects violations such as modifying read-only
accounts. The wire transaction has a hard serialized-size limit.
[Solana transaction structure](https://solana.com/docs/core/transactions/transaction-structure),
[Solana program execution](https://solana.com/docs/core/programs/program-execution),
[Solana writing and account ownership](https://solana.com/docs/intro/quick-start/writing-to-network)

### Inference

An intelligent client can calculate a target set and transmit it without becoming
the authority. The safe pattern is not “trust the client list.” It is:

1. accept a typed, bounded target/effect manifest;
2. authenticate the actor and authorize the operation;
3. validate every declared reference and permission;
4. recheck all relevant current facts;
5. write no subject outside the declared and validated manifest; and
6. commit all accepted consequences atomically or none at all.

Unreal's radial-damage implementation is deliberately not Aicadia's desired
semantic boundary: it encodes radius, damage and occlusion in server gameplay code.
The transferable lesson is target-data separation and authoritative validation, not
server-side blast intelligence.

### Candidate implication

An Aicadia Agent could compose an `occurrence_intent` containing a claimed affected
Place set and exact consequences. World would never parse the accompanying prose to
decide what an explosion does. It would process only typed identities, facts,
relations, limits, authorization and the final consequence manifest.

## 2. A declared scope needs a verifier, authority or ratification

### Evidence

Unreal's server-RPC documentation explicitly calls for checking every client
parameter against server-known rules and constraints. A client-supplied target array
therefore remains input, not authority.
[Unreal server RPC validation](https://dev.epicgames.com/documentation/unreal-engine/remote-procedure-calls-in-unreal-engine)

Ethereum's EIP-2930 shows the danger of confusing a declared access list with an
enforced boundary. Its transaction access list names addresses and storage keys that
execution plans to access, but accesses outside the list are expressly allowed at a
higher gas cost. The list helps preloading and cost accounting; it is not a security
sandbox.
[EIP-2930 optional access lists](https://eips.ethereum.org/EIPS/eip-2930)

PostGIS shows one form of independently checkable spatial evidence. `ST_DWithin`
tests whether two stored geometries lie within a given distance, and `ST_Intersects`
tests whether geometries share a point; both can use spatial indexes. These
predicates only apply when authoritative geometry and a relevant metric or topology
actually exist.
[PostGIS `ST_DWithin`](https://postgis.net/docs/ST_DWithin.html),
[PostGIS `ST_Intersects`](https://postgis.net/docs/en/ST_Intersects.html)

OAuth Rich Authorization Requests offer an authorization analogue. A client may ask
for detailed rights over specified actions, locations and data types. The
authorization server controls the type definitions, must reject unknown or invalid
details and issues only rights permitted by the underlying grant or policy. The
client's requested scope does not grant itself that scope.
[RFC 9396: OAuth 2.0 Rich Authorization Requests](https://www.rfc-editor.org/rfc/rfc9396.html)

Macaroons are cryptographic bearer credentials that carry contextual caveats and can
constrain delegation by circumstances such as when, where, by whom and for what
purpose a service should authorize a request. The target service still verifies the
credential and caveats.
[Macaroons original paper](https://research.google.com/pubs/archive/41892.pdf)

### Inference

There are three defensible meanings for a claimed multi-Place scope:

1. **Structurally witnessed scope.** The Agent supplies a bounded witness over facts
   World already owns, such as current Place adjacency edges, containment relations
   or geometry. World verifies the witness without knowing what an explosion means.
2. **Capability-bounded authorship.** The request carries authority to affect these
   exact subjects under these constraints. This proves permission, not physical
   causality.
3. **Collectively ratified scope.** An eligible assembly accepts the Agent-authored
   scope as the canonical game outcome. This proves that the chosen governance rule
   was satisfied, not that A and B are objectively the only affected Places.

These routes can be combined. For example, a proposal may be limited to the
authoritatively stored neighborhood of Place A and still require ratification by
eligible Characters in A and B.

No checksum, signature or majority can prove that “explosion” semantically entails
exactly A and B unless the corresponding semantic rule is authoritative somewhere.
A proof can establish identity, permission, freshness or a path through stored
relations. It cannot prove unstated fiction.

### Candidate implication

Every multi-subject intent should identify which of these bases it relies on. A
bounded World operation could reject a scope that has neither a valid structural
witness, sufficient delegated authority nor the required collective settlement.
World would validate the selected basis mechanically and would not decide whether
the word “explosion” deserves one basis or another.

Which basis is allowed for which game situation remains a product decision. Letting
the submitting Agent choose an unrestricted “direct” route would make the distinction
meaningless; the admissible route must itself be derived from authoritative context
or explicit prior authority.

## 3. The explosion example, end to end

The following is a candidate analysis, not a wire contract.

### Agent-authored material

After inspecting World state and reasoning with its User, the initiating Agent might
compose:

```text
intent identity
  request_id

claimed occurrence
  prose: "The charge detonates under the eastern arch ..."
  origin: Entity or Place reference already knowable to the Character

claimed scope
  affected_places: [A, B]
  basis:
    current explicit relation A --adjacent-to--> B
    expected relation revision or exact relation facts

semantic dependencies actually used
  exact Property facts, exact Trait versions, placements or relations
  expected absence where absence mattered

exact consequences
  bounded typed changes for the named Entities and Places
  no generic instruction for World to infer additional damage
```

The Agent may use `wall_condition`, an existing Trait statement, an explicit Place
relation or no Property at all. Nothing requires a tree to have `form`, `exists`,
`shape` or another universal lifecycle key.

### What World can validate without blast intelligence

World can deterministically verify:

- the User, Character and request identity;
- that every referenced Entity, Place, Property key, Trait version and relation is
  valid and available under the operation's privacy rules;
- that the claimed list, witness, dependency list and consequence list stay within
  hard size and cost bounds;
- that A and B are exactly the Places named in the request;
- that the supplied A-to-B relation witness is current and satisfies a generic
  allowed relation/hop constraint, if such a generic constraint is authoritative;
- that every expected actual fact still has its expected value or revision;
- that every expected-absent fact remains absent;
- that the actor has authority for every requested write, or that the required
  collective settlement has succeeded;
- that the write manifest contains no duplicate or forbidden subject and that the
  operation will not mutate anything outside it;
- that every exact write target and dependency is rechecked in the same transaction;
  and
- that current state, Activity and any accepted settlement result commit atomically.

### What World cannot validate under the dumb-server rule

Without adding a specific authoritative rule, World cannot determine:

- whether this occurrence counts as an explosion;
- whether an explosion of this fictional size should reach B;
- whether C should also have been included;
- whether changing a given Property is “destructive”;
- whether a tree is standing, felled, alive or destroyed when those meanings exist
  only across arbitrary Agent-authored content; or
- whether the participating Agents reasoned well, were sincere or reached a wise
  conclusion.

### Unavoidable trust fork

Suppose A and B are adjacent, C is also adjacent, and the Agent names only A and B.
A generic adjacency witness proves that B is within an allowed structural envelope.
It does not prove completeness; C may also satisfy the envelope.

World has only four honest choices:

1. derive the complete candidate set itself from a generic bounded relation query;
2. require a witness that proves both inclusion and completeness under a generic
   World-known predicate;
3. treat the Agent list as authorized or collectively ratified authorship; or
4. reject or narrow the operation because scope cannot be justified.

Calling the list “smart” does not create a fifth choice.

## 4. Dynamic semantic dependencies do not require universal Properties

### Evidence

etcd transactions are atomic if/then/else operations guarded by comparisons on one
or several exact keys. A comparison can test a key's value, version, creation
revision or modification revision. All comparisons are applied atomically before
the success or failure block.
[etcd v3 transaction API](https://etcd.io/docs/v3.6/learning/api/)

DynamoDB condition expressions can test comparisons and can explicitly distinguish
`attribute_exists(path)` from `attribute_not_exists(path)`. An absent attribute is
therefore a checkable precondition; applications do not need to encode absence as a
mandatory `exists = false` value.
[DynamoDB condition expressions](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html),
[DynamoDB `ConditionCheck`](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_ConditionCheck.html)

FoundationDB tracks the exact key ranges a transaction reads and writes. Concurrent
transactions conflict when a prior committed write intersects a later transaction's
read conflict range. Applications may explicitly add a narrow conflict key or range
when that is the invariant they need. FoundationDB's own example uses a broad
snapshot read followed by an explicit conflict only on the selected key.
[FoundationDB conflict ranges](https://apple.github.io/foundationdb/developer-guide.html#conflict-ranges),
[FoundationDB special conflict keys](https://apple.github.io/foundationdb/special-keys.html)

HTTP `If-Match` provides a representation-scoped precondition specifically useful
for preventing lost updates. It does not require a global resource revision.
[RFC 9110 conditional requests](https://www.rfc-editor.org/rfc/rfc9110.html#section-13)

### Inference

The Agent can choose semantic dependencies dynamically from facts that actually
exist in the situation. For painting a tree, it might depend only on the tree's
current `colour`; it might additionally depend on one actual placement relation; or
it might decide that a particular current Trait statement matters. Another Entity
may expose a different set of meaningful facts.

Absence is itself a legitimate expected fact:

```text
expect Property (tree, colour) is absent
then establish Property (tree, colour) = blue
```

This does not assert that the tree “has an exists field.” It identifies the natural
fact slot `(Entity, Property key)` and expects no current value in that slot.

The World can guarantee consistency relative to declared dependencies. It cannot
guarantee that an untrusted Agent declared every semantic dependency it should have
considered. Exact write targets must therefore conflict regardless of which read
dependencies the Agent volunteers. Discussion may expose omitted dependencies, but
that remains semantic review rather than deterministic proof.

### Candidate implication

A future intent could carry two distinct bounded sets:

- **expected facts:** actual values/versions or explicit expected absence selected by
  the Agent because its reasoning depends on them; and
- **write facts:** exact natural Property slots, Trait lineages, placements or
  structural relations the final package will change.

World would always protect the write facts and would additionally reject if an
expected fact changed. No generic `Tree` schema or mandatory lifecycle vocabulary is
introduced.

Expected absence still needs a stable concurrency identity in storage. One candidate
is an internal slot keyed by the natural identity `(entity_id, property_key)` even
when no current value exists. That internal mechanism must not be exposed as a
fictional `exists`, `form` or `shape` Property. This report does not select its schema.

## 5. Deliberation is a protocol; settlement is a separate authority rule

### Evidence

The FIPA Propose protocol gives one useful minimum choreography: an initiator
describes an action it will perform if a participant agrees; the participant accepts
or rejects; the conversation has a globally unique conversation id.
[FIPA Propose Interaction Protocol](https://www.fipa.org/specs/fipa00036/SC00036H.html)

The FIPA Contract Net protocol provides a bounded multi-agent proposal round. An
initiator issues a call for proposals with task conditions; any number of
participants propose while others refuse; after the deadline the initiator selects
one, several or no proposals. Late proposals are automatically rejected. Every
message carries the conversation id. FIPA also states that the pattern does not by
itself address real-world asynchrony, abnormal termination or nested protocols.
[FIPA Contract Net Interaction Protocol](https://www.fipa.org/specs/fipa00029/SC00029H.html)

FIPA communicative acts distinguish the message's role from its domain content. A
`propose` act submits an action subject to preconditions; the surrounding protocol
is known by prior agreement or named explicitly.
[FIPA Communicative Act Library](https://www.fipa.org/specs/fipa00037/SC00037J.html#_Toc26729693)

### Inference

These standards show that intelligent Agents can retain freedom over how they
produce a proposal while following a small shared external state machine. They do
not supply Aicadia's answer to:

- who is eligible;
- whether the initiator, a majority, unanimity or another rule selects the result;
- whether non-response counts;
- how amendments replace earlier proposals;
- which result is authorized to mutate World state; or
- how concurrency with later World facts is resolved.

In particular, Contract Net is not voting: the initiator chooses after a deadline.
Its useful pattern is the explicit round, deadline, typed response and conversation
identity.

### Candidate implication

A collective World-change mechanic could separate four phases:

```text
proposal    one bounded Agent-authored scope and consequence package
deliberate  eligible Agents read, challenge and submit bounded alternatives
seal        no new alternatives; eligible explicit responses are accepted until deadline
settle      World applies one public deterministic rule, then rechecks current facts
```

Agents may write free-form arguments for other Agents, but the settlement input must
be a typed proposal identity, amendment, response or final mutation package. World
cannot resolve natural-language debate while remaining dumb.

A new alternative should be a complete bounded package or a typed patch against one
package, not prose that World must merge. If the accepted package is stale at final
commit, World should reject or reopen according to an explicit rule; it must not ask
an LLM to rebase it.

Whether any form of voting is compatible with Aicadia's current no-score rule is a
separate product decision. Research does not make that choice.

## 6. MCP subscriptions can signal a board; they do not implement one

### Evidence

The published MCP `2026-07-28` specification defines `subscriptions/listen` as a
client-opened, long-lived notification stream. The client declares exactly which
notification types and resource URIs it wants, and the server must not send
unrequested types. Resource updates carry a URI, not the resource's new contents.
The client must re-establish subscriptions after a reconnect; the server retains no
subscription state across reconnections.
[MCP `2026-07-28` subscriptions](https://modelcontextprotocol.io/specification/2026-07-28/basic/patterns/subscriptions)

The versioned resource specification says the host application decides how resources
enter model context; MCP does not mandate a user-interaction model. It also requires
servers to validate resource URIs and check permissions. Resource update
notifications identify that a watched resource changed.
[MCP `2026-07-28` resources](https://modelcontextprotocol.io/specification/2026-07-28/server/resources)

The older FIPA Subscribe protocol similarly distinguishes subscription from
decision-making. An initiator subscribes to referenced objects; the participant may
agree or refuse and sends new results as those objects change. The specification
notes that ordered update semantics require an ordered transport.
[FIPA Subscribe Interaction Protocol](https://www.fipa.org/specs/fipa00035/SC00035H.html)

### Inference

MCP subscriptions can implement a low-latency hint such as “proposal resource X
changed.” They do not provide:

- durable catch-up after disconnection;
- automatic model invocation;
- participant eligibility;
- proposal, discussion or voting semantics;
- exactly-once response submission; or
- authoritative settlement.

Those are application-domain responsibilities. Treating a transport listener as the
consensus system would conflate transient connection state with durable World truth.

A server-sent notification also must not silently spend a User's tokens. An active
host may surface the hint, but only an explicit User/Agent call should read, reason
and respond under Aicadia's current operating principle.

### Candidate implication

A durable proposal board could be exposed through ordinary bounded World reads and
writes. MCP subscriptions could optionally notify an already active client that the
board or Place inbox resource changed. Authoritative reads remain the recovery path.

An Entity listener alone is insufficient for the explosion. If the proposal claims
that Places A and B are affected, relevant participants may be attached to A, B or a
structurally derived neighborhood rather than to the originating Entity. The durable
proposal scope must first exist; transport subscriptions can then hint to clients
whose authorized resource view includes that scope.

## 7. Area of interest selects recipients, not truth

### Evidence

Unreal's Iris filtering system determines which replicated objects go to which
network connections. It supports owner, connection, group and dynamic filters. One
provided dynamic grid filter divides a world into cells and sends objects in cells
near a player's view. Epic describes filtering as a bandwidth and CPU optimization
over authoritative server state.
[Unreal Iris filtering](https://dev.epicgames.com/documentation/en-us/unreal-engine/iris-filtering-in-unreal-engine),
[Unreal Iris authoritative server model](https://dev.epicgames.com/documentation/unreal-engine/introduction-to-iris-in-unreal-engine)

### Inference

Area of interest answers “which connected clients might need an update?” It does not
answer “which Places did the explosion affect?” or “who has governance legitimacy?”
Those sets may overlap but must not be silently equated.

For Aicadia, at least four scopes may differ:

| Scope | Question |
| --- | --- |
| causal claim | Which Places and Entities does the proposing Agent say are affected? |
| structural envelope | Which claimed subjects can World verify are within generic current bounds? |
| electorate | Which Characters are eligible to contribute to this decision? |
| notification interest | Which active clients should receive a lossy update hint? |

If the Agent supplies all four sets, it can select supporters and suppress
opponents. If World derives all four from one blunt Entity listener, a large Place
or cross-Place occurrence becomes either incomplete or a fan-out hotspot.

### Candidate implication

World could deterministically derive the eligible and notify-able candidate sets
from authoritative Character placement plus the proposal's admitted Place scope.
The Agent still authors why A and B matter and the desired consequences. World only
applies a generic membership relation to an already admitted scope.

Eligibility should be fixed by an explicit time or revision rule. Recomputing it
continuously lets Characters enter or leave to manipulate a round; letting the Agent
name it makes the vote self-authorizing. The exact snapshot policy is unchosen.

Notification fan-out must remain bounded or degradable. One durable proposal plus
optional change hints is cheaper than one durable notification row per observer, but
it cannot make delivery to a million active clients free.

## 8. Optimistic concurrency makes declared dependencies executable

### Evidence

FoundationDB uses optimistic concurrency: transactions do not block on reads or
writes and are rejected at commit when conflict ranges overlap relevant committed
writes. Clients ordinarily retry rejected transactions. Narrow conflict ranges let
independent keys proceed while preserving chosen invariants.
[FoundationDB transaction processing](https://apple.github.io/foundationdb/transaction-processing.html),
[FoundationDB conflict ranges](https://apple.github.io/foundationdb/developer-guide.html#conflict-ranges)

PostgreSQL row locks block writers or lockers of the same row but not ordinary
readers. Its documentation warns against holding transactions open while waiting for
user input and recommends acquiring multiple object locks in a consistent order to
avoid deadlocks.
[PostgreSQL explicit locking](https://www.postgresql.org/docs/current/explicit-locking.html)

Solana can process transactions in parallel when they do not write the same account.
Its first-party architecture guidance calls a shared account a concurrency
bottleneck: writes to the same account must proceed sequentially. The runtime also
charges scheduler cost per writable-account lock and imposes transaction compute and
data bounds.
[Solana concurrency and shared accounts](https://solana.com/developers/courses/program-optimization/program-architecture),
[Solana compute and write-lock budget](https://solana.com/docs/core/fees/compute-budget)

### Inference

Agent deliberation may turn many contradictory immediate writes into one selected
final write, but it does not remove the serialization point. Two settlements that
write the same exact fact still need a deterministic order or one must fail.

The expensive reasoning window must occur outside any database transaction. At
settlement, one short transaction should recheck:

- the admitted proposal revision;
- the exact Agent-selected expected facts;
- every exact write target;
- any eligibility/settlement facts; and
- any cooldown or stability precondition.

Multi-Place application requires a deterministic lock or conflict-key order. An
unbounded affected list is unsafe even if all Places are legitimate: transaction
cost, lock count, retry work and deadlock surface grow with the list.

### Candidate implication

The concurrency unit should be the smallest exact fact that must not admit
incompatible writes, not an entire Entity by default. A request affecting
`(tree, colour)` and another affecting an unrelated actual Property may proceed
independently unless an Agent explicitly declares the other fact as a dependency.

A collective round can be indexed by its declared write facts and admitted scope.
Overlapping proposals may be shown together for semantic review, while World still
uses exact write conflicts at final commit. Because an Agent can omit a read
dependency, only write-set overlap is mechanically complete.

A post-settlement cooldown can reduce repeated oscillation but is not a database
lock and does not settle simultaneous contenders. A candidate form is an
authoritative `protected_until` attached to the exact accepted fact or settlement
result and checked using database time. Applying it to an entire Entity would block
independent facts. The duration, bypass authority and game meaning remain open.

The discussion board can itself become hot. Append-only bounded responses keyed by
participant avoid every response updating one tally row, but settlement still needs
to read or aggregate a bounded set. Unlimited global voting is therefore a distinct
scale problem, not a free extension of local discussion.

## 9. Voting needs authoritative identity and eligibility

### Evidence

Douceur's original Sybil paper shows that redundancy fails when one hostile entity
can present multiple identities. Without a logically centralized identity authority,
Sybil attacks remain possible except under extreme assumptions.
[The Sybil Attack](https://www.microsoft.com/en-us/research/publication/the-sybil-attack/)

FIPA protocols identify conversations and message roles, but do not certify that
different Agent identifiers correspond to different humans or independent
interests. Contract Net also delegates selection to its initiator rather than
claiming that proposal count establishes truth.
[FIPA Contract Net Interaction Protocol](https://www.fipa.org/specs/fipa00029/SC00029H.html)

### Inference

“Many Agents agree” is meaningful only if World already knows which durable
participants are eligible and how many responses each may contribute. Agent process
identity, connection identity and model identity are inadequate voting identities:
one User can reconnect, run several models or resend the same response.

Even with one eligible response per durable User or Character, a vote selects a game
outcome under a policy. It does not verify causal truth. Nearby participants may be
wrong, coordinated or self-interested. That may be excellent gameplay, but it must
be named accurately.

### Candidate implication

If collective settlement is explored, World—not an Agent—must mechanically derive:

- the durable voting subject;
- eligibility from authoritative current or snapshotted placement/role facts;
- one-response identity and idempotency;
- proposal and response deadlines;
- amendment replacement rules;
- quorum, tie and non-response behavior; and
- the exact final package authorized by the result.

Agents remain responsible for arguments, alternatives, compromise and choosing a
response for their User. World remains responsible for counting only valid typed
responses according to the accepted rule. Authentication strong enough to make one
User identity meaningful is a separate prerequisite; the current local development
header is not such proof.

## 10. Candidate high-level architecture

The evidence supports one possible architecture for later prototyping. It is not a
recommendation to add all of these concepts at once.

### A. Semantic authoring plane

One explicitly invoked Agent:

1. reads bounded current World facts;
2. reasons privately with its User;
3. chooses actual facts and expected absence on which its result depends;
4. composes an exact claimed scope and consequence manifest;
5. explains the meaning in prose for Users and other Agents; and
6. chooses or requests an available proposal path.

No mandatory Entity lifecycle keys exist. Agents use the facts each subject really
has.

### B. Deterministic admission plane

World:

1. authenticates context and resolves the Character;
2. enforces byte, item, target, dependency, hop and time bounds;
3. validates each identifier and typed value;
4. verifies authority or capability constraints;
5. validates a structural scope witness when supplied;
6. determines whether direct application or collective settlement is allowed by an
   authoritative rule; and
7. stores at most one bounded admitted proposal identity for the relevant current
   revision.

Admission means “well-formed and allowed to be considered,” not “true” or “already
accepted as World state.”

### C. Agent deliberation plane

Eligible Users may explicitly invoke their Agents to:

- inspect the proposal and cited current facts;
- submit a bounded objection or alternative manifest;
- add missing semantic dependencies;
- accept, reject or select a typed alternative; and
- explain the choice to their User.

MCP resource subscriptions may provide active-client update hints. Ordinary reads
remain authoritative and recoverable.

### D. Deterministic settlement plane

After the deadline, World:

1. closes the response set according to database time;
2. validates one response per eligible durable subject;
3. applies the predefined selection rule;
4. identifies one complete final manifest or no result;
5. starts one short transaction;
6. locks or conflict-checks exact facts in stable order;
7. rechecks all expected facts, writes and structural bases;
8. atomically writes current state and durable Activity if accepted; and
9. optionally stores an exact-fact stability deadline.

No Agent runs during settlement. No prose is interpreted. A stale final manifest is
never silently repaired.

## Failure and abuse analysis

| Failure or abuse | What a dumb strict World can do | What it cannot do |
| --- | --- | --- |
| Agent names 50,000 Places | enforce a small hard target bound | infer which Places were narratively intended |
| Agent names A/B but omits equally adjacent C | verify A/B are within a generic envelope; require completeness witness or ratification | know that C belongs without a complete generic predicate |
| Agent omits a semantic dependency | protect exact writes and declared dependencies; expose proposal to review | know what the Agent should have reasoned about |
| Two final manifests write the same fact | serialize, compare-and-swap or reject one | make both incompatible values simultaneously canonical |
| Two requests touch different facts on one Entity | allow independent exact-fact commits when no dependency overlaps | know they are semantically incompatible if no Agent declares it |
| Agent self-labels a change non-destructive | ignore the label for authority | infer destructiveness from prose or arbitrary keys |
| One User submits many Agent messages | deduplicate by authoritative User/Character and proposal identity | prove human uniqueness without an identity boundary |
| Clients disconnect during discussion | preserve proposal state for later bounded reads | rely on MCP subscription state for durable truth |
| Million-participant round | cap, partition or refuse the mechanic | make a million authenticated responses costless |
| Repeated accepted oscillation | apply a precise stored cooldown policy | use cooldown as a substitute for concurrency control |

## Research conclusions

### Supported by evidence

- Intelligent clients can submit explicit targets, read/write subjects and
  preconditions while a deterministic authority retains the final word.
- Exact client-declared dependency sets are a proven optimistic-concurrency pattern.
- Expected absence can be checked directly; it need not be encoded as a universal
  lifecycle Property.
- Multi-subject writes can be atomic when their target and dependency sets are hard
  bounded.
- Agent communication standards support proposal, rejection, deadline,
  conversation-id and subscription patterns.
- MCP subscriptions are suitable for active-client change hints, not durable domain
  consensus.
- Interest management can bound notification relevance but does not determine
  semantic effect or voting authority.
- Capability constraints can prove bounded permission, not causal meaning.
- Hot exact facts inevitably serialize or conflict at final commitment.
- Voting requires authoritative participant identity and eligibility to resist
  duplicate or Sybil influence.

### Inferred design constraints

- A claimed spatial or causal scope needs one explicit standing: structurally
  witnessed, capability-authorized, collectively ratified or rejected/narrowed.
- A generic structural witness can validate reach through stored relations without
  teaching World what an explosion means.
- Inclusion is easier to prove than completeness. The World can verify A and B are
  allowed yet still be unable to prove that C was correctly omitted.
- Agents can improve semantic concurrency by declaring dependencies and negotiating
  a final package, but exact writes remain mechanically protected by World.
- Discussion must finish before the short commit transaction; no database lock may
  remain open while Agents or Users deliberate.
- A cooldown should protect the smallest accepted fact or settlement result, not an
  Entity merely because the Entity hosted the discussion.

### Open product and research questions

- Which current World facts can form a useful generic Place-scope witness: explicit
  adjacency, containment, a bounded neighborhood, optional geometry, or something
  else?
- Must a witnessed scope be complete under its predicate, or is inclusion plus
  collective ratification enough?
- Which subjects, if any, require collective settlement rather than direct Agent-
  authored action?
- Who is eligible: current co-presence, a proposal-time snapshot, affected-Place
  membership, ownership or another explicit relation?
- Does a collective result use consent, selection, veto or another rule, and how
  does that coexist with the no-score constitution?
- Are discussion messages durable World history, temporary bounded coordination
  data or external Agent communication?
- What is the smallest exact-fact concurrency representation that also protects
  expected absence without creating fictional lifecycle fields?
- What bounded prototype can test whether Agent-proposed alternatives create fun
  and coherent gameplay rather than latency, token burn and griefing?

## Primary sources

### Client-composed targets and authority

- [Unreal `FGameplayAbilityTargetData`](https://dev.epicgames.com/documentation/en-us/unreal-engine/API/Plugins/GameplayAbilities/FGameplayAbilityTargetData)
- [Unreal `FGameplayAbilityTargetData_ActorArray`](https://dev.epicgames.com/documentation/en-us/unreal-engine/API/Plugins/GameplayAbilities/FGameplayAbilityTargetData_Actor-)
- [Unreal Ability System target-data RPC](https://dev.epicgames.com/documentation/unreal-engine/API/Plugins/GameplayAbilities/UAbilitySystemComponent)
- [Unreal server RPC validation](https://dev.epicgames.com/documentation/unreal-engine/remote-procedure-calls-in-unreal-engine)
- [Unreal Gameplay Ability execution policies](https://dev.epicgames.com/documentation/unreal-engine/using-gameplay-abilities-in-unreal-engine)
- [Unreal `ApplyRadialDamage`](https://dev.epicgames.com/documentation/en-us/unreal-engine/API/Runtime/Engine/UGameplayStatics/ApplyRadialDamage)
- [Solana transaction structure](https://solana.com/docs/core/transactions/transaction-structure)
- [Solana program execution](https://solana.com/docs/core/programs/program-execution)
- [EIP-2930 optional access lists](https://eips.ethereum.org/EIPS/eip-2930)

### Preconditions, concurrency and hot subjects

- [etcd v3 API](https://etcd.io/docs/v3.6/learning/api/)
- [DynamoDB condition expressions](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html)
- [FoundationDB conflict ranges](https://apple.github.io/foundationdb/developer-guide.html#conflict-ranges)
- [FoundationDB transaction processing](https://apple.github.io/foundationdb/transaction-processing.html)
- [PostgreSQL explicit locking](https://www.postgresql.org/docs/current/explicit-locking.html)
- [RFC 9110 conditional requests](https://www.rfc-editor.org/rfc/rfc9110.html#section-13)
- [Solana concurrency and shared accounts](https://solana.com/developers/courses/program-optimization/program-architecture)
- [Solana compute budget](https://solana.com/docs/core/fees/compute-budget)

### Spatial verification and interest

- [PostGIS `ST_DWithin`](https://postgis.net/docs/ST_DWithin.html)
- [PostGIS `ST_Intersects`](https://postgis.net/docs/en/ST_Intersects.html)
- [Unreal Iris filtering](https://dev.epicgames.com/documentation/en-us/unreal-engine/iris-filtering-in-unreal-engine)
- [Unreal Iris server model](https://dev.epicgames.com/documentation/unreal-engine/introduction-to-iris-in-unreal-engine)

### Agent interaction and subscriptions

- [FIPA Propose Interaction Protocol](https://www.fipa.org/specs/fipa00036/SC00036H.html)
- [FIPA Contract Net Interaction Protocol](https://www.fipa.org/specs/fipa00029/SC00029H.html)
- [FIPA Subscribe Interaction Protocol](https://www.fipa.org/specs/fipa00035/SC00035H.html)
- [FIPA Communicative Act Library](https://www.fipa.org/specs/fipa00037/SC00037J.html)
- [MCP `2026-07-28` subscriptions](https://modelcontextprotocol.io/specification/2026-07-28/basic/patterns/subscriptions)
- [MCP `2026-07-28` resources](https://modelcontextprotocol.io/specification/2026-07-28/server/resources)

### Authorization and identity limits

- [RFC 9396: OAuth 2.0 Rich Authorization Requests](https://www.rfc-editor.org/rfc/rfc9396.html)
- [Macaroons original paper](https://research.google.com/pubs/archive/41892.pdf)
- [The Sybil Attack](https://www.microsoft.com/en-us/research/publication/the-sybil-attack/)
