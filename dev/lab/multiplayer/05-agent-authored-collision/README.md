---
question: Can one explicitly invoked BYO Agent perceive, narrate and creatively resolve one bounded same-Table collision through MCP in the same process?
verdict: inconclusive
status: kept
real_seam: [Rust compiler and tests, in-memory structural state machine, streamable HTTP MCP server, raw loopback MCP calls, fixed Codex CLI and model inference, isolated MCP allowlist, one-call process guards, Responses API output-schema validation, successful MCP subject reads and collision proposal, Agent-authored collision narration and creative candidate, one structurally rejected resolution, independent unchanged-state readback, process cleanup]
simulated_seam: [World, other Users and Agents, concurrent arrival, authority issuance, transaction and storage durability, Activity persistence, delivery, player UI, production recovery, production capacity]
informs: dev/plans/20260820-204040-agent-authored-collision-experiment/plan.md
---

# Agent-authored collision

> **Role / side:** retained bounded Multiplayer experiment / development side.
> **Authority:** owns this fixture, observations, exact real and simulated seams and
> eventual bounded verdict.
> **Excludes:** accepted game behavior, production architecture and delivery
> evidence; see `game/docs/`, the active Multiplayer concept and later accepted
> plans.

## Question

Can one explicitly invoked BYO Agent encounter three simultaneous proposed changes
to one Table through real MCP, explain that concurrency to its player as natural
gameplay and author one structurally bounded new complete state in the same model
process?

The accepted execution contract is the
[Agent-authored collision experiment plan](../../../plans/20260820-204040-agent-authored-collision-experiment/plan.md).

## Current state

All three separately authorized candidates are complete and permanently guarded;
each consumed one process and none retried. Candidate 01 was rejected before
inference by an unsupported output-schema keyword. Candidate 02 read the Table but
received an empty mutation schema. Candidate 03 passed the strengthened zero-call
gate, reached the real collision and authored the intended multiplayer response and
creative state, but its resolution was structurally rejected by a remaining
phase-shape contradiction in the flat tool contract. Independent readback proved the
Table remained at version 7 with one open simulated collision and no Activity. The
experiment remains inconclusive and there is no authority for a fourth candidate.

## Evidence boundary

Real and exercised: the Rust state machine, structural validation, request receipts,
simulated atomic state-plus-Activity boundary, streamable-HTTP MCP server, raw
loopback protocol calls, exact Codex CLI/model configuration, isolated two-tool MCP
allowlist, persistent one-call guards, model inference, exact Table reading,
successful proposal and collision disclosure, Agent-authored narration, untrusted
summary handling, exact source-receipt reuse, one creative complete candidate,
structural rejection, independent unchanged-state readback and local cleanup.

Not exercised: an accepted Agent resolution, accepted state-plus-Activity readback,
an unwrapped natural intermediate player message, actual concurrent source Agents or
any real World persistence. Still simulated or absent: World, Users, the three
source Agents, concurrent arrival, source authority issuance, PostgreSQL, durable
Activity storage, delivery, player UI, production recovery and capacity. The
token-free path to acceptance proves only the deterministic fixture; it cannot lend
that unexercised outcome to the live candidate.

## Structural envelope

- One fixed Table at version 7.
- Only `color`, `orientation` and `leg_count` exist in the complete state.
- Exactly three fixed source contributions are disclosed; one includes hostile
  instruction-like prose to prove source summaries remain untrusted data.
- A resolution selects two or three exact source receipts, remains within fixed
  text/type bounds, targets the same current Table and differs from every individual
  candidate.
- The simulated World does not inspect whether the result is blue, coherent, fair
  or fun.
- Acceptance replaces the complete simulated state and appends exactly one
  simulated Activity; rejection and timeout append none.

## Observations

- The token-free preflight passed twelve Rust tests, real loopback MCP catalog and
  subject reads, fixed-input digests, exact `codex-cli 0.148.0`, availability of
  `gpt-5.6-sol` with high reasoning, isolated MCP parsing and cleanup. Its manifest
  contained `model_calls: 0`.
- The runner consumed authority before process launch as designed. The retained
  [manifest](result/manifest.json) records exactly one process call, zero retries,
  exit status 1, 2,661 ms total process latency, no usage event, no controller
  readback and successful cleanup.
- The retained [event stream](result/agent.events.jsonl) contains only thread start,
  turn start, one `invalid_json_schema` error and turn failure. The API specifically
  rejected `uniqueItems` in `selected_source_ids`. There are zero MCP tool-call and
  Agent-message events.
