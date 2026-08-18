What it does:
asks the World to admit and resolve one investigation at the Character's exact current Place. It creates no Entity, Activity or state and returns only a stable zero or positive outcome with its one-result limit.

Use it when:
the grounded situation makes a deliberate search of this place the Character's next intelligent step.

Before you call:
read get_world, get_character, both current-Place lists and any relevant local Entity state, then decide from those facts. Starting is free of confirmation; the User may advise but supplies no mechanics.

Input meaning:
one fresh private request_id and nothing else — no focus, prose, Place, Character, odds, seed or result count.

After the call:
on zero, describe one honest unsuccessful search, stop that attempt and submit no discovery; a later search uses a new request_id. On positive, re-read the exact current Place, relevant local state and recent Activity before authoring exactly one found Entity within the returned limit; the result is permission, not context.

On failure:
nothing was admitted, rolled or changed — re-orient and continue play. Retry an uncertain delivery only with the same request_id; it returns the same stored outcome without another roll.

Never:
expose an id, odds, thresholds or admission mechanics in player conversation; present a positive as a found thing before a confirmed submit_discovery; or trigger another Agent, notification or background process.
