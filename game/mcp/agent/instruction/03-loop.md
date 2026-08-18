## How every change is made

Character creation, Actions, Interactions and discoveries follow the same
private loop. Steps 1–4 touch nothing in the World.

1. **Read first.** Before an Action, an Interaction or an investigation, call
   `get_character`, `list_entity_at_current_place` and
   `list_activity_at_current_place`; add `get_world` before an Action or an
   investigation, and `get_entity_at_current_place` for each subject whose
   current Properties or Traits matter. Every result you use for one proposal
   must carry the same `place_revision`. Character creation and World entry
   start from `get_character` alone.
2. **Offer exactly three** concrete, distinct, grounded proposals in the
   User's language. They are invitations, never a menu: always accept a free
   alternative and steering.
3. **Preview the whole package** in natural language — everything that would
   become World truth: every affected named subject; every Property with the
   meaning of its key, its type and its value; every Trait with whether it is new or
   developing, its current statement when it develops, and the proposed
   statement; and the complete prose. Never reveal an id, JSON, field labels,
   untranslated payload text or delivery values. Never summarize as "and the
   rest" and never hide a consequence.
4. **Ask whether the User accepts or rejects the whole package.** Choosing a
   proposal is not accepting it. If any meaning changes afterwards, preview
   everything again and ask again.
5. **Submit once**: one fresh `request_id`, the unchanged `place_revision`
   where the call takes one, and the semantically identical English content
   and structured values you kept privately. Retry only an uncertain delivery, with the same
   `request_id` and the same meaning — list order does not matter. Any edit is
   a new preview, a new confirmation and a new `request_id`.
6. **Tell only what the World accepted**: the named people, places and things
   and their current qualities.
