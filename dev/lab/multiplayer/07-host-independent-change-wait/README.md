---
question: Can terminal, browser and one explicitly active MCP Agent share one bounded version-aware stale hint and authoritative reread contract without automatic Agent invocation or durable recipient state?
verdict: inconclusive
status: kept
real_seam: [Rust compiler and tests, in-memory bounded wait and race injection, loopback HTTP, official rmcp 3.1.1 client and server, MCP 2026-07-28 tool calls, real Codex in-app browser, terminal client, 10,000 logical waiters, Codex CLI 0.149.0, gpt-5.6-sol high inference, one-run guard, captured tool calls, controller readback, process cleanup]
simulated_seam: [World and Entity persistence, authorization, deployed HTTPS, ChatGPT and Claude embedding, PostgreSQL, multi-instance fan-out, broker behavior, network partitions, concurrent production Users, million-connection capacity]
informs: dev/plans/20260821-144813-host-independent-change-wait/plan.md
---

# Host-independent bounded change wait

> **Role / side:** retained Multiplayer transport experiment / development side.
> **Authority:** records this lab's question, fixture, method, real and simulated
> seams, observations and bounded verdict.
> **Excludes:** product decisions, production implementation and delivery evidence;
> see the accepted plan, `game/docs/` and `dev/docs/evidence/`.

## Question

Can one bounded version-aware wait provide the same stale-hint and authoritative
reread behavior to a terminal client, portable browser component and one explicitly
active MCP Agent without automatic model invocation or durable recipient state?

The accepted execution and evidence boundary lives in
[`plan.md`](../../../plans/20260821-144813-host-independent-change-wait/plan.md).

## Fixture

One simulated subject is a brown wooden Table at version `1`. A secret lab controller
may change it exactly once to a blue wooden Table at version `2`. A wait result
contains only the changed subject identity; every client must reread current state.

## Verdict

The local change-wait mechanism is supported for the exact deterministic, HTTP,
official MCP-client and real-browser seams exercised here. The real pinned Codex
Agent route is **inconclusive**: its tool contract required an exact subject on the
very first read, but supplied no operation that could discover that subject. The
Agent therefore never reached the wait being tested.

This is not evidence against a bounded waiting tool. It is evidence that an Agent
operation may never require a hidden identifier as the prerequisite for discovering
that same identifier. A wait subject and known version must come directly from a
successful preceding World read or another bounded context result.

## Observations

### Token-free mechanism

- Eleven Rust tests passed, including prior change, post-registration change,
  forced commit-at-registration, timeout, reconnect with an old version, coalesced
  hints, invalid bounds and guarded one-way mutation.
- The HTTP terminal adapter and official `rmcp 3.1.1` client both performed one
  version-`1` read, one bounded wait, one content-free changed-subject result and one
  authoritative version-`2` reread.
- `10,000` logical waiters on one hot subject all recovered from one transition.
  The fixture stored no recipient records and ended with zero active waiters. This
  is a local algorithm pressure test, not a production capacity claim.
- The real browser rendered `waiting` with the brown Table at version `1`, then
  rendered `changed and reread` with the blue Table at version `2`. The first
  five-second ceiling proved too small for browser-tool overhead; a thirty-second
  ceiling still returned immediately after the actual change.

### Sole live Agent run

- The persistent guard admitted exactly one `codex-cli 0.149.0` process with
  `gpt-5.6-sol` high, zero retries and a 120-second deadline. Cleanup passed.
- The natural User prompt revealed neither the future colour nor version.
- `get_state` required `subject = "lab://entity/table"`, but neither its schema nor
  another tool exposed that value before a successful read. In 121 seconds the
  Agent issued 583 unsuccessful `get_state` calls with guesses such as `Table`,
  `houten tafel` and `wooden-table`.
- The Agent never completed an initial read, never called `wait_for_change`, and
  produced no final answer. Consequently terminal and browser live waits were not
  started, the controller correctly made no transition, and independent readback
  remained the brown Table at version `1`.
- Because the configured host never exercised the candidate wait, classifying it as
  supported or refuted would overstate the evidence. The retained manifest records
  `inconclusive`.

## Implications

1. The candidate semantic contract remains small: bounded exact subjects plus known
   versions in, changed subject identities or timeout out, then authoritative reread.
2. Subject selection must be grounded by the World surface that precedes the wait.
   For one already selected Entity, a read can need no repeated identifier; for
   broader play, a bounded authorized context read must return the exact references
   that a later wait accepts. Name guessing is never a valid discovery mechanism.
3. The World still needs no semantic inference, per-recipient truth or automatic
   Agent invocation. The failed run occurred before those boundaries were tested.
4. A later real-host smoke would need a discoverable read→wait contract and fresh
   authority. This lab's zero-retry budget is consumed and is not silently renewed.

## Evidence boundary

Real seams are the standalone Rust wait function, race injection, loopback HTTP,
official MCP client, real in-app browser, guarded Codex process, captured tool calls
and independent controller readback. Simulated seams are World persistence,
authorization, deployed HTTPS, ChatGPT and Claude embedding, multi-instance fan-out,
PostgreSQL contention, broker behavior and million-connection capacity.

Retained evidence lives in [`result/`](result/). The accepted execution boundary is
the linked plan; this verdict changes no current game contract or production code.
