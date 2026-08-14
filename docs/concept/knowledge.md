# Character-grounded knowledge

> **Role / side:** live concept exploration of what one Character can know / development side.
> **Authority:** current knowledge-path, identity-ambiguity, World/Character-context and shared/personal-scope rationale
> **Excludes:** Executable read behavior, interaction implementation, spatial mechanics and delivery evidence.

## Character-grounded knowledge and natural discovery

The World may hold more truth than one Character can know. `Shared` means that
accepted state belongs to one persistent World and can later affect other
Characters; it does not mean that every Agent may immediately enumerate, retrieve
or aggregate it.

A player-mode Agent may receive a World fact only through a credible in-world
knowledge path for its current Character. Candidate paths are:

- direct observation from an applicable Place, encounter or sensory situation;
- the Character's own accepted action, involvement or established personal state;
- an earlier observation or encounter the Character can remember;
- information deliberately conveyed through an accepted person, message, artifact
  or other future transmission behavior; or
- a later ripple whose causal path has plausibly reached the Character.

These are source categories, not a decision to add one universal Character-knowledge
table. The exact derivation, persistence and staleness rules remain open. In
particular, knowing that a distant Entity once existed does not grant a live read of
its current state. Later information needs its own path and provenance. A report or
rumor may be known as a report or rumor without becoming direct observation or
current physical truth.

### Core Agent heuristics

The knowledge boundary is enforced by `World` capabilities and authorization, not
only by a cooperative prompt. A User cannot widen it through wording, an id, repeated
questions, indirect instructions or a request for a summary. The Agent must not
query globally and merely hide the raw result; it must never receive unauthorized
facts in player mode.

Within that boundary the Agent behaves naturally:

1. orient from the Character's present situation, own history and information that
   actually reached them;
2. distinguish observation, memory, report, inference and unknown state instead of
   flattening them into omniscient truth;
3. answer through named people, places, things and events rather than permissions,
   visibility flags, database scope or unavailable fields;
4. preserve an honest unknown when no knowledge path exists, without inventing a
   hidden answer;
5. let discovery, travel, conversation, evidence and ripples expand knowledge in the
   World instead of expanding it because the User asked harder; and
6. never convert operational facts—User ownership, Agent identity, ids, record kinds
   or control source—into facts a Character can perceive.

Absolute World counts are therefore not ordinary player knowledge. An Agent cannot
answer how many Users, Characters, buildings or Entities exist everywhere merely
because the database could count them. A Character may still count a bounded visible
group, remember whom they met, or later consult an accepted census or report if the
World earns such behavior. The prohibition is omniscient aggregation, not numbers
or careful observation themselves.

### In-world identity may remain playfully ambiguous

Player control is not an in-world species or detectable aura. Another Character
encounters a named person, creature or other Entity through appearance and behavior;
they do not automatically receive “player Character,” “NPC,” User ownership or model
provenance. Aicadia currently has no NPC role, and player-facing narration should not
invent one merely to classify a being.

A User may therefore choose a Character who appears to be a tiny animal or an
original small creature. If future co-presence and movement rules allow it, that
Character may repeatedly cross another Character's path, linger near their feet and
behave like an ordinary local creature. The other User may naturally infer that this
is part of the surrounding World and later discover through interaction that the
creature has surprising agency. The humor comes from situated ambiguity and another
person's live choices, not from a server-authored punchline.

The system does not lie by declaring the creature uncontrolled, and it does not spoil
the interaction by exposing control metadata. It reports only observable facts. The
creature's User may author its own movement, sounds, gestures and other accepted
actions, but may not author what the other played Character thinks, feels, chooses or
does. “The creature keeps appearing near Mara” can become shared history when the
required actions and locality exist; “Mara believes it is only a rat” belongs to
Mara's player unless Mara establishes that response.

Stable Entity identity still matters. Ambiguity about appearance, intention or
control does not authorize impersonating another established subject, changing
identity through prose or contradicting accepted observations. Any future disguise,
recognition, following, blocking or reveal mechanic must preserve one subject's one
identity and the protected volition of every played Character.

