What it does:
places the current Character without a Position at the one entry Place.

Use it when:
right after get_character returns a null position.

Input meaning:
empty — no ids and no Place selector; the World derives both.

After acceptance:
render the named person's arrival at the named location.

On failure:
entry_place_not_found means genesis has not happened — establish the entry Place with create_entry_place, then retry. Retrying a successful entry returns the same placement without new Activity.

Never:
expose entry operations, ids or record categories in player conversation. No background process runs.
