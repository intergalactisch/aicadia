# Aicadia player capability map

> **Role / side:** player-experience planning map / development side.
> **Authority:** maps current and future User experiences to their owning authorities and backlog state.
> **Excludes:** accepted game contracts and delivery evidence; see `docs/game/` and `docs/evidence/`.

Status: living planning index

This map lets a reader see which experiences Aicadia currently gives a User through
their Agent and which game capabilities are still being designed. It is not an
implementation contract: [current game docs](../../docs/game/README.md) govern the
accepted behavior, the [ordered backlog](README.md) and active plan govern unfinished
execution state, [evidence](../../docs/evidence/README.md) records delivered results,
and the [concept log](../../docs/concept/log/README.md) records why direction changed.

The delivered Interaction tasks in the [interaction plan](../plans/20260813-114241-character-grounded-interaction-history/plan.md)
contains the complete D&D and wider-tabletop traceability matrix from research
finding to current requirement, future capability, editorial lens or explicit
non-import.

Update this map whenever a current capability, selected edge or ordered horizon
changes. Do not mark a capability executable from documentation or research alone.

## Current executable experience

| User experience | What the User can do now | Boundary | Authority |
| --- | --- | --- | --- |
| Enter player mode | Start an isolated Agent conversation that requires live Aicadia MCP and stays in natural in-world language | No browser chat, fallback authority or server-side model | [Agent play contract](../../docs/game/agent.md) |
| Establish one Character | Choose or steer one of three Agent proposals, preview the full meaning and explicitly confirm one Character | At most one Character; creation leaves it unplaced | [Character workshop](../../docs/game/agent.md#required-character-workshop-and-world-entry-flow) |
| Enter the shared World | Arrive with that Character at the one World-derived entry Place | No destination choice, movement or further Place | [Place and World entry](../../docs/game/domain.md#place-and-world-entry) |
| Orient at the exact Place | Read compact local descriptions, then fetch the actor or one selected exact-local Entity with bounded current Property/Trait associations, plus only Activity authorized for this Character at that Place | Exact Place equality is eligibility, not general visibility, sensory access or neighborhood; Property/Trait content is not control provenance | [Local Entity list](../../docs/game/capability/list_entity_at_current_place.md) · [local Activity](../../docs/game/capability/list_activity_at_current_place.md) · [scoped Entity state](../../docs/game/capability/get_entity_at_current_place.md) |
| Recall personal history | Read immutable accepted Activity in which the current Character acted or was explicitly involved | No private conversation, rejected attempt or inferred knowledge history | [Personal history](../../docs/game/capability/list_activity.md) |
| Establish or change local state | Choose and steer one of three grounded directions, completely preview and confirm one Action that introduces a placed Entity with initial Properties and Traits or changes both state kinds together | No introduction-plus-existing-state mixture, generic patch, remote selector, direct storage edit or executable Trait prose | [Character-grounded Action](../../docs/game/capability/submit_action.md#contract) |
| Investigate and complete one find | Let World resolve an admitted zero or positive investigation before authorship; after a positive, re-ground, preview and confirm one found Entity whose state and Activity become shared | No caller-selected odds/result, made-object substitution, movement, new Place or unconfirmed commit | [Investigation start](../../docs/game/capability/start_investigation.md) · [discovery commit](../../docs/game/capability/submit_discovery.md) |
| Observe another marker | A second Character at the same Place can find the same Entity and canonical prose | This proves shared state, not complete co-presence or Character observation | [Cross-contract evidence obligations](../../docs/game/adapter-parity.md#cross-contract-evidence-obligations) |
| Interact with present Entities | Direct one confirmed outward behavior toward 1–100 distinct co-present Entities, optionally changing actor/explicit-target Properties and establishing/developing their Traits | No target-authored response, thought, consent, relationship, placement, implicit witness or Trait mechanics | [Character-grounded Interaction](../../docs/game/capability/submit_interaction.md#contract) |
| Recall directed interaction | Actor and explicit target Character can later read the same canonical outward behavior and complete target set; a reverse response is a new Interaction | Non-target bystanders and distant Characters receive no Interaction automatically | [Personal history](../../docs/game/capability/list_activity.md) · [Place history](../../docs/game/capability/list_activity_at_current_place.md) |
| Encounter without control labels | Meet a locally described Entity such as Pip the rat without being told whether it is User-controlled, an NPC or an ordinary subject | Control provenance, global counts and distant facts are unavailable in player mode | [Agent knowledge boundary](../../docs/game/agent.md#agent-guidance-and-player-facing-communication) |
| Inspect the local ledger | In development, view World, Entity and personal Activity records in a read-only browser page | The ledger is not gameplay and exposes no mutation | [Local play](../../docs/game/local-play.md#ledger-boundary) |

## Current player boundary

The [capability catalog](../../docs/game/README.md#capability-catalog) owns the exact
accepted player surface. The table above groups the delivered subset by User
experience; this planning map does not restate contract or implementation meaning.

## Future experience

Uniform Entity creation, state-change packages and the first complete discovery loop
are delivered in the current experience above. The bounded Sol-medium smoke
validation is closed and adds no player capability. No next edge is currently
selected; the ordered backlog retains the queued horizon.

| Capability | Intended User experience | Planning state | Next step or unresolved edge |
| --- | --- | --- | --- |
| Private attention control | Privately stop repeated unwanted targeting without rewriting co-presence or shared history | Deferred | Revisit before movement, notifications or broader Interaction reach |
| Ripple knowledge | Learn a reduced, sourced consequence of a distant event only after it plausibly travels to the Character | Explored | Define propagation, information loss, provenance and arrival behavior |
| Place neighborhood | Inspect explicit containing and adjacent Places around the exact current Place | Later / Queued | Select first relationship behavior without implying visibility |
| Movement and expansion | Establish additional Places and connections and move through validated transitions | Later / Queued | Define route, action, time and historical location semantics |
| Rich World subjects | Establish flora, fauna, materials, boundaries and temporal processes when play needs them | Later / Queued | Introduce one concrete domain behavior at a time |

## Cross-cutting rules

- The User acts through one Character and their own Agent; World alone accepts state.
- Every accepted mutation leaves immutable, attributable Activity in the same
  transaction.
- Shared persistence never grants omniscient player access.
- Agent proposals, steering and rejected drafts remain private until exact
  confirmation and World acceptance.
- One Entity keeps one stable identity; appearance and recognition may be
  asymmetric, while player mode exposes no User-control provenance.
- The first Interaction has no block/ignore state, notification or background Agent
  activation. A target owns every response; private attention controls must be
  designed before Interaction reach expands.
- Participation, observation, knowledge, relationship and recap are different
  meanings and may never be collapsed into one generic interaction flag.
- Entity Properties are key/value facts and Traits are characterizing statements
  that can develop. Both are state owned by the Entity—not Relationship or
  observer-knowledge state—but remain separate meanings, not per-key systems, RPG
  statistics or incidental prose.
- Property keys provide canonical meaning and value types. Many Entity-owned
  Properties may reference the same key while retaining their own values; a key is
  mechanics vocabulary, not Character knowledge or a central Property.
- The accepted Property slice treats all Entity creation routes uniformly, reuses
  `submit_action` for bounded World changes and permits typed Property consequences
  inside `submit_interaction` for its actor/targets. Each value is stored once in
  immutable Activity-backed history with only a current pointer per Entity/key.
- Agent-created canonical keys contain only immutable English key and value type;
  current Properties are outward/local facts and override conflicting introductory
  prose for that exact fictional meaning. World infers no aliases or synonyms.
  Control-like Property keys and values remain user-authored in-World content, never
  actual control, ownership, User, Character or NPC provenance.
- Delivery history and current status: see [Trait evidence](../../docs/evidence/trait.md).
- Users steer and confirm natural-language meaning but never write Property or Trait
  state directly; Agents author exact Actions and World alone validates and writes.
- Uniform local Property consequences do not branch on Entity role or User control,
  including for Characters and Places. Current executable causes are confirmed
  Agent-authored Actions, Interactions and discoveries. External-factor mechanics
  may later reuse the private writer only after explicit acceptance; no background
  Agent, timer, `world_event` or ungrounded simulation is delivered now.
- No capability adds scores, levels, currencies, background inference or a hidden
  server narrator.

## Living play-experience heuristics

These apply across current and planned capabilities without becoming new domain
objects:

- orient from Character-knowable current Place, recent change, grounded callback and
  open affordance;
- offer concrete attempts rather than endings, with three proposals as assistance
  rather than an exhaustive menu;
- keep expression free while World consequences remain typed, attributable and
  explicitly confirmed;
- let success, failure and accepted interaction change future play rather than award
  points;
- use Entity and Activity recurrence for continuity while recaps remain derived
  Character-scoped lenses;
- preserve honest unknowns, delayed evidence and causal ripples instead of hidden
  Agent-authored truth or global feeds;
- allow humor, ordinary days and small acts without compulsory escalation; and
- keep User intent, Agent framing, World resolution, Character knowledge and shared
  recollection visibly separate.
