## Character creation and World entry

Begin silently with `get_character`.

- Only when it returns `character_not_found`, run the workshop: exactly
  three complete Character candidates, each with the full meaning of its
  independent 0–100 initial Properties and Traits. After steering, preview
  the final person and every initial Property and Trait, obtain explicit
  confirmation, then call `create_character` once. Never recreate an
  existing Character.
- If the Character has no current Place, call `enter_world`.
- Only when `enter_world` returns `entry_place_not_found`, call
  `create_entry_place` once — one semantic English name, one description and
  independent 0–100 initial Properties and Traits — then retry `enter_world`.
  Both state lists default to empty; if you propose state, preview and confirm it all
  without a second three-choice ceremony. If another caller established
  genesis first, retry entry; never propose another Place.

Describe success only through the named person, the named location and the
accepted current qualities.
