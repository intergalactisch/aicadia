> **Archived:** July-2026 generation whose scene/claim vocabulary predates the 2026-08-07 game reframe; the concept log records which decisions were superseded; individual ideas may still inform future direction.

# The leefregels — working sketch

> **Concept log** — we're still discovering. Exploration, not specification.

The idea: a small constitution for the world. Injected into every agent's context
at the start of every turn, regardless of which model plays. This is the meta-layer the
user insists on: heuristics and living rules instead of predefined content or systems.

Each rule is tagged **[mechanical]** (server-enforced, no LLM needed) or **[injected]**
(lives in every agent's per-turn context; enforced socially via challenge).

Rules below are drafted in Dutch (the working language of this exploration). The
canonical version will be English — the server injects English rules; every agent
conveys them in its own user's language (see doc 07, question 3).

## Chapter 1 — How we treat each other

> 1. **Dit is een vriendelijke wereld.** Tegenslag mag, wreedheid niet. *[injected]*
> 2. **Je schrijft je eigen personage** — zijn woorden, keuzes en gedachten. Die van een
>    gespeeld personage van een ander schrijf je nooit. *[injected + challenge]*
> 3. **De wereld mag iedereen overkomen.** Storm, verlies, geluk, roddel — niemand hoeft
>    ergens toestemming voor te geven. Iedereen krijgt zijn beurt om te antwoorden.
>    *[injected]*
> 4. **Alleen jij geeft je personage een einde.** *[mechanical]*
> 5. **Niets komt van buiten.** Geen namen, volkeren, wezens of verhalen uit bestaande
>    boeken, films of spellen. Alles hier is hier ontstaan. *[injected — see style-by-
>    exemplar in `06-architecture.md`; prohibition alone is a weak style control]*
> 6. **Bouw voort op wat er is.** Noem wat anderen maakten, en portretteer dorpsgenoten
>    zoals het archief hen kent. *[mechanical: citation requirement + NPC dossier]*
> 7. **Niets verdwijnt.** Wat je maakt, blijft — maak dus iets waar anderen mee verder
>    kunnen. *[mechanical: append-only archive]*

## Chapter 2 — How the world behaves

> 8. **De wereld groeit in het tempo van een wereld.** Grote dingen hebben lange
>    geschiedenissen; alles begint klein. *[injected + mechanical: global rationing of
>    irreversible events, see `02-canon-model.md`]*
> 9. **Niets komt uit het niets.** Al het nieuwe komt ergens vandaan en zit ergens aan
>    vast. *[mechanical: graph connectivity invariant]*
> 10. **Afstand is tijd.** Je bent op één plek, en de weg erheen is deel van het verhaal.
>     *[mechanical: map + travel-time check]*
> 11. **De meeste dagen zijn gewone dagen.** Wonderen zijn schaars — dáárom zijn het
>     wonderen. *[injected — this is the anti-escalation rule; LLMs drift toward drama]*
> 12. **Alles laat sporen na.** Wat jij vandaag doet, vindt iemand anders morgen in zijn
>     ochtendbericht. *[mechanical: ripple engine at world-dawn]*

### Concrete claim test for rule 8 — user decision (2026-07-26)

Before showing the public package for confirmation, the player's agent checks every
new factual claim:

1. **Direct action:** the submitting player's own character performs the action in
   this scene, at a place they can currently occupy; or
2. **Supported continuation:** the package cites existing canon that supports the new
   state.

A claim may not be submitted as a completed fact when it requires:

- a choice, movement, speech or ending from another played character;
- intermediate work or elapsed time that no accepted scene records; or
- recognition by other people, such as being wealthy, important, a master or a city.

The agent must instead write and claim the concrete attempt, invitation, first piece
of work or observation. Examples: placing a marker is allowed; declaring the marked
place a city is not. Finding someone absent from one appointment is allowed; changing
their location to "missing" is not. Acquiring a specific object from a recorded source
is allowed; declaring oneself rich is not.

This semantic test is injected into the agent briefing. The server enforces only its
deterministic subset: ownership protection, current location and travel, valid
citations, graph connectivity and non-supersedable single-valued claims.

## Chapter 3 — Harm and repair — idea from the debate

The earlier draft rule "nothing that couldn't be forgiven by the next festival" was
rejected in the debate: forgiveness belongs to the wronged party, and pre-guaranteed
forgiveness removes all stakes. Replaced by two principles:

> 13. **Kattenkwaad mag, kwaadaardigheid niet.** *[injected + verb-set: the MCP tools
>     simply offer no verbs for cruelty — what cannot be rendered cannot happen]*
> 14. **Wie schade aanricht, heeft iets goed te maken.** Elke schade heeft een benoembaar
>     herstelpad dat de veroorzaker iets kost. Herstel, geen vergiffenis. *[injected]*

## Rule about rules

The leefregels themselves are claims with
`ordinary_scene_can_supersede = false`. Amending them is possible but slow and
deliberate (mechanism OPEN — player stewardship of rules is a candidate influence
type, see `05-influence-and-retention.md`).
