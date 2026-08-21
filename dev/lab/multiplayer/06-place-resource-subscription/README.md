---
question: Can one pinned real Codex host follow one exact MCP resource, receive its update hint, reread current state and expose the changed fact to its already active Agent?
verdict: refuted
status: kept
real_seam: [rmcp 3.1.1 client and server, MCP 2026-07-28 exact-resource listen and notification, Streamable HTTP, reconnect, Codex CLI 0.149.0, gpt-5.6-sol high inference, isolated single-server configuration, captured host requests, one-run guard, controller readback, process cleanup]
simulated_seam: [World, Character, Place and Entity persistence, authorization, movement, multi-instance fan-out, player UI, concurrent Users, production recovery, production capacity]
informs: dev/plans/20260821-133052-place-resource-subscription-smoke/plan.md
---

# Exact-resource subscription

> **Role / side:** retained Multiplayer transport experiment / development side.
> **Authority:** owns this experiment's fixture, observations, real and simulated
> seams and bounded verdict.
> **Excludes:** accepted Multiplayer behavior, final spatial attention anchor,
> production architecture and delivery evidence; see the accepted plan,
> `dev/areas/multiplayer/README.md`, `game/docs/` and `dev/docs/evidence/`.

## Question

Can the official Rust MCP client and then one pinned real Codex host follow one exact
resource through `subscriptions/listen`, receive a content-free update, reread
authoritative current state and expose the changed fact to an already active Agent?

The Place is only a concrete fixture. This lab cannot establish that Place is the
final Aicadia attention anchor.

## Bounds

- one exact resource URI;
- one brown-to-blue state transition per fresh fixture;
- two token-free deterministic scenarios;
- at most one separately guarded Codex Agent run and zero retries;
- 120 seconds maximum for the live process; and
- no production World, database, auth, movement, multi-Agent, scale or UI claim.

## Evidence state

T1 is complete: four token-free Rust tests and the retained preflight prove exact
listen, content-free notification, authoritative reread and reconnect recovery with
zero Agent processes. The one authorized live run is also complete: it consumed one
Codex process, made zero retries, exited normally after 11 seconds and cleaned up
every owned process and temporary directory.

## Observations

- The retained [token-free preflight](result/token-free-preflight.json) used the
  official Rust SDK on both ends. Its connected case acknowledged the exact URI,
  read brown revision `1`, received one content-free update, then read blue revision
  `2`. Its reconnect case recovered blue revision `2` without notification replay.
- The live [manifest](result/live/manifest.json) records `codex-cli 0.149.0`,
  `gpt-5.6-sol` with high reasoning, one Agent process, zero retries, normal exit,
  no timeout and successful cleanup. The persistent
  [live guard](result/live.guard/consumed.json) remains consumed.
- The complete [server observations](result/live/observations.json) contain only
  `server/discover` and `tools/list`. Codex never listed or read the resource, never
  opened `subscriptions/listen` and therefore gave the controller no initial-read
  condition on which to perform the change.
- Independent [controller readback](result/live/controller-readback.json) remained
  the brown Table at revision `1`. This confirms that the controller correctly did
  not manufacture the expected change when Codex skipped the resource path.
- The Agent nevertheless repeated the blue revision-`2` sentence from the fixed
  output instruction in its [final transcript](result/live/agent.final.txt). That
  line is explicitly not grounding evidence: no World read supports it. The event
  stream records 9,191 input tokens, 228 output tokens and 199 reasoning-output
  tokens for the sole run.

## Verdict

**Refuted for this pinned host path.** MCP exact-resource subscription itself works
in the deterministic SDK client, but the isolated Codex host connected only to
discover the server and list tools. It did not expose, read or follow the exact
resource during the already active turn. The accepted `refuted` criterion is met:
Codex connected successfully, the controller and process remained healthy, and the
complete listen→notification→reread→Agent route was absent when the task ended.

This does not refute realtime Multiplayer attention, MCP subscriptions, other Codex
versions or another BYO host. It does refute treating standard MCP resources as the
universal default carrier for Aicadia on the tested Codex version. A later design
must make realtime awareness an explicit host/application capability and retain
authoritative reread as correctness. If an active Agent must react in the same turn,
the next bounded experiment should compare a normal MCP tool call that waits for one
eligible change with application-owned player presentation; neither route is chosen
by this lab.

## Real and simulated seams

The real deterministic seams are `rmcp 3.1.1`, exact-resource discovery, read,
subscription acknowledgment, notification, reread, cancellation and Streamable
HTTP reconnect. The real live seam adds Codex CLI `0.149.0`, the pinned model,
isolated server configuration, captured host requests, Agent output, fixed-input
digests, one-run guard and independent controller state.

World, Character, Place and Entity persistence, authorization, movement,
multi-instance fan-out, player UI, concurrent Users and million-connection capacity
are simulated or absent.