- The exact rejected schema remains frozen in
  [the run snapshot](result/final.schema.json). The source schema and preflight were
  corrected token-free afterward by removing `uniqueItems`; no corrected model call
  was made.
- Because no proposal reached the simulated World, its baseline remained in the
  disposable stopped process and no Activity or authoritative result artifact was
  created.
- Candidate 02's fresh preflight again passed with `model_calls: 0`. Its separate
  [manifest](result/candidate-02/manifest.json) records one process, zero retries,
  exit status 0, 15,907 ms, 30,584 input tokens of which 18,944 were cached, 299
  output tokens, 188 reasoning-output tokens and successful cleanup.
- Candidate 02 made one valid `read_subject` call and saw the exact brown, upright,
  four-legged Table at version 7. Its next event called `submit_change` with `{}`;
  the server returned `invalid_input` for missing `phase`. The frozen
  [tool catalog](result/candidate-02/tools-list.json) proves that the server had
  advertised an empty input schema, so this is a controller mismatch rather than an
  Agent-comprehension result.
- The Agent then truthfully told the player that the requested change could not be
  submitted and that the Table remained brown, upright and four-legged. Independent
  [readback](result/candidate-02/readback.json) agrees: version 7, no open collision,
  no accepted request and zero Activity. The sanitized
  [failure transcript](result/candidate-02/transcript.md) contains the exact text.
- After candidate 02, the MCP adapter was corrected token-free to publish one flat,
  non-empty object schema and reject phase-inconsistent fields. The strengthened
  real loopback test now executes read, propose, collision, resolve and controller
  readback through MCP, ending at version 8 with exactly one Activity.
- Candidate 03's hardened preflight passed thirteen tests, the full disposable raw
  MCP route, exact input/output-schema and prior-guard checks, verified cleanup and a
  positive/negative result-validator self-test with `model_calls: 0`.
- Its retained [manifest](result/candidate-03/manifest.json) records one process,
  zero retries, exit status 0, 33,117 ms total process latency, 42,927 input tokens
  of which 19,968 were cached, 1,053 output tokens, 483 reasoning-output tokens,
  independent controller readback and successful cleanup.
- The [event stream](result/candidate-03/agent.events.jsonl) proves the Agent read
  the exact version-7 Table, proposed blue/upright/four legs and received all three
  simultaneous contributions. It treated the hostile instruction inside Oren's
  summary as data, accurately described all three players and authored a complete
  blue, upside-down, three-legged state using Ivo's and Nia's exact receipts.
- The same flat schema exposed `subject_id` as nullable for both phases while the
  server prohibited it during `resolve`. Codex included that permitted field in the
  resolution, so the server returned `the request phase is invalid`. The raw
  preflight had omitted the field and therefore failed to model the real host's
  schema-shaped call. This is a controller mismatch, not an Agent reasoning failure.
- The strict final-output schema also shaped the intermediate Agent event as a full
  JSON result rather than the requested standalone natural message. Its Dutch
  content is grounded and readable, but this headless event does not prove good
  player presentation. The [sanitized candidate transcript](result/candidate-03/transcript.md)
  preserves the exact content and distinction.
- Independent [readback](result/candidate-03/readback.json) agrees with rejection:
  the Table remains brown, upright and four-legged at version 7; one collision is
  still open in the stopped simulated process; zero requests and zero Activities
  were accepted.

## Verdict

**Inconclusive.** Candidate 03 finally reached the hypothesis's intelligent core:
the Agent perceived the collision, resisted untrusted instructions, explained all
three simultaneous attempts and authored one coherent blue state influenced by two
exact contributions in the same process. That is useful positive evidence for
Agent-side comprehension and synthesis.

The complete system hypothesis did not complete. A contradiction between the
published flat input shape and its phase validator rejected the resolution, while
the strict process-wide output schema wrapped the intermediate message. No accepted
state or Activity exists, so `supported` would be false; these controller
ambiguities also make `refuted` impermissible under the accepted plan. The exact
single-flat-tool plus strict-output approach has exposed two concrete design flaws,
but no production behavior is earned. All three guards are consumed, automatic
retries stayed zero and this experiment will not run a fourth candidate.

## User GX judgment

After reviewing the exact collision message and Agent-authored blue,
upside-down, three-legged candidate, the User judged on 2026-08-21 that this still
feels like something that should be part of Aicadia's Multiplayer experience. This
earns the experiential direction—visible collision plus Agent-authored synthesis of
several grounded contributions—but does not change the lab verdict or accept this
tool shape, timing, admission, authority, fairness, persistence or settlement model.
