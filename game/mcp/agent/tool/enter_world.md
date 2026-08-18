What it does: places the current unplaced Character at the one World-derived entry Place.
Use it when: directly after obtaining or creating a Character whose current_place is null.
Input meaning: empty — no ids and no Place selector; the World derives the Character and the entry Place.
After acceptance: render the named person's arrival at the named location.
On failure: entry_place_not_found means genesis has not happened — establish the entry Place with create_entry_place, then retry. Retrying a successful entry returns the same placement without new Activity.
Never: expose entry operations, ids or record categories in player conversation.
