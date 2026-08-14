# World-entry evidence

> **Role / side:** World-entry delivery and evidence history / evidence bridge.
> **Authority:** owns the retained deterministic and live proof for Agent World entry.
> **Excludes:** current entry behavior and runner operations; see `docs/game/` and `runner/agent-playtest.md`.

## Deterministic completion record

- `migration/0004_world_entry_activity.sql`, `src/world/`, `src/wire/` and
  `src/server.rs` implement the accepted state, transaction and adapter contract.
- `tests/world/` proves concurrency, retry safety, rollback, immutability, the
  exact migration boundary, historical Place retention, authorization and stable
  pagination; `tests/server/` proves HTTP concurrency, HTTP/MCP semantic parity
  and the complete ten-capability catalog fixture.
- `docs/game/`, `CONTEXT.md`, `docs/concept/log/README.md`, `AGENTS.md` and the Agent
  playtest contract agreed on the current behavior and nomenclature at completion.

## Retained live result

Generated MCP descriptions, server instructions and output schemas carried the
entry and Activity meanings; the checked-in ten-tool fixture pinned them. The
expanded runner and fake integration suite proved its exact two-Agent orchestration,
expected genesis-error handling, authoritative post-run state validation,
fail-closed evidence and cleanup paths.

`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, all 38 Rust tests,
the Bash syntax checks, `bash tests/agent-playtest.sh`, `git diff --check` and the real
token-free Codex/PostgreSQL preflight passed on 2026-08-09. Explicitly authorized
live run `run-9TOG5yrJ` passed on 2026-08-10: two distinct Characters entered the
same entry Place, both personal Activity proofs matched authoritative HTTP state,
Agent B observed Agent A's exact shared Entity, and the disposable World was dropped.
That observed shared Entity was unplaced.

This narrower entry proof remains separate from the later resumed-Action evidence.
