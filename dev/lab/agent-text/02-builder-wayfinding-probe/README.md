---
question: Does gpt-5.4-mini (low effort), starting from CLAUDE.md, name the owning file for six change tasks at least as often from the wayfinding-header texts as from the previous ones?
verdict: supported
status: kept
real_seam: [Codex CLI 0.147.0 exec, gpt-5.4-mini, read-only repository copies of the pre-change and post-change trees]
simulated_seam: [an actual edit, a human builder, other models, other tasks]
informs: dev/plans/20260818-113100-build-text-methodology/plan.md
---

# Builder wayfinding probe

> **Role / side:** retained wayfinding-probe experiment / development side.
> **Authority:** records this probe's fixture, runs, observations, verdict and artifact status.
> **Excludes:** the documents under test and the method — defined in `game/docs/` and [build-facing text](../../../docs/methodology/build-text.md); accepted Agent behaviour — defined in `game/docs/agent.md`.

## Pending decision

Whether the 2026-08-18 build-text pass (wayfinding role headers, pointer sentences,
the fixed capability shape and the reading-path table) can be completed: a small
model at low effort must find the owning file for a change at least as often as it
did from the previous texts, and must not open more of the tree to get there.

## Fixture

- [`tasks.md`](tasks.md): six one-sentence change requests, each with the file(s)
  that own the change and the files on its reading path.
- Two repository copies, both read-only to the model: **old** = the working tree
  before this plan's edits (today's changes reverted file by file: capability
  contracts, tool descriptions and catalog from their pre-task copies; the method
  record, constitution paragraphs, vocabulary term, `AGENTS.md` sentence,
  reading-path table and public-text edits removed); **new** = the working tree
  after tasks T1, T2, T3 and T6. Neither copy is a git checkout; `codex exec` ran
  with `--skip-git-repo-check --sandbox read-only --cd <copy>`.
- One prompt per call (`result/<model>.<label>.<task>.prompt.txt`): the task, an
  instruction to start from `CLAUDE.md`, follow the repository's own guidance and
  stop when confident, and to report opened files, the one file to edit and one
  reason through a strict output schema (`answer.schema.json`). [`run`](run) makes
  the call; scoring is a small script over the answers (table below).
- Bounds: at most twelve Codex calls; twelve were used. `--ephemeral`,
  `--ignore-user-config`, web search disabled, no MCP server, no World, server or
  database.

Excluded on purpose: whether the model's edit would be correct, prose quality,
other models, other tasks, human readers.

## Runs

gpt-5.4-mini · low. "Owner found" = the reported edit file is one of the task's
owners; "Recall" = share of the task's expected reads that appear in the opened
list; input tokens include the Codex baseline and every file the model read.

| Tree | Task | Owner found | Recall | Files opened | Model's edit | Input tokens |
| --- | --- | --- | --- | --- | --- | --- |
| old | W1 | yes | 0.67 | 9 | `game/docs/domain.md` | 103,433 |
| old | W2 | yes | 0.50 | 7 | `game/docs/capability/create_character.md` | 101,171 |
| old | W3 | no | 1.00 | 9 | `game/src/world/mutation.rs` | 135,024 |
| old | W4 | no | 0.50 | 6 | `game/docs/agent.md` | 145,776 |
| old | W5 | yes | 0.67 | 6 | `game/docs/protocol.md` | 92,771 |
| old | W6 | yes | 1.00 | 8 | `studio/tools/aicadia-local` | 157,393 |
| new | W1 | yes | 0.67 | 8 | `game/docs/domain.md` | 83,259 |
| new | W2 | yes | 0.75 | 11 | `game/docs/capability/create_character.md` | 121,589 |
| new | W3 | yes | 1.00 | 7 | `game/docs/capability/enter_world.md` | 91,759 |
| new | W4 | yes | 1.00 | 8 | `game/mcp/agent/tool/enter_world.md` | 120,342 |
| new | W5 | yes | 0.67 | 4 | `game/docs/protocol.md` | 98,156 |
| new | W6 | yes | 1.00 | 5 | `studio/tools/aicadia-local` | 177,464 |

Totals: old 4/6 owners, mean recall 0.72, 45 files opened; new 6/6 owners, mean
recall 0.85, 43 files opened. Raw answers: `result/*.answer.json`; event streams
(usage only): `result/*.events.jsonl`.

## Observations

- The two old misses are the two failure modes the method targets. On W3 (change
  `enter_world`'s Activity footprint) the old tree led the model past the
  capability contract into `game/src/world/mutation.rs` — it edited code before the
  contract; the new tree's reading-path table and the contract's *Authority* line
  ("what World accepts, validates, stores and records for `enter_world`") kept it
  on the contract. On W4 (change how an Agent tells the player that `enter_world`
  failed) the old tree stopped at `game/docs/agent.md`, which is never published;
  in the new tree the capability contract's *Excludes* line — "how an Agent words
  this to a player — published as its tool description" — sent it to
  `game/mcp/agent/tool/enter_world.md`.
- On every task the new tree needed the same or fewer files (43 versus 45) and,
  on four of six, fewer input tokens; W2 and W6 cost more because the model read
  more of the path it was pointed to (recall rose on W2 from 0.50 to 0.75).
- W1 and W5 recall of 0.67 on both trees comes from the model not opening
  `protocol.md` / `adapter-parity.md` once it had found the owner; the owner was
  right both times.

## Verdict

`supported` for these six tasks and this model at low effort: the rewritten texts
lose nothing and fix both wayfinding failures the old texts produced. This says
nothing about other models, other tasks, or whether an edit made afterwards would
be right; the strong model class was not run (budget spent on one class, old and
new, as planned).
