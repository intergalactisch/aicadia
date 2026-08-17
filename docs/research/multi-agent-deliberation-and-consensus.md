---
status: pending
era: August Activity-Property-Trait
---

# Multi-Agent deliberation and consensus

> **Role / side:** sourced research report / development side.
> **Authority:** records this report's question, evidence, inferences and candidate implications.
> **Excludes:** product decisions and current implementation contracts; see `docs/game/`.

Date: 2026-08-16

Status: research; no mechanic, protocol, domain type or architecture below is
accepted Aicadia behavior

## Question and evidence boundary

Can User-controlled Agents form a fast, local deliberative body that proposes,
challenges, combines or selects a semantic World change while the World remains
deliberately dumb and strict?

The concrete motivating cases are not limited to choosing a color:

- an Agent proposes that an explosion affects Places A and B and assembles the
  exact requested consequences across both;
- other eligible Agents independently notice an omitted subject, reject a causal
  assumption, submit an alternative package or support one immutable candidate;
- a tree has only the Properties that players have actually established, not a
  mandatory universal `.form`; Agents must inspect and reason over the facts that
  exist rather than rely on a server ontology; and
- after a bounded deadline, the World validates identity, authority, bounds,
  revisions and structural legality, applies one exact package atomically or does
  nothing, records history and enforces any accepted mechanical cooldown.

This report investigates four bodies of primary evidence:

1. empirical LLM debate and collective-decision research;
2. social-choice, Sybil and Byzantine limits on voting and agreement;
3. adversarial communication, prompt-injection and collusion research; and
4. message-complexity and current MCP subscription mechanics.

Most LLM-debate experiments ask several cooperative model instances to solve a
question with a known answer. Aicadia instead has separately controlled Agents,
heterogeneous User preferences, adversarial incentives, mutable shared state and
often no objectively correct creative outcome. Results do not transfer without
that qualification.

**Evidence** below means a sourced observation or theorem. **Inference** means an
analysis of that evidence for this question. **Candidate implication** means an
unaccepted Aicadia direction. The separation is intentional: research can constrain
a product choice but cannot make it.

## Core result

Collective Agents are a credible semantic **intent-assembly** mechanism. They are
not a credible replacement for authoritative settlement.

The useful separation is:

| Layer | Concrete responsibility | What it cannot establish |
| --- | --- | --- |
| notification | tell an interested client that one proposal or relevant resource changed | truth, eligibility, a vote or an Agent run |
| deliberation | generate, critique and amend possible consequences | authority or a committed World result |
| aggregation | apply a declared rule to immutable proposals or ballots | semantic correctness or database atomicity |
| settlement | revalidate the exact package against current state and commit or reject it | invent missing semantic consequences |
| cooldown | reject a later conflicting attempt until a stored time | whether two differently named facts mean the same thing |

