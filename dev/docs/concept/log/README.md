# Concept decision register

> **Role / side:** index and maintenance contract for the concept decision register / development side.
> **Authority:** period navigation, tag vocabulary and append-only recording rules.
> **Excludes:** Current game truth, concept rationale, research findings and detailed delivery evidence.

This per-month register is the record of concept development—not the source of
truth, but the trail. Current behavior lives in `game/docs/`; current concept
rationale lives in the themed records one directory above.

---

## Periods

- [July 2026](2026-07.md) — frozen July generation and its original citations.
- [August 2026](2026-08.md) — current append-only decision register.

## Tag vocabulary

Prefixes state what happened to a choice; a qualifier after the prefix names its
scope. The consolidated vocabulary is:

- **decided**, **direction**, **principle**, **method**, **user direction** and
  **process decision** — calls, current thinking and build rules;
- **adopted**, **rejected**, **accepted**, **confirmed**, **resolved**, **retained**,
  **restored** and **superseded** — disposition of a proposed direction;
- **corrected**, **clarified**, **reopened**, **reframed**, **revised**, **refined**
  and **aligned** — changes or sharpening without erasing the earlier entry;
- **researched**, **explored**, **discussed**, **challenged**, **observed** and
  **identified** — information or pressure that informs but does not itself decide;
- **selected**, **planned**, **proposed**, **provisional**, **current**, **open** and
  **explicitly deferred** — forward or unresolved state;
- **implemented**, **built**, **completed**, **verified**, **closed**, **executed**,
  **authorized** and **published** — execution or evidence-boundary milestones; and
- ad-hoc scoped prefixes such as **evidence boundary**, **backlog state**,
  **anti-ceremony boundary**, **draft-blocking choices** and **next open dependency**
  keep their literal meaning. Slash combinations and arrows preserve combined or
  superseding dispositions, including `researched / 5jaar` and
  `rejected → replaced`.

## Append-only rule

Append each new choice to the current month under its real date. Never reorder,
rewrite or delete an earlier entry; record a correction, rejection or supersession
as a later entry that names the affected scope and material reason. Historical
links remain citations even when their targets later move.

Detailed delivery bookkeeping lives in [evidence](../../evidence/README.md). A new
delivery outcome contributes one decision-register line plus its evidence link;
runs, candidates, digests, audits and evolving delivery status are maintained only
in that evidence record.
