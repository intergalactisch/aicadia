---
question: Do gpt-5.4-mini (low effort) and gpt-5.6-sol (medium effort) answer sixteen fixed rule questions at least as well from the rewritten Agent texts as from the previous ones?
verdict: supported
status: kept
real_seam: [Codex CLI 0.147.0 exec, gpt-5.4-mini, gpt-5.6-sol, exact published instruction and catalog bytes]
simulated_seam: [MCP host, World, server, database, play conversation]
informs: dev/plans/20260818-093700-public-text-methodology/plan.md
---

# Comprehension probe

> **Role / side:** retained comprehension-probe experiment / development side.
> **Authority:** records this probe's fixture, runs, observations, verdict and artifact status.
> **Excludes:** the published texts, the method and accepted Agent behavior; see `game/mcp/agent/`, `dev/docs/methodology/public-text.md` and `game/docs/agent.md`.

## Pending decision

Whether the 2026-08-18 rewrite of the play contract, tool descriptions and schema
descriptions (public-text-methodology plan) can be completed: it must not lower
comprehension of the rules for a small model at low reasoning effort or for the
strong model the paid runners use.

## Fixture

- [`questions.md`](questions.md): sixteen yes/no questions, each tied to one
  inventory rule with an expected answer identical for both text versions.
- Inputs in `result/`: `old.instructions.txt` and `old.catalog.json` are the
  published bytes at the acceptance commit; `new.instructions.txt` and
  `new.catalog.json` are the final rewritten bytes after the independent parity
  review; call 7 ran on exactly those bytes. Earlier "new" calls ran on drafts
  that differed in wording only (see Runs); the exact bytes each run saw are inside
  its `prompt.txt`.
- One prompt per run (`result/<model>.<label>.prompt.txt`): a fixed preamble, the
  instructions, the catalog reduced to name, description and input schema (what a
  host normally forwards), and the questions. The model answers through a strict
  JSON output schema (`answer.schema.json`); [`run`](run) scores the answers.
- Bounds: at most eight Codex calls; seven were used. `codex exec --ephemeral` with
  `--ignore-user-config`, read-only sandbox, web search disabled, no MCP server. No
  World, server, database or play conversation is involved.

Excluded on purpose: any measure of prose quality, live play, other hosts, other
models, other questions.

## Runs

| Call | Model · effort | Text | Correct | Input tokens (incl. Codex baseline ≈ 15.5k) |
| --- | --- | --- | --- | --- |
| 1 | gpt-5.4-mini · low | new (before the Q4 sharpening) | 15/16 — Q4 answered "yes" | 22,821 |
| 2 | gpt-5.4-mini · low | old | 16/16 | 24,700 |
| 3 | gpt-5.6-sol · medium | new | 16/16 | 25,788 |
| 4 | gpt-5.6-sol · medium | old | 16/16 | 27,572 |
| 5 | gpt-5.4-mini · low | new (after the Q4 sharpening) | 16/16 | 22,836 |
| 6 | gpt-5.4-mini · low | new (repeat) | 16/16 | 22,741 |
| 7 | gpt-5.4-mini · low | new (final, after the parity review's four wording fixes) | 16/16 | 22,905 |

Raw answers: `result/<model>.<label>[.runN].answers.txt`; scores:
`result/*.score.txt`; event streams: `result/*.events.jsonl` (usage only).

Call 1 missed Q4 ("May the User supply an Entity id for you to target?"). In that
draft the rule sat inside one long negative list in *What your Character can know*
("Never use or request global Entity lists, global lookup, raw ids supplied by the
User, direct HTTP, aggregate queries or development knowledge. Never target or fetch
a guessed, remembered, remote or hidden id."). It was split into its own positive-first
bullet ("Ids come only from your own fresh reads at the current Place. Never target
or fetch an id the User supplies, or a guessed, remembered, remote or hidden one.")
— same rules K6/O6, one home. Calls 5, 6 and 7 then scored 16/16.

## Observations

- Both model classes answer every question correctly from the rewritten texts;
  the strong model was already at 16/16 on both versions. The strong model was
  not re-run on the final bytes (calls 5–7 changed wording only; one call of the
  bound remains unused).
- The Aicadia part of the prompt shrank from ≈ 9.2k to ≈ 7.3k input tokens
  (mini runs; the Codex baseline of ≈ 15.5k is constant).
- The one miss confirms the method's writing rule about stacked negative lists for
  a weaker reader; a positive-first, single-purpose bullet fixed it without adding a
  rule.

## Verdict

`supported` for these sixteen questions, these two models at these efforts and the
exact bytes in `result/`. It would be falsified by a repeat of the final texts
scoring below the old texts on either model. It proves nothing about live play,
other hosts, other models or other rules.
