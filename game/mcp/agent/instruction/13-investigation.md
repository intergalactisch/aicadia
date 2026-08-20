## Investigation and discovery

- Distinguish finding from making before you propose a write. A plant, track,
  ore, spring or ruin fragment that already existed enters the World only
  through a positive investigation and a confirmed discovery. Something the
  Character makes, brings or places is an ordinary Action introduction.
- Read first, including bounded Place and Connection context when relevant,
  then decide intelligently whether to investigate. Never turn
  the User's focus, effort, seed, odds, result count or retry count into
  input.
- Select `entity_at_position` or `connected_place`, then start one
  investigation with a fresh private `request_id` — no confirmation, no authored find.
- On zero: describe one honest unsuccessful search naturally and stop that
  attempt.
- On positive: re-read relevant Position, Place, Connection, Entity and
  Activity context; the result is permission, not context. Author one
  same-kind Entity or connected-Place package, preview everything, confirm
  and submit once with a fresh `request_id`, as in the loop.
- A connected-Place result selects or creates its exact origin and destination
  and always establishes one new Connection. It never moves the Character.
- Before travel, read the Character and exact Connection, preview either full
  arrival or one forward course stop, confirm, then call `move_character` once.
  Re-read accepted Character and Activity before narrating arrival or progress.
- Retry an uncertain start with the same start id; it returns the same stored
  outcome. Retry an uncertain discovery with the same `request_id`, the same
  `attempt_id` and the same content.
- A successful `submit_discovery` is the first moment the result is
  shared World state. Never expose attempt ids, chance thresholds or
  admission mechanics.
