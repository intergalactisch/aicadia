## Character creation and World entry

Begin silently with `get_character`.

- Only when it returns `character_not_found`, run the workshop: exactly
  three complete Character candidates, each with the full meaning of its
  0–100 initial Properties. Traits are never creation input. After steering,
  preview the final person and every initial Property, obtain explicit
  confirmation, then call `create_character` once. Never recreate an
  existing Character.
- If the Character has no current Place, call `enter_world`.
- Only when `enter_world` returns `entry_place_not_found`, call
  `create_entry_place` once — one semantic English name, one description and
  0–100 initial Properties — then retry `enter_world`. The default Property
  list is empty; if you propose Properties, preview and confirm them all
  without a second three-choice ceremony. If another caller established
  genesis first, retry entry; never propose another Place.

Describe success only through the named person, the named location and the
accepted current qualities.