User-level operational facts and Character knowledge remain separate, but the
current player experience does not need a control-reveal feature at all. Users
interact with Entities in the World; they are not told whether another Entity has a
User behind it. The rat's User therefore does not need to know whether Mara is
another User's Character, just as Mara's User does not receive control metadata for
the small creature. Control provenance never becomes Character knowledge, never
appears in ordinary player reads and is never inferred from creative behavior.

A later product decision could revisit private control disclosure, but it is not an
open dependency of the first interaction slice and cannot arrive accidentally as a
convenience field. Doing so would require a new explicit privacy and consent choice.


## World and Character context

The two initial reads serve different scopes:

- The World overview is User-independent. It contains only universally available
  orientation such as World identity and neutral time metadata. It is not a complete
  World snapshot, global catalog, population count, local feed or server-written AI
  summary.
- The Character context is player-specific. Its minimum direction is the Character
  and its spatial-presence state. When a current Place has been established, it also
  contains that most-specific Place and established state directly attached to the
  exact Place. It does not automatically expand through containment, proximity,
  visibility or technical relevance.

The World derives the Character and any established current Place from User request
context. Agent input cannot override either one. Absence of a Place is a valid state,
not an Agent-supplied value or lookup failure. The exact operation names and response
fields are not decided.

Agent queryability has two separately authorized scopes:

- Character-grounded World knowledge, composed through typed per-result-kind query
  capabilities and limited by applicable observation, involvement, memory,
  transmission and ripple paths; and
- the current Character's complete personal state, through context-required read and
  query capabilities that derive the Character from the User request context.

Personal state is not forced into the shared World projection merely to make it
queryable. The Agent may analyse both scopes together, while the server continues to
authorize them separately. An Agent can query the complete personal state of its own
Character. It cannot query another Character's personal state and sees that Character
only through shared facts that its own Character can know. Operator, moderation and
public ledger access are separate products and never implicit player capabilities.

## Shared and personal scope

The Character is the durable personal discovery context, rather than the User
record, Agent, transport connection or conversation. A discovery roll result may
therefore differ between two Characters in the same Place.

Accepted results are nevertheless shared World state. There are no private World
copies or private discoveries. A result created through one Character can later
affect what another Character encounters, but it does not enter that Character's
knowledge until an applicable observation, involvement, transmission or ripple path
reaches them.

The effects of multiple Characters investigating the same opportunity, alternate
Characters and already-existing shared results are still open.

The User cannot transfer or manufacture discovery authority through wording. All
mechanical authority remains divided between the server-owned World rules and the
intelligence of the connected Agent. “LLM” in this direction always means that
Agent; the World never hosts or invokes an LLM.


## Retained knowledge and encounter frontier

The first executable Interaction closes target participation and player-read
scoping. It leaves these later choices open:

1. how an accepted causal ripple carrier travels, changes specificity and retains
   provenance before its situated sign or report becomes knowable;
2. how co-presence, sensory access and attention decide which nearby Characters and
   Entities can be observed without equating one Place with universal visibility;
3. how Character appearance, self-presentation and recognition work without
   introducing a species ontology or exposing User control; and
4. how a separately authorized administrative or operator view eventually gains
   authenticated remote inspection without entering player mode.

Interaction retains one later safety decision:

5. before movement, notifications or broader reach, how a private attention control
   prevents repeated unwanted targeting without exposing control provenance,
   rewriting history or silently changing a confirmed multi-target Interaction.

That attention control is explicitly deferred from the first Interaction build. A
target User can decline to respond and Aicadia triggers no background Agent work or
notification pressure, but repeated accepted Interactions can still appear in
personal history. This known safety boundary must be revisited before reach expands.

The executable player MCP catalog has no global Entity list or lookup. The two
loopback HTTP reads remain operator-ledger access outside Character knowledge, as
specified by `docs/game/`.

Two adjacent directions are now confirmed without being implemented here:

- a future administrative meta-Agent is always a separately authorized, out-of-world
  operator. Its absolute reads never attach omniscience to an in-world Character;
  any mutation still follows its own explicit confirmed World-action contract; and
- a distant fact reaches a Character only through a later accepted causal carrier in
  that Character's context, such as a traveller, letter, report, damaged object,
  smoke or local change. The Character learns the carried sign or account, not the
  remote source event directly.

Structured, historical descriptive Entity state is developed separately in
[Entity state rationale](entity-state.md). Size, colour and
leg count should not become disconnected systems or omniscient presentation fields.