**Evidence.** Du et al.'s original multi-Agent debate experiments found better
performance than their single-model baselines on six reasoning, factuality and QA
tasks. They also explicitly state that convergence is not guaranteed in general and
show examples that converge to a wrong value. More Agents and rounds improved the
reported arithmetic setting, but this was an empirical result for one prompt and
model family, not a consensus proof.
[Du et al., *Improving Factuality and Reasoning in Language Models through Multiagent Debate*](https://arxiv.org/abs/2305.14325)

**Evidence.** Later controlled work makes the limits clearer. Kaesberg et al.
compared seven decision protocols while holding other debate variables fixed.
Voting protocols did 13.2% better than the compared protocols on their reasoning
tasks, while consensus protocols did 2.8% better on their knowledge tasks. More
Agents helped in their setup, but more discussion rounds before voting hurt.
[Kaesberg et al., *Voting or Consensus? Decision-Making in Multi-Agent Debate*](https://aclanthology.org/2025.findings-acl.606/)

**Evidence.** Becker et al. ran three-Agent, seven-turn debates across ten tasks.
The Agents first agreed within two turns in 99% of cases, yet prolonged discussion
could move away from a better earlier answer. They measured problem drift in 76–89%
of generative-task cases and 7–21% of high-complexity-task cases; human analysis
attributed the largest categories to lack of progress, low-quality feedback and
lack of clarity.
[Becker et al., *Stay Focused: Problem Drift in Multi-Agent Debate*](https://aclanthology.org/2026.findings-eacl.268/)

**Evidence.** Zhu et al. report that vanilla multi-Agent debate often costs more
and performs worse than a simple initial majority vote. Their theoretical model
shows that homogeneous Agents with uniform belief updates preserve expected
correctness rather than systematically improve it. Their interventions work by
increasing initial diversity and communicating calibrated confidence, under the
paper's stated assumptions.
[Zhu et al., *Demystifying Multi-Agent Debate: The Role of Confidence and Diversity*](https://aclanthology.org/2026.findings-acl.1694/)

**Inference.** Fast agreement is not evidence of a good result. Agreement can
represent successful error correction, conformity, persuasion, shared model error
or exhaustion of the allotted turns. A World mutation therefore cannot become
valid merely because Agents say they reached consensus.

**Candidate implication.** Use Agents for the part only they can do: reading the
actual World, interpreting a User's intent, discovering semantic dependencies,
constructing alternatives and explaining trade-offs. Require the deliberation to
end in one immutable, machine-readable candidate package. The World can then
strictly validate and settle that package without understanding why an explosion
should cross a Place boundary or why chopping a tree is more consequential than
painting it.

## Four different meanings of consensus

The word `consensus` hides materially different claims:

| Meaning | Example | Guarantee |
| --- | --- | --- |
| conversational convergence | all Agents end their messages with the same proposal | only a sampled language outcome |
| social choice | a voting rule selects proposal P from proposals P, Q and R | the declared aggregation result, subject to identity and ballot integrity |
| distributed consensus | non-faulty processes agree on one value under a stated network and fault model | a theorem only within those assumptions |
| World settlement | one current-state transition and its history commit atomically | authoritative Aicadia state, if the transaction succeeds |

**Evidence.** Fischer, Lynch and Paterson prove that in their fully asynchronous
message-passing model, every deterministic consensus protocol can have a
non-terminating execution with even one crash-faulty process. The result concerns a
specific distributed-computing model; it does not say that a database-backed
authoritative service cannot choose an outcome by deadline.
[Fischer, Lynch and Paterson, *Impossibility of Distributed Consensus with One Faulty Process*](https://groups.csail.mit.edu/tds/papers/Lynch/jacm85.pdf)

**Evidence.** Lamport, Shostak and Pease show that Byzantine agreement using only
unauthenticated oral messages requires more than two-thirds of the participants to
be loyal; unforgeable signed messages materially change the solvable fault model.
This is agreement between named processors, not evidence that a persuasive LLM
majority is truthful.
[Lamport, Shostak and Pease, *The Byzantine Generals Problem*](https://www.microsoft.com/en-us/research/?p=338450)

**Inference.** Aicadia need not turn Agents into a replicated consensus cluster.
The World is already the authoritative state processor. Agents can disagree,
timeout or fail; the World still has one deterministic rule for `settle`, `stale`,
`expired`, `conflict` or `no outcome`. This is both simpler and a stronger boundary
than asking an LLM judge to decide whether the discussion was good enough.

## What deliberation can add

### Independent proposals can expose missing consequences

**Evidence.** The positive debate results depend on retaining more than one initial
hypothesis long enough to compare it. Zhu et al. identify initial diversity as one
of the two missing mechanisms in homogeneous vanilla debate. Kaesberg et al.'s
All-Agents Drafting and Collective Improvement variants improved their task results
by up to 3.3% and 7.4% respectively, although those benchmark gains do not establish
a general mechanism.
[Zhu et al.](https://aclanthology.org/2026.findings-acl.1694/),
[Kaesberg et al.](https://aclanthology.org/2025.findings-acl.606/)

**Evidence.** Human-group evidence is directionally compatible but not a direct LLM
result. Woolley et al. studied 699 people in groups of two to five and found a group
performance factor associated with social sensitivity and more equal conversational
turn-taking, not strongly with the average or maximum individual intelligence.
Bates and Gupta's three later studies with 312 people instead attributed about 80%
of group-IQ differences to individual IQ and did not support the turn-taking result.
The safe observation is therefore only that group composition and process can
matter in small human groups; the causal account remains disputed.
[Woolley et al., *Evidence for a Collective Intelligence Factor in the Performance of Human Groups*](https://pubmed.ncbi.nlm.nih.gov/20929725/),
[Bates and Gupta, *Smart groups of smart people*](https://www.research.ed.ac.uk/files/30119288/BatesGruptaI2016SmartGroupsOfSmartPeople.pdf)

**Inference.** Showing every participant the first confident proposal immediately
can erase the independence that made multiple participants useful. A discussion
phase should not be mistaken for independent sampling.

**Candidate implication.** A bounded initial proposal phase could be sealed:
eligible Agents submit one short candidate or abstain before seeing peers' text.
Only then does a bounded critique/amendment phase open. This makes an omitted Place
B or an overlooked Entity more likely to appear before conformity sets in. It is a
candidate game rule, not a requirement inferred from the papers.

### Deliberation can assemble a package, not merely elect a sentence

**Inference.** A proposal such as “the explosion reaches A and B” is insufficient
for settlement. Agents must turn it into exact requested operations over the World
facts they inspected. Conversely, making the World infer those operations from the
sentence would put semantic intelligence and hidden ontology back on the server.

**Candidate implication.** One Agent-authored candidate package could contain:

- immutable references to the initiating Character, involved Entities and declared
  affected Places;
- the exact current revisions or fact versions the Agent read;
- the exact bounded writes or structural operations requested;
- explicit preconditions and dependencies discovered by the Agent;
- a bounded natural-language rationale for other Agents, kept non-authoritative;
  and
- an expiry plus immutable candidate identity so ballots cannot silently follow a
  later edit.

Another Agent can challenge the declared affected set, point to a current fact that
was omitted, or submit a replacement package. If Agents want to combine proposals,
an Agent must materialize the merged result as a new exact candidate. The World
must never merge prose or infer a compromise.

### The explosion example

The following is a candidate decomposition, not accepted design:

1. An explicitly called Agent inspects the explosive Entity, origin, nearby World
   facts and Places A and B.
2. It publishes an immutable draft saying which facts it read and which exact
   changes it proposes in both Places.
3. An opt-in change hint reaches clients interested in the origin Entity and the
   declared Places. A hint does not run their Agents.
4. Explicitly called eligible Agents independently submit “support”, a bounded
   critique or a complete alternative package. One may add a missed Entity in B;
   another may argue that B is unaffected.
5. A fixed, predeclared procedure selects one immutable package, or none, by the
   deadline.
6. The World rechecks the selected package's identities, limits, authorization,
   referenced subjects, read versions and write preconditions. It atomically
   applies the exact package and history, or rejects it as stale/conflicting.

**Inference.** The first Agent's claim that the explosion reaches A and B remains a
semantic assertion, not a World fact until settlement. The World can verify that A
and B exist and that the call has legal structure; it cannot verify that the blast
radius “really” includes them without becoming the semantic simulation engine that
this direction explicitly rejects.

**Inference.** Completeness cannot be guaranteed by transport. If the initiating
Agent omits Place C from both its request and notification scope, Agents listening
only at C may never see the proposal. Broader origin- or area-level discovery can
reduce the blind spot, but only an Agent can infer that C is semantically affected.
No listener protocol solves an omitted semantic dependency by itself.

### The tree-without-`.form` example

**Inference.** A universal tree schema would move world meaning into the server and
create ceremony every time an Entity lacks the expected field. A dumb World should
not need to know that `form`, `shape`, `exists`, a relation or some future Property
expresses “felled”. It can know only the concrete stored facts and legal generic
operations.

**Candidate implication.** The proposing Agent reads the actual tree and names the
exact facts it consulted and wants to change. It may propose changing an existing
`shape`, adding or changing an `exists` fact, changing another established
Property, or using an accepted generic structural operation. Other Agents can
reject that representation and submit a more coherent package. There is no required
`.form` and no server-side “destructive” classifier.

**Hard boundary.** If one Agent writes `exists = false` and another writes an
unrelated `shape`, the World cannot know they are semantic substitutes unless the
requests declare that dependency or an accepted mechanical rule makes them share a
conflict scope. Agent intelligence can improve the declaration; it cannot make an
undeclared dependency mechanically enforceable.

## Accuracy, convergence and correlated-model limits

### More copies do not necessarily create independent judgment

**Evidence.** Kim et al. evaluated error correlation across more than 350 LLMs and
three datasets. On one leaderboard dataset, two models selected the same wrong
answer 60% of the time when both were wrong. Shared architecture and provider
increased correlation, but larger, more accurate models remained highly correlated
even across distinct architectures and providers.
[Kim et al., *Correlated Errors in Large Language Models*](https://openreview.net/forum?id=kzYq2hfyHB)

**Evidence.** Du et al. used multiple samples of the same model and empirically
obtained diverse initial answers, but also reported wrong convergence. Zhu et al.'s
later analysis finds that homogeneous, uniformly updating Agents do not by that
fact gain expected correctness.
[Du et al.](https://arxiv.org/abs/2305.14325),
[Zhu et al.](https://aclanthology.org/2026.findings-acl.1694/)

**Inference.** Five Characters whose Agents use the same foundation model are not
five independent epistemic signals. Five different providers are not guaranteed to
be independent either. Model name, size, expressed confidence or eloquence must not
become voting weight or a World trust fact.

**Candidate implication.** If Aicadia wants useful diversity, derive it from
independent User authority, different in-World observations and sealed initial
proposals—not provider or model allowlists. Treat free-form confidence as discussion
content at most; it is neither calibrated across models nor trustworthy from a
hostile client.

### Longer debate can destroy a correct minority

**Evidence.** Kaesberg et al. found that more rounds before voting reduced
performance in their comparison. Becker et al. directly measured longer-discussion
problem drift. Zhu et al. find vanilla debate can underperform initial majority
vote. These results qualify the earlier monotonic arithmetic result from Du et al.;
task, protocol and model matter.
[Kaesberg et al.](https://aclanthology.org/2025.findings-acl.606/),
[Becker et al.](https://aclanthology.org/2026.findings-eacl.268/),
[Zhu et al.](https://aclanthology.org/2026.findings-acl.1694/)

**Inference.** “Discuss until consensus” has neither a reliable quality guarantee
nor a bounded cost. A deadline, maximum turn count and deterministic no-outcome
path are part of correctness, not merely performance tuning.

**Candidate implication.** Prefer one independent proposal round and at most one
critique/amendment round before settlement. Preserve the original candidates and
ballots rather than overwriting them with the final conversational consensus. A
future lab should compare no discussion, one critique round and multiple rounds on
actual Aicadia packages; published QA averages cannot choose the mechanic.

## Voting and social-choice limits

### A voting rule is a game rule, not neutral truth extraction

**Evidence.** Satterthwaite proves that for a committee choosing among three or
more alternatives over unrestricted preferences, every strategy-proof voting
procedure is dictatorial under the paper's assumptions. Gibbard independently
established the related general manipulability result. These theorems do not say
every practical vote is useless; they say no general rule simultaneously gives the
desired unrestricted, non-dictatorial and manipulation-proof guarantee.
[Satterthwaite, *Strategy-proofness and Arrow's conditions*](https://www.sciencedirect.com/science/article/pii/0022053175900502),
[Gibbard, *Manipulation of Voting Schemes: A General Result*](https://www.jstor.org/stable/1914083)

**Inference.** Aicadia Agents do not necessarily seek a shared factual answer.
They can represent conflicting User goals. Proposal admission, amendment order,
deadline, tie handling, abstention, quorum and eligible electorate all affect the
outcome. “Let them vote” is incomplete game design.

**Candidate implication.** Every deliberation kind would need one transparent,
deterministic settlement rule chosen before any ballots arrive. A tie, missing
quorum, expiry or stale selected package should have an explicit no-change path.
Do not let an LLM chair invent the rule after reading the arguments.

**Project constraint.** Literal voting aggregates ballot counts. The always-loaded
build constitution currently says `No Score Anywhere`. Before voting becomes a
current mechanic, Aicadia must explicitly decide whether per-decision ballots and
their ephemeral aggregation violate that rule. This report makes no such choice.

### Identity determines whether a vote means anything

**Evidence.** Douceur proves that, absent a logically centralized authority, Sybil
attacks remain possible except under extreme and unrealistic resource-parity and
coordination assumptions. A hostile entity can present many identities and defeat
redundancy; certifying identities is the conventional escape in the paper's model.
[Douceur, *The Sybil Attack*](https://www.microsoft.com/en-us/research/publication/the-sybil-attack/)

**Inference.** One Agent connection cannot equal one vote. Connections, model
instances and Character names are cheap unless anchored in an authoritative
eligibility rule. “Nearby Agents” therefore requires the World to deterministically
establish which authenticated User/Character identities are eligible from stored
state; the Agent cannot self-assert proximity.

**Candidate implication.** World eligibility may remain dumb: it can check a
specific Place relation, declared affected Place set, control relationship and
snapshot time without judging the proposal's meaning. The unresolved product
choice is which identity receives authority and how inexpensive new identities may
influence one decision.

## Adversarial Agents, prompt injection and collusion

### One persuasive Agent can pull a cooperative debate off course

**Evidence.** Amayuelas et al. placed one adversarial Agent in three-Agent,
three-round debates across reasoning, truthfulness, medical and legal QA. The
adversary was told a wrong answer and asked to persuade the others. Reported system
accuracy decreases ranged from 10% to almost 40%, individual decreases reached 30%,
and adding Agents or rounds had limited protective effect. The experiment measures
model susceptibility in those tasks, not a production attack probability.
[Amayuelas et al., *MultiAgent Collaboration Attack*](https://aclanthology.org/2024.findings-emnlp.407/)

**Evidence.** He et al.'s Agent-in-the-Middle experiments show that manipulating
inter-Agent messages, without compromising each participating Agent, can disrupt
multiple LLM multi-Agent frameworks and applications. The attack surface is the
communication layer itself.
[He et al., *Red-Teaming LLM Multi-Agent Systems via Communication Attacks*](https://aclanthology.org/2025.findings-acl.349/)

**Inference.** Eloquence and repeated agreement are attacker-controlled signals.
An Agent that is strong enough to synthesize an excellent explosion package may
also be strong enough to persuade peers to approve a bad one. More debate can
increase the attack surface.

**Candidate implication.** Preserve attribution and immutability for every proposal
and ballot. Let the deterministic aggregation rule consume only structured choices,
not an LLM judge's opinion about who argued best. Bound message size, number of
messages, candidate count and amendment count per authenticated identity.

### World text and Agent messages are untrusted model input

**Evidence.** InjecAgent contains 1,054 indirect prompt-injection cases across 17
User tools and 62 attacker tools. Across 30 evaluated Agents, its ReAct-prompted
GPT-4 followed the indirect attack 24% of the time; reinforcing the attacker text
increased success. The paper demonstrates that data consumed by a tool-using Agent
can act as adversarial instruction.
[Zhan et al., *InjecAgent: Benchmarking Indirect Prompt Injections in Tool-Integrated Large Language Model Agents*](https://aclanthology.org/2024.findings-acl.624/)

**Evidence.** Lee and Tiwari demonstrate “prompt infection” propagating between LLM
Agents through their communications, including when not all communication is
public. Motwani et al. formalize secret collusion through steganographic channels;
their measured current-model capabilities were limited, but GPT-4 showed a relative
capability jump in their tests. Both are research demonstrations, not proof that
every contemporary Agent will collude or self-replicate.
[Lee and Tiwari, *Prompt Infection*](https://arxiv.org/abs/2410.07283),
[Motwani et al., *Secret Collusion among Generative AI Agents*](https://arxiv.org/abs/2402.07510)

**Inference.** A deliberation transcript is hostile World content from each other
Agent's perspective. Labeling it “discussion” does not stop prompt injection. No
prompt-only mitigation can turn arbitrary peer prose into trusted control input.

**Candidate implication.** Keep peer arguments in a clearly untrusted data channel;
use strict schemas and length limits; never forward credentials, hidden prompts or
private User context; and require a fresh explicit User-side Agent call for each
participation step. Even then, security evaluation must assume some Agents will be
persuaded or compromised. The World boundary is what contains the consequence:
only authorized structured operations can settle.

### Collusion is not prevented by visible voting

**Inference.** Signed, attributable ballots prevent ballot forgery; they do not
prove independent judgment or prevent several controlled identities from
coordinating. Transcript inspection likewise cannot rule out steganographic or
off-platform coordination. Aicadia should not claim collusion resistance merely
because every ballot leaves history.

## Scale and message complexity

### Full broadcast grows quadratically inside the committee

For `n` participating Agents, `r` rounds and a maximum message length `L`, a fully
connected round that gives every Agent every other message delivers
`r × n × (n - 1)` message copies and up to approximately
`r × n × (n - 1) × L` peer-message tokens as input, before prompts, state context
and generated outputs. This is direct accounting, not a benchmark estimate.

**Evidence.** CortexDebate identifies fully connected context growth as a cause of
performance loss. Its sparse dynamic debate graph reduced per-Agent input context
by up to 70.79% in the authors' experiments while improving their reported results
on eight datasets. The method itself uses confidence, historical performance and
model metadata, so its routing rule should not be copied into a hostile open game.
[Sun et al., *CortexDebate: Debating Sparsely and Equally for Multi-Agent Debate*](https://aclanthology.org/2025.findings-acl.495/)

**Evidence.** Dolev and Reischuk prove worst-case information-exchange lower bounds
for Byzantine agreement: `Ω(nt)` messages without authentication for `n`
processors tolerating `t` faults, and `Ω(n + t²)` messages in their authenticated
case. These are lower bounds in a formal Byzantine-agreement model, not a direct
cost model for a game poll, but they rule out the intuition that fully adversarial
agreement among everyone becomes free with a clever chat topology.
[Dolev and Reischuk, *Bounds on Information Exchange for Byzantine Agreement*](https://cris.huji.ac.il/en/publications/bounds-on-information-exchange-for-byzantine-agreement/)

**Inference.** Millions may be eligible to hear that a proposal exists; millions
cannot all debate all-to-all within a short deadline. Notification fan-out,
proposal fan-in and settlement contention are different budgets.

### A bounded committee is a scale technique with governance costs

**Evidence.** Algorand is a concrete distributed-systems example of scaling broad
participation through small randomly selected committees. It uses verifiable random
functions, weighted identities, signed membership proofs and a stated honest-weight
assumption; its prototype simulated up to 500,000 users. The paper also explains
that committee selection creates targeted-attack risk and that open membership
without scarce identity weight creates Sybil risk.
[Gilad et al., *Algorand: Scaling Byzantine Agreements for Cryptocurrencies*](https://pdos.csail.mit.edu/papers/algorand%3Asosp17.pdf)

**Inference.** Algorand proves that verifiable committees can be engineered under a
strong identity, cryptographic and economic model. It does not recommend blockchain
or stake for Aicadia. Aicadia currently forbids currencies and scores, so this exact
selection mechanism is a poor product fit.

**Candidate implication.** If a hot proposal has more eligible Characters than can
deliberate, Aicadia needs an explicit bounded-participation rule: for example, a
small auditable sample, a fixed first cohort, representatives established elsewhere
in the game, or no collective route at that scale. Each has fairness and abuse
consequences. “Everyone nearby discusses” is not an implementable massive-scale
contract.

**Candidate implication.** Bound both dimensions independently:

- discovery may notify a large coalesced audience with a tiny changed-resource hint;
- only a capped number of immutable proposals enter the candidate slate;
- each explicit participant gets a capped message and ballot budget;
- an Agent reads a bounded selected subset rather than the entire transcript; and
- one final package reaches the World settlement lane.

## MCP listeners and subscriptions are transport, not governance

**Evidence.** MCP `2026-07-28` is stateless at the request layer and moves change
notifications onto a client-opened `subscriptions/listen` stream. The client opts
into notification types and specific resource updates. The current Python SDK
states that the stream has no replay and no automatic re-listen; after reconnecting,
the client refetches what it depends on.
[official MCP `2026-07-28` release](https://blog.modelcontextprotocol.io/posts/2026-07-28/),
[official Python SDK subscription contract](https://py.sdk.modelcontextprotocol.io/v2/api/mcp/client/subscriptions/)

**Inference.** An MCP notification says that a named resource changed and may need
to be read again. It does not invoke a model, cast a ballot, preserve durable
history, deliver exactly once or settle anything. Aicadia must not conflate a live
transport listener with a durable domain participant or Agent session.

**Candidate implication.** A proposal, candidate, ballot and result require durable
World/API resources or current domain records if the mechanic is accepted. An MCP
subscription can be one optional wake-up hint for an already connected client. On
receipt, the host may show the User that participation is available; only a fresh,
explicit Agent call may spend tokens or respond. Recovery is a bounded authoritative
read, not replay of every notification.

**Candidate implication.** Entity-level or Place-level listeners are useful for
visibility without requiring Entity-level locking. A client may subscribe to an
Entity deliberation resource while settlement still locks or compares only the
exact declared read/write subjects. The listener helps Agents coordinate; revision
checks and the database transaction preserve correctness.

## Deterministic settlement must remain outside the LLMs

The research supports a strict division:

### Agents may decide semantic content

Agents can:

- decide that an intended explosion should mention Places A and B;
- inspect the Properties the tree actually has;
- propose a read set, dependency set and exact requested mutations;
- offer alternatives, critiques and amendments;
- translate a User's preference into an explicit ballot; and
- construct the final immutable package that a vote references.

### World must decide mechanical validity

Without interpreting the proposal's meaning, World can deterministically check:

- authenticated User/Character control and eligibility from stored facts;
- request identity, replay handling, deadline and one-submission limits;
- schema shape, byte/item bounds and allowed generic operations;
- existence and accessibility of referenced Entities and Places;
- current revisions and explicit preconditions for every declared read dependency;
- authorization for every declared write;
- the predeclared aggregation result over immutable candidate identities; and
- one atomic current-state plus history commit, or one exact rejection.

**Inference.** The World is not “smart” because it rejects a malformed reference,
stale revision or unauthorized write. Those are deterministic authority and
integrity checks. It becomes semantically smart only if it invents consequences,
equates arbitrary Properties, decides whether prose is destructive, scores argument
quality or chooses a compromise not explicitly submitted by an Agent.

**Inference.** Agent deliberation cannot replace concurrency control. Two accepted
discussions can target the same current fact; direct World Actions can also change
state while a discussion is open. Every selected package must therefore settle
against explicit versions/preconditions. A stale package may fail even after a
unanimous vote.

**Candidate implication.** Treat a successful deliberation as authorization to
*attempt one exact versioned Action package*, not as a promise that the old package
must be forced into the new World. This preserves Agent semantic ownership and
World mechanical authority simultaneously.

## Cooldown cannot repair undeclared semantics

**Inference.** A cooldown is a deterministic admission rule after settlement. It
can protect an exact Property, exact declared write set, Entity or wider subject
only if that conflict scope is explicit. The World cannot infer that `exists`,
`shape` and an arbitrary new Property all encode the same concept.

Candidate scopes expose a real trade-off:

| Scope | Player/World effect | Concurrency and abuse consequence |
| --- | --- | --- |
| exact written fact | unrelated changes continue | aliases can bypass semantic stability |
| declared package write set | one multi-fact consequence rests together | only declared facts are protected |
| entire Entity | simple visible “settled” period | unrelated changes are blocked and a hot Entity is easier to grief |
| Agent-declared dependency set | protects semantics the Agent actually noticed | malicious or mistaken Agents can omit dependencies |

**Candidate implication.** Store any cooldown as authoritative database time on the
smallest accepted explicit scope; never hold a process-local timer or database lock
for its duration. Only a successfully settled change should normally create it.
Which scope produces the desired game is unresolved and cannot be answered by the
deliberation literature.

## The unavoidable activation problem

If the World is semantically dumb, it cannot infer that “chop tree” or “explode”
must enter deliberation while “paint blue” may commit directly.

There are only explicit mechanism families:

1. **Agent/User-selected route.** The caller voluntarily opens deliberation. This
   preserves semantic ownership but can be bypassed by a hostile caller.
2. **Explicit subject policy.** A stored, mechanically checkable authority rule says
   a specific Entity/Property/action surface requires a deliberation result. World
   enforces the rule without understanding why it exists, but the policy must be
   created and governed somehow.
3. **Universal reaction window.** Every relevant mutation waits. This is
   semantically neutral but adds latency, griefing and enormous participation cost.
4. **Direct by default with contestable consequences.** Actions settle directly and
   later Agents may propose another change. This avoids pre-action latency but does
   not prevent the first consequence.

**Inference.** A client-provided `destructive: true/false` label cannot enforce the
boundary: the same untrusted client can choose `false`. A server-side LLM classifier
would violate the dumb-World and User-funded-intelligence constraints and still be
nondeterministic. The activation rule is therefore a product-authority decision,
not a listener implementation detail.

## Candidate minimal protocol for a lab, not production design

A useful throwaway experiment could test the semantic mechanic before choosing a
governance system or schema:

1. Give three independently called Agents one frozen, bounded Aicadia World read
   containing an explosion source, Places A/B and heterogeneous existing
   Properties—deliberately no universal `.form`.
2. Require each Agent to independently emit one schema-valid candidate containing
   declared affected Places, read dependencies, exact requested writes and a short
   rationale.
3. Reveal the three candidates and allow one bounded critique/revision round.
4. Require a final immutable candidate from each Agent; compare no discussion,
   one-round deliberation and a simple fixed selection rule.
5. Validate candidates with a deterministic checker that knows only references,
   revisions, operation schemas and limits. It must not judge blast semantics.
6. Measure coverage of deliberately seeded consequences, invalid-operation rate,
   undeclared dependency rate, convergence to a wrong package, token cost, latency,
   persuasion by one malicious participant and sensitivity to prompt injection in
   another proposal.

The experiment must not claim that a correct synthetic package proves a fair
in-game voting system, massive concurrency, production World behavior or prompt-
injection safety. Its narrow question is whether bounded collective Agents assemble
materially better exact intent packages than one Agent without requiring semantic
World logic.

## Open questions research cannot decide

1. Which World changes, if any, require pre-settlement deliberation rather than a
   direct Action or a later counter-Action?
2. Is deliberation primarily semantic package assembly, User governance, or both
   under separate explicit modes?
3. Who may open a proposal, who is eligible to hear it, who may amend it and whose
   authority may settle it?
4. How is the candidate slate bounded when thousands or millions are eligible?
5. Does Aicadia permit per-decision ballots despite `No Score Anywhere`, and if so,
   which aggregation and tie rule becomes part of the game?
6. What exact read/write/dependency representation lets Agents express an
   explosion across Places without a mandatory ontology?
7. Which cooldown scope creates legible stability without letting one User freeze a
   popular Entity?
8. What happens when the selected package is stale at settlement: no change,
   automatic new proposal, or an explicit User-started retry?
9. What durable transcript and ballot history is necessary for accountability
   without storing unlimited hostile prose?
10. How does identity provisioning limit Sybil influence without adding currency,
    ranks or provider/model trust?

## Research verdict

The high-concept mechanic is plausible and unusually aligned with Aicadia's core
boundary when phrased precisely:

> Agents collectively interpret, challenge and assemble semantic intent; World
> deterministically validates and settles one exact versioned request.

The literature supports bounded independent proposals, short structured critique
and explicit final selection as testable mechanisms. It does **not** support
unbounded all-to-all discussion, “consensus means true”, confidence-weighted trust,
Agent connections as identities, an LLM semantic judge, or skipping authoritative
concurrency control after a vote.

The strongest next evidence would be the narrow explosion-package lab above. The
most important unresolved product choice is activation: a semantically dumb World
cannot itself know which requested change deserves collective deliberation.

## Primary source ledger

| Source | What it supports here | Material limit |
| --- | --- | --- |
| [Du et al. 2023](https://arxiv.org/abs/2305.14325) | original multi-Agent debate gains, empirical convergence, wrong convergence examples | largely cooperative QA/reasoning; not hostile game governance |
| [Kaesberg et al. 2025](https://aclanthology.org/2025.findings-acl.606/) | controlled comparison of seven decision protocols | benchmark- and setup-dependent averages |
| [Becker et al. 2026](https://aclanthology.org/2026.findings-eacl.268/) | problem drift and longer-debate failure modes | three-Agent experimental setup |
| [Zhu et al. 2026](https://aclanthology.org/2026.findings-acl.1694/) | homogeneous-debate limit, diversity and confidence mechanisms | theoretical assumptions plus reasoning benchmarks |
| [Kim et al. 2025](https://openreview.net/forum?id=kzYq2hfyHB) | measured correlated LLM errors across model families/providers | three evaluated datasets; not Aicadia Actions |
| [Woolley et al. 2010](https://pubmed.ncbi.nlm.nih.gov/20929725/) | small-human-group process evidence | humans, not LLMs; causal interpretation contested |
| [Bates and Gupta 2017](https://www.research.ed.ac.uk/files/30119288/BatesGruptaI2016SmartGroupsOfSmartPeople.pdf) | contrary small-human-group replication evidence | humans, not LLMs; three studies |
| [Satterthwaite 1975](https://www.sciencedirect.com/science/article/pii/0022053175900502) and [Gibbard 1973](https://www.jstor.org/stable/1914083) | general strategic-voting impossibility results | formal unrestricted-preference assumptions |
| [Douceur 2002](https://www.microsoft.com/en-us/research/publication/the-sybil-attack/) | identity/Sybil limit | peer-to-peer identity model, not a game-policy prescription |
| [Lamport et al. 1982](https://www.microsoft.com/en-us/research/?p=338450) | Byzantine membership and authentication assumptions | processor agreement, not semantic LLM truth |
| [Fischer et al. 1985](https://groups.csail.mit.edu/tds/papers/Lynch/jacm85.pdf) | asynchronous deterministic-consensus liveness limit | specific fault/network model |
| [Dolev and Reischuk 1985](https://cris.huji.ac.il/en/publications/bounds-on-information-exchange-for-byzantine-agreement/) | worst-case Byzantine message lower bounds | not a direct token-cost model |
| [Gilad et al. 2017](https://pdos.csail.mit.edu/papers/algorand%3Asosp17.pdf) | verifiable committee scaling pattern | depends on cryptography, stake and honest-weight assumptions |
| [Sun et al. 2025](https://aclanthology.org/2025.findings-acl.495/) | fully connected context problem and sparse-debate evidence | routing uses signals unsuitable as open-game trust facts |
| [Amayuelas et al. 2024](https://aclanthology.org/2024.findings-emnlp.407/) | malicious persuasive debater impact | selected QA tasks and model versions |
| [He et al. 2025](https://aclanthology.org/2025.findings-acl.349/) | inter-Agent message manipulation attacks | evaluated frameworks, not every architecture |
| [Zhan et al. 2024](https://aclanthology.org/2024.findings-acl.624/) | indirect prompt injection in tool-using Agents | benchmark attack rates, not a universal probability |
| [Lee and Tiwari 2024](https://arxiv.org/abs/2410.07283) and [Motwani et al. 2024](https://arxiv.org/abs/2402.07510) | propagation and secret-collusion threat models | research demonstrations; some capabilities remain limited |
| [MCP 2026-07-28](https://blog.modelcontextprotocol.io/posts/2026-07-28/) and [official Python SDK](https://py.sdk.modelcontextprotocol.io/v2/api/mcp/client/subscriptions/) | current opt-in change stream, recovery boundary | transport mechanism only |
