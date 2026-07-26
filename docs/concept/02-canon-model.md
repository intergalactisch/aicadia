# Canon model

> **Concept log** — we're still discovering. Exploration, not specification.
> Player-facing terminology is deliberately undecided.

## Acceptance, provenance and current state — decided

An accepted scene is one immutable public source package. Acceptance means the human
confirmed the complete package and the deterministic server validation passed. It
does not certify every included proposition as universally true.

Every accepted claim is immediately queryable with:

- its stable claim id;
- the scene and actor that supplied it;
- its world-effective time;
- its agent-authored provenance; and
- any accepted source claim ids it references.

The exact fixed vocabulary and storage shape for source basis remain open. Direct
action, direct observation, memory, report/citation and interpretation must remain
distinguishable; the server must not infer the distinction from prose.

There is no universal claim evidence status and no claim promotion. A later scene
that cites, repeats, observes, contradicts or replaces an earlier claim appends its
own claims and provenance. It never mutates the earlier source package.

## Current projection selection — decided

Every current projection declares its own deterministic contract:

| Contract part | Question |
|---|---|
| key | Which claims compete for this current row? |
| authority | Which actor or applicable rule may change it? |
| time | At what world-effective time does the claim apply? |
| replace | Does an eligible later claim replace or coexist with the earlier claim? |

Every projected row retains the source claim id that produced it. A valid direct
action can therefore update current state immediately. A repeated rumor remains
queryable but cannot become physical state merely because several characters repeat
it.

`ordinary_scene_can_supersede` remains a source-package guard:

- `true`: a later claim may replace it when the applicable projection contract also
  permits that actor, time and replacement;
- `false`: an ordinary scene may not replace it. Geography, endings, epochs and the
  leefregels use this value. Their separate amendment path is still open.

Contradiction is not automatically an error. Two villages may forever preserve
different claims about the hero's birthplace. The server rejects only a claim that
attempts an unauthorized update to a mechanically exclusive current projection.
"Being misremembered" — a sourced false report that remains beside the claim selected
for current state — remains one of the world's drama engines (see doc 05).

## Independent attention — separate from truth

A later character may cite, repeat or independently observe earlier material.
Queries can expose the exact later source packages. This supports discovery,
retention and cultural continuation without a counter, rank or truth transition.

Locality may be required for a direct observation: an actor cannot observe a place or
event they could not plausibly reach. Citation and repetition do not satisfy that
observation rule.

## Verification of a discovered place — specific projection

A solo discovery creates a permanent place id and accepted discovery claims
immediately. The place appears on the map with
`verification_status = unverified`.

A second distinct character who reaches the place and submits a qualifying direct
presence or observation claim changes the place-verification projection to
`verification_status = verified`. This rule changes only the place-verification
projection. It does not change the provenance or status of the discovery's source
claims.

## Naming economy — idea from the debate (possibly the highest-leverage rule)

In every collaborative fiction wiki, inventing is cheaper than reusing — which is why
they sprawl into unread thickets. Invert it:

- **Citing existing entities is free.** Scenes that flow through existing people,
  places, dishes and words face no friction.
- **Introducing a new named entity costs a naming right** — rationed, roughly one per
  player per week, non-transferable, non-stackable beyond a small cap.
- **Phantom mentions are allowed and encouraged**: referencing something that has no
  detail yet ("the miller's estranged sister") costs nothing and creates an open hook
  another player may fulfill — the Lexicon/red-link mechanic, a coherence engine and a
  retention engine in one.

## Connectivity invariant — current direction

The knowledge graph is one connected component. Every proposal must reference at least
one existing entity; the server mechanically rejects islands. Combined with rule 9
("niets komt uit het niets"): the stranger arrives by an existing road, the new herb
grows in a known forest.

## Global rationing of the irreversible — idea from the debate

Escalation is the observed killer of shared fiction (stakes ratchet because everyone
wants to matter). The brake: irreversible claims with
`ordinary_scene_can_supersede = false` (a flood, a fire, a death of a landmark) are
**rationed globally per world-year**, not per player. One flood a year, and everyone
knows the year's allotment is spent. Cap the world, not just the person.
