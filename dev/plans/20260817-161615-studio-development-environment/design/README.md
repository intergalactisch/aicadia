# Aicadia Studio design system (T4.0)

This folder is the one Studio design accepted under D11a/D11b: `studio.css` is the
stylesheet T4 moves to `studio/web/studio.css`, and the four reference pages show how the
classes compose on real content. Reference pages are plan artifacts; they are never
served. Open them locally: `open dev/plans/20260817-161615-studio-development-environment/design/*.html`.

## Intent

The developer sits down after a build session to step back. Three kinds of content
meet on one page: the environment (navigation, meta, actions), the world's contracts
(long English prose that must read like a document) and the world's facts (ids,
columns, rows). The interface is the map room of a world under construction — paper,
ink, hairlines, one compass-blue accent, status shown as small seals. Light mode
only; calm, clean, typography first; color only for status.

## Tokens (`:root`)

- Paper and ink: `--paper` page, `--paper-deep` sidebar/plates/code, `--vellum`
  hover/selected, `--rule` hairline, `--rule-strong` inputs/emphasis; `--ink`,
  `--ink-soft`, `--ink-mute`, `--ink-faint` text hierarchy.
- The one accent: `--compass` (links, current marker, focus ring, Live seal),
  `--compass-deep`, `--compass-wash`.
- Seals (status only, never decoration): `--seal-moss` accepted / current / kept /
  supported / done / complete / connected; `--seal-amber` open / draft / pending /
  proposed / user direction / warning; `--seal-brick` rejected / refuted / blocked /
  dropped / error; `--seal-slate` superseded / historical / researched / frozen /
  archive. Each has a `-wash` for notes.
- Voices: `--voice-ui` (system sans; navigation, labels, tables, notes),
  `--voice-contract` (Charter/Iowan/Georgia serif; page titles, record prose, ledger
  text, lede), `--voice-fact` (system mono; ids, paths, columns, code, references).
- Scale: 4px base; radius 4px (inputs 6px); bar 52px; tree 260px; related 288px;
  prose measure 72ch. Depth is borders-only; the only elevation is contrast.

## Page anatomy (every page)

1. `.bar` — wordmark, `.sections` (Overview · Game · Development · Live; current has
   an underline in compass), `.jump` (server-side jump box, `/` focuses it),
   `.pulse` (connection: database name and read time), Refresh. `.menu-toggle` on
   narrow screens.
2. `.tree` — the complete, stable index of the current section: `.tree-title`
   (side + section name), `.tree-group` with `.tree-label` small caps and lists;
   `aria-current="page"` marks the current page (vellum + 2px compass rule);
   optional `<small>` counts; nested lists for tables/experiments; `<details>` only
   for long leaf lists.
3. `.main` — `.crumbs` breadcrumb, `.head` (`.seals`, `h1`, `.lede`), `.plate` (the
   authority plate: home, path/id, tables/dates, referenced-by, actions and the
   italic Authority sentence), `.body` = `.content` + `.related`.
4. `.related` — `.panel`s with small-caps titles: vocabulary, outline, storage,
   referenced-by, provenance, contract links; sticky on wide screens, flows below
   content under 1240px.
5. `.colophon` — the copyable reference line.

## Components

- Prose records: `.prose` (serif 17/1.65, h2/h3 hierarchy, `pre`, tables, links
  underlined in `--rule-strong`, `a.anchor` on hover).
- Sections in data pages: `.section > h2` (serif 20) with optional `.count`,
  `.section-note`, `.subhead` for one table inside a section.
- Facts: `.data` tables (13.5px, small-caps headers, hairline rows, hover paper-deep,
  `.fact` mono cells, `.num` right-aligned, `.mute`), `.id` with hidden copy button,
  `.data-note`, `.toolbar` with `input[type=search]` and `.chips`.
- Meta: `.meta` definition grid; `.seal`/`.seal-*` status; `.stamp` for stored roles
  and qualifiers; `.btn`, `.btn-quiet`, `.btn-primary`, `.btn-small`.
- Ledger (decisions): `.ledger` two columns (sticky `.ledger-date`, `.ledger-day`
  with `h3` topics and `.entry` = tag column + serif text + `.entry-foot`); `:target`
  highlights a linked entry.
- Board (plans): `.board` of four `.lane`s with `.card`s (`b` task id in mono
  compass).
- Overview: `.state` sections with small-caps `h2`, `.count-row` figures,
  `.list` rows with trailing `<small>` context.
- States: `.note` (+ `-warn`, `-error`, `-ok`), `.empty-state`, `.truncated`,
  `.pulse[data-state]`.

## Behavior expectations for T4

- Every page is a stable path; the tree and breadcrumb are rendered server-side from
  the projection; nothing in the browser holds route state.
- One small enhancement script: `/` focuses the jump box, Escape blurs it; copy
  buttons write the reference/id/path and toast; `.menu-toggle` toggles
  `.tree.is-open` and `aria-expanded`; loaded-row filter inputs hide non-matching
  rows client-side and say so.
- Reduced motion respected; focus ring is the compass shadow; contrast ≥ AA for all
  text tokens on paper.

## Checks run on the reference pages

Desktop 1440px and 390px screenshots of all four pages: no horizontal overflow,
hierarchy survives the squint test, hairlines recede, seals are the only color
besides links, the plate and reference line appear on every page.
