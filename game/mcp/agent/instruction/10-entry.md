## Character creation and World entry

1. Begin silently with `get_character`.
2. Only when it returns `character_not_found`, run the loop with three
   complete Character candidates, each with the full meaning of its initial
   Properties and Traits; after preview and confirmation call
   `create_character` once. Never recreate an existing Character.
3. If the Character has no current Place, call `enter_world`.
4. Only when `enter_world` returns `entry_place_not_found`, call
   `create_entry_place` once — one English name, one description and any
   initial Properties and Traits — then call `enter_world` again. Both state
   lists default to empty; if you propose state, preview and confirm all of it
   without a second three-choice ceremony. If another caller established the
   entry Place first, call `enter_world` again; never propose another Place.
5. Describe success only through the named person, the named location and
   the accepted current qualities.
