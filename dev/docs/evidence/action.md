---
status: Deterministic delivery and bounded live proof complete
---

# Character-grounded Action evidence

> **Role / side:** Action delivery and evidence history / evidence bridge.
> **Authority:** owns the retained deterministic ladder and paid trail-marker candidates.
> **Excludes:** current Action semantics and runner operations; see `game/docs/` and `runner/agent-playtest.md`.

## Deterministic ladder

The deterministic ladder passed 57 Rust tests. The fail-closed runner passed 27 fake
invocations and 19 failure modes, including duplicate or incomplete submission,
wrong actor, observer disagreement and unsafe cleanup. Public preflight pinned the
exact CLI/model/reasoning, schemas, catalog, least-privilege roles and owned database
lifecycle.

## Live evidence history

Authorized candidate `run-G8k1sTRm` ran on 2026-08-11 and was rejected by the API
with `invalid_json_schema` because `proposals.json` contained unsupported
`uniqueItems`. Rejection occurred before model execution and before any MCP tool
call. The disposable database ownership was verified, cleanup finished with
`dropped`, and no `aicadia_playtest_%` database remained. The candidate supplies no
Agent-interaction or game-outcome evidence and exhausted the sole authorization; no
automatic or currently authorized rerun exists.

Token-free correction removed `uniqueItems`, `minLength` and `maxLength` from Agent
output schemas, retained supported array cardinality and UUID format, and added the
recursive fail-closed schema-policy gate described in the runner contract.

Authorized rerun `run-nvULnvxQ` ran on 2026-08-11. Its proposal, preview and commit
phases passed. The observer's three MCP reads found the correct placed Entity and
canonical prose, but its final validation failed because the harness required an
`entity_description` field that the Place-local Entity summary tool published at
that time did not expose. The current scoped Entity output does expose description,
but that does not retroactively change this failed run. The authoritative HTTP stage
was therefore not reached. Cleanup finished with `dropped`, no
`aicadia_playtest_%` database remained, and the candidate is not complete outcome
evidence. The observer schema, prompt and validation were corrected token-free for
that catalog to require only observable Entity id, name, Place and prose; the HTTP
stage still checked the complete Entity description. It remains strong partial
evidence and is not relabelled by the later successful candidate.

## Completed resumed-action candidate

Separately authorized candidate `run-gE8iED5m` ran once on 2026-08-11 after the
frozen token-free audit returned GO. One resumed `gpt-5.6-sol` action Agent at high
reasoning:

- made the four required grounded reads once each and returned exactly three
  distinct proposals `one`, `two`, `three`;
- received selection `two` plus steering only in its first resume, produced one exact
  preview without a tool call, and received explicit confirmation only in its second
  resume; and
- made exactly one paired `submit_action` attempt/result with request UUID
  `7b2dd549-dcdf-4821-9443-24a308916611`, with no retry or other commit tool call.

World accepted Activity `81c6b9fe-ab8c-402f-ad91-af5dfffae49c`, Entity
`46c5b7d0-116a-46e2-afb2-c35866989969` and Place
`e19e9c85-c68b-4dff-a715-aebcdee749c6`. Authoritative HTTP found exactly one placed
action Entity and one `submit_action` Activity. Its actor is the independently read
action Character, and its Place, `subject` and `location` roles, full description and
canonical prose all match the confirmed preview.

Only after HTTP passed, a separate ephemeral observer made exactly the three granted
reads and copied the same Entity id/name, Place id and prose without receiving those
values out of band. Its stderr contains one non-blocking Codex cache-TTL warning;
the process exited `0`, all calls completed and its canonical output matches.

The private mode-`700` evidence directory contains forty mode-`600` artifacts. Its
manifest is a `live_candidate` with every phase and validation `passed`,
`run_status: completed` and ownership-verified cleanup `dropped`. Independent T4R4
review found no P0-P3 issue, no evidence drift, no second candidate and zero database,
process, listener or isolated-config leftovers. This proves the bounded interaction
under the pinned setup; it does not broaden the product or claim universal Agent or
prose quality.
