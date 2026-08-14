# Trait playtest

This is the token-free controller contract for the accepted Trait capability. It is
separate from the historical trail-marker Agent runner and never relabels that
runner or its evidence as Trait evidence.

## Current token-free evidence

Run the dedicated deterministic check from the repository root:

```sh
tests/trait-playtest.sh
```

The suite invokes no Codex command, model, Aicadia server or database. It validates
the runtime-generated exact thirteen-tool catalog fixture, closed output schemas,
least-privilege role allowlists and a staged fake controller. Every fake MCP
`structured_content` result is checked against that tool's exact runtime
`outputSchema`; malformed Character, accepted-Action and Activity-page results fail
the run. Its private temporary artifacts are deleted by the test after inspection.

The staged scenario proves controller discipline rather than stochastic Agent
quality:

1. Pip's Action workshop receives only grounded reads. Selection and steering are
   withheld until a zero-tool preview phase; whole-package confirmation is withheld
   until a commit phase that exposes only `submit_action`.
2. The natural preview names Pip, his current characterization, the fact that a new
   characterization would be established and the proposed characterization. It
   never exposes an id. The accepted protocol result establishes one non-executable
   Trait and privately returns its World-assigned stable identity.
3. A later Interaction workshop reads that established current state, discovers
   Mara through compact orientation, fetches only the selected Entity and again
   withholds selection, steering and confirmation by phase.
4. The natural Interaction preview names Pip and Mara, the current and proposed
   characterization and that it develops, while keeping every id private. The
   single `submit_interaction` fake result uses the private stable selector, retains
   the previous statement and authors no response for Mara.
5. Mara's independent observer role receives compact orientation without current
   associations, derives Pip's id from that result, then obtains Pip's current Trait
   only through `get_entity_at_current_place`. Its authorized Activity read supplies
   the historical development. No observer-specific Knowledge or receipt state is
   invented.

The suite also rejects malformed MCP results, premature, duplicate and incomplete
mutation attempts, changed confirmed previews, executable Trait claims, wrong Trait
continuity, wrong current projection, invented observer state, malformed
output-schema policy, altered frozen Action prose, target-authored Interaction prose
and ambiguous cleanup. Every retained manifest declares
`evidence_kind: fake_controller_test`, `codex_invoked: false`, `model_calls: 0` and
`paid_candidate: false`.

## Frozen live candidate

T8 freezes one candidate in this same dedicated runner; there is no parallel live
system. Its exact boundary is two Agents, two sessions, seven paid Codex process
calls and zero retries:

1. Pip starts one resumable session for a grounded Action proposal call. Selection
   and steering arrive only in a zero-tool preview resume; confirmation arrives only
   in a third resume exposing `submit_action` alone.
2. An authoritative HTTP gate proves the establishment, generated stable Trait id,
   current statement and Action Activity before continuation.
3. Pip's same session resumes for a later grounded Interaction proposal, zero-tool
   preview and one `submit_interaction`-only commit. The commit develops the same
   stable Trait id, targets Mara and authors no response for her.
4. A second authoritative HTTP gate proves the predecessor/current continuity,
   directed Activity and absence of Trait state on Mara.
5. Mara starts one separate ephemeral session. Her Agent receives no Entity or Trait
   id out of band: it derives Pip from compact orientation, fetches him through
   `get_entity_at_current_place`, then reports the current Trait and authorized
   history.

Every process has a 600-second default deadline (configurable only within the frozen
30–1,800 second safety range). A timeout, malformed result, unexpected tool, changed
preview, failed HTTP gate or cleanup ambiguity ends the only candidate. It is never
retried or replaced.

The environment pin is the resolved executable
`/Users/sanderjansma/Library/Application Support/Herd/config/nvm/versions/node/v22.21.1/lib/node_modules/@openai/codex/bin/codex.js`,
`codex-cli 0.147.0`, `gpt-5.6-sol` with high reasoning, a verified login and explicit
session-id resume. Every phase disables non-Aicadia features, web search, plugins,
subagents and shell access; direct code mode admits only the phase's Aicadia MCP
allowlist. Preview phases have the empty allowlist, not a merely instructed read
allowlist. Six closed output schemas govern proposal, establishment preview/commit,
development preview/commit and Mara's observation.

Codex CLI 0.147.0 exposes no enforceable per-run token ceiling. The honest spend
boundary is therefore exactly seven model process calls, not a fabricated token
number; actual usage events are retained when Codex emits them. The command has no
automatic retry path. One failed call still consumes the one authorization.

## Token-free readiness and paid gate

Public readiness requires a PostgreSQL administration URL:

```sh
DATABASE_URL='postgres://localhost/postgres' tools/trait-playtest preflight
```

Preflight never calls `codex exec`. It verifies the exact executable/version,
login, model/effort availability, feature/config parsing, explicit-session resume,
strict schemas and direct-only allowlists; builds the server/operator binaries;
creates, ownership-tags and reads a disposable PostgreSQL database; compares live
HTTP OpenAPI and MCP `tools/list` with the exact thirteen-tool catalog; then verifies
the exact database name plus its unguessable token before dropping it. Its JSON
result declares `codex_invoked:false`, `model_calls:0`, the frozen call/token claim,
GO/NO-GO and cleanup result.

The audited candidate digest is
`3eb10e6ec1d375048dc96fb415ecad8c77b81f177c65138c315711d248d0f449`.
It hashes canonical path-and-byte material for `Cargo.toml`, `Cargo.lock`, every
Rust source and SQL migration; the exact runtime catalog and six schemas; and the
runner constants and function bodies that generate prompts, configure Codex,
start/clean the database and server, validate every phase and enforce the spend
gate. The digest file itself is excluded, avoiding self-reference. Any bound drift
makes preflight and the old paid command fail before candidate consumption or model
invocation.

Preflight and candidate evidence live beneath `.aicadia-trait-playtest/`. The root
and each evidence directory are mode 700; every retained artifact is mode 600. A
candidate create ambiguity retains recovery instructions and forbids automatic
drop without ownership proof. A `candidate-consumed` sentinel and prior candidate
directory both forbid a second attempt.

The only paid entry point is:

```sh
DATABASE_URL='postgres://localhost/postgres' tools/trait-playtest run \
  --confirm-token-spend \
  --candidate-digest 3eb10e6ec1d375048dc96fb415ecad8c77b81f177c65138c315711d248d0f449
```

Do not run it until a fresh User authorization quotes or unambiguously accepts that
exact command and seven-call/no-retry boundary. T8 GO, plan acceptance, fake
evidence and an earlier unrelated live run do not authorize it.

## Historical runner boundary

[`tools/agent-playtest`](../../tools/agent-playtest) and its recorded Action evidence
remain historical. Its token-free replay uses a frozen Property-era catalog fixture,
not the current runtime catalog or current Agent contract. The old prompts, schemas,
candidate history and evidence claims are not relabelled or reused by this
controller; it is not evidence for current Trait gameplay.
