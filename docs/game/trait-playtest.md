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

The environment pin discovers `codex` through `PATH`, canonicalizes that executable
once for the run, then requires `codex-cli 0.147.0`, `gpt-5.6-sol` with high
reasoning, a verified login and explicit session-id resume. The installation
location may differ per machine and is recorded only in ignored private evidence;
it is not candidate digest material or repository configuration. Every phase
disables non-Aicadia features, web search, plugins, subagents and shell access;
direct code mode explicitly enables the sole supported MCP wire protocol revision
`2026-07-28` and fails closed without it. There is no older initialize flow,
downgrade, fallback or compatibility shim. Each phase admits only its Aicadia MCP
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

Preflight never calls `codex exec`. It discovers and canonicalizes the executable,
then verifies its exact version, login, model/effort availability, feature/config
parsing, explicit-session resume,
strict schemas and direct-only allowlists; builds the server/operator binaries;
creates, ownership-tags and reads a disposable PostgreSQL database; compares live
HTTP OpenAPI and MCP `tools/list` with the exact thirteen-tool catalog; then verifies
the exact database name plus its unguessable token before dropping it. Its JSON
result declares `codex_invoked:false`, `model_calls:0`, the frozen call/token claim,
GO/NO-GO and cleanup result.

Independent T8 review covered the superseded path-bound digest
`3eb10e6ec1d375048dc96fb415ecad8c77b81f177c65138c315711d248d0f449`.
The a564 replacement candidate digest was
`a564ddedd49094dd70b02aad3eefdb39e6334b482d94859d6f61dea720fb1302`.
Candidate `candidate-MmwRmcBv` used the immediately preceding digest
`f38ed39b7a100ee09cca46743b3b9398f46ccb3d4999f4591f478113fb2b4fa3`,
consumed its authorization and invoked Codex for exactly one model process call. It
failed before any Agent MCP read or post-setup Agent submit/Trait mutation because MCP revision `2026-07-28` was
not enabled. Ownership-verified cleanup dropped its database and retained the
evidence. After T9R GO, the User explicitly authorized one paid a564 replacement
using only MCP `2026-07-28`. `candidate-ydttdFfc` consumed that authorization and
one model process call (`68117` input, `50176` cached input, `798` output and `419`
reasoning tokens). Exactly `get_world`, `get_character`,
`list_entity_at_current_place` and `list_activity_at_current_place` completed, and
the strict final contained three valid grounded proposals. The controller then
falsely rejected valid fractional RFC3339 timestamps from the latter affected
outputs. No preview, post-setup Agent submit, Trait mutation or candidate HTTP gate ran. Ownership-verified
cleanup dropped the database; independent audit returned GO for this exact failed
claim with one P1 at `validate_tool_output` and no other finding. Archive
`.aicadia-trait-playtest/archive-original-f38-MmwRmcBv` preserves all 68 original
files with private permissions and unchanged content/permission fingerprints.
Token-free `preflight-cEeVelIZ` returns GO for a564 with
`candidate_started=false`, `authorization_consumed=false`, `codex_invoked=false`,
`model_calls=0`, thirteen equal runtime tools, six passed schemas and an
ownership-verified dropped database. Independent audit found only the stale-status
P3 corrected here; focused independent re-review returns GO with no P0–P3 finding
and completes T9R. T10 accepts the real UTC fractional-second Chrono form while
requiring exact normalized parse/format roundtrip. The current post-failure runner
digest is `6649959c7f230f2240f8d1b7e67081c20c473c00654ef36409fa439a8d85a824`;
the later T4 candidate is the exact two-call failure recorded below. Initial final
review's sole P1/P2
were corrected; second independent Terry/ownership review returned GO with no P0–P3
finding. No retry or Trait live gameplay outcome was earned by that correction.

The accepted
[Trait live-validation plan](../../.agents/plans/20260814-111749-trait-live-validation/plan.md)
is complete and leaves no `Now / Active` edge. T2 completed: private
`archive-replacement-a564-ydttdFfc` contains all 64 unchanged a564 historical files
plus its private algorithm index, and the original 68-file f38 archive remains
unchanged. Public token-free `preflight-HMxwGPCF` returned GO for digest
`6649959c7f230f2240f8d1b7e67081c20c473c00654ef36409fa439a8d85a824`:
`candidate_started=false`, `authorization_consumed=false`,
`codex_invoked=false`, `model_calls=0`, current-only MCP `2026-07-28`, 13
live-equal tools, 6 schemas and cleanup `ownership_verified_and_dropped`. The initial
T3 review found exactly two P3 findings—the index lacked an explicit serialization
recipe and current authorities were still prospective—and both are corrected.
Independent re-review returned GO with no P0–P3 finding, completing T3 readiness.
Exact-digest `candidate-63hjH4HW` then consumed its authorization and two model
calls. The Action proposal passed after the four current-MCP reads `get_world`,
`get_character`, `list_entity_at_current_place` and
`list_activity_at_current_place`. Its zero-tool Action preview exited 0 with strict
valid JSON and exact `entity_name: "Pip"`, then failed solely because live
`validate_action_preview` required `startswith("Pip ")` while the prompt and
permanent fake contract require exact `Pip`. No Action commit, candidate HTTP gate,
Interaction or Mara phase ran; submits, post-setup Trait mutation and live success
are zero. Total usage was 148068 input, 100352 cached input, 1722 output and 867
reasoning-output tokens. Cleanup was `ownership_verified_and_dropped`, evidence is
private, no process remains and no retry or new authorization exists. Final
independent review returned GO with P0/P2/P3=0. The known deferred P1 is this Action
live-name drift plus analogous unreached Interaction `startswith("Pip ")`/
`startswith("Mara ")` drift against prompt/fake exact names. No fix, retry, live
success or new authorization was added. The User chose the unchanged draft
documentation-architecture plan next; it is not activated here.

It hashes canonical repository path-and-byte material for `Cargo.toml`, `Cargo.lock`,
every Rust source and SQL migration; the exact runtime catalog and six schemas; and
the runner constants and function bodies that generate prompts, configure Codex,
start/clean the database and server, validate every phase and enforce the spend gate.
The locally resolved executable path is deliberately excluded. The digest file
itself is excluded, avoiding self-reference. Any bound material drift fails
preflight; the supplied a564 digest was frozen for the now-consumed replacement
command, and any differing digest failed before candidate consumption or model
invocation. The post-failure digest passed public token-free
`preflight-HMxwGPCF`; it has no paid authorization.

Preflight and candidate evidence live beneath `.aicadia-trait-playtest/`. The root
and each evidence directory are mode 700; every retained artifact is mode 600. A
candidate create ambiguity retains recovery instructions and forbids automatic
drop without ownership proof. A `candidate-consumed` sentinel and prior candidate
directory both forbid a second attempt.

The frozen paid command that produced failed `candidate-ydttdFfc` was:

```sh
DATABASE_URL='postgres://localhost/postgres' tools/trait-playtest run \
  --confirm-token-spend \
  --candidate-digest a564ddedd49094dd70b02aad3eefdb39e6334b482d94859d6f61dea720fb1302
```

That authorization is consumed and the sentinel forbids another candidate. The
post-failure runner has no preflight or execution authorization. No retry, other
candidate or protocol fallback is authorized.

## Historical runner boundary

[`tools/agent-playtest`](../../tools/agent-playtest) and its recorded Action evidence
remain historical. Its token-free replay uses a frozen Property-era catalog fixture,
not the current runtime catalog or current Agent contract. The old prompts, schemas,
candidate history and evidence claims are not relabelled or reused by this
controller; it is not evidence for current Trait gameplay.
