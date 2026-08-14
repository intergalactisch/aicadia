#!/bin/bash
set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly REPO_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
readonly RUNNER="$REPO_DIR/tools/trait-playtest"
TEST_ROOT="$(mktemp -d "${TMPDIR:-/tmp}/aicadia-trait-playtest-test-XXXXXXXX")"
trap 'rm -rf "$TEST_ROOT"' EXIT

fail() {
    printf 'trait-playtest-test: %s\n' "$*" >&2
    exit 1
}

latest_manifest() {
    find "$1" -name manifest.json -type f | sort | tail -1
}

candidate_state_snapshot() {
    local root="$1" path
    if [[ ! -e "$root" ]]; then
        printf 'absent\n'
        return
    fi
    {
        stat -f 'ROOT %HT %Lp %z %m %N' "$root"
        find "$root" -mindepth 1 \
            \( -name candidate-consumed -o -path "$root/candidate-*" \) -print \
            | LC_ALL=C sort \
            | while IFS= read -r path; do
                if [[ -L "$path" ]]; then
                    stat -f 'LINK %Lp %z %m %N' "$path"
                    readlink "$path"
                elif [[ -d "$path" ]]; then
                    stat -f 'DIR %Lp %z %m %N' "$path"
                elif [[ -f "$path" ]]; then
                    stat -f 'FILE %Lp %z %m %N' "$path"
                    shasum -a 256 "$path"
                else
                    stat -f 'OTHER %HT %Lp %z %m %N' "$path"
                fi
            done
    } | shasum -a 256 | awk '{print $1}'
}

copy_candidate_material() {
    local copy="$1"
    mkdir -p "$copy/tools/trait-playtest-schema" "$copy/tests" "$copy/src" "$copy/migration"
    cp "$RUNNER" "$copy/tools/trait-playtest"
    cp "$REPO_DIR/Cargo.toml" "$REPO_DIR/Cargo.lock" "$copy/"
    cp "$REPO_DIR/tests/agent-tool-catalog.json" "$copy/tests/"
    cp -R "$REPO_DIR/src/." "$copy/src/"
    cp -R "$REPO_DIR/migration/." "$copy/migration/"
    cp "$REPO_DIR/tools/trait-playtest-schema/"* "$copy/tools/trait-playtest-schema/"
    chmod +x "$copy/tools/trait-playtest"
}

run_mode() {
    local mode="$1" root
    root="$TEST_ROOT/$mode"
    mkdir -p "$root"
    AICADIA_TRAIT_PLAYTEST_OUTPUT_ROOT="$root" \
        AICADIA_INTERNAL_TRAIT_PLAYTEST_MODE="$mode" \
        "$RUNNER" test-internal-run --confirm-fake-controller-test \
        >"$root/stdout" 2>"$root/stderr"
}

test_preflight_and_happy_path() {
    local root="$TEST_ROOT/happy" manifest run
    mkdir -p "$root"
    "$RUNNER" test-internal-preflight --confirm-fake-controller-test >"$root/preflight.stdout"
    grep -F 'Trait token-free contract passed' "$root/preflight.stdout" >/dev/null
    run_mode happy
    manifest="$(latest_manifest "$root")"; run="$(dirname "$manifest")"
    jq -e '
      .evidence_kind=="fake_controller_test" and .codex_invoked==false
      and .model_calls==0 and .paid_candidate==false
      and .run_status=="fake_completed" and .validation=="passed"
      and .cleanup.status=="not_needed"
      and all(.phases[];.=="passed")
    ' "$manifest" >/dev/null || fail 'happy manifest is not strictly token-free and complete'
    [[ "$(stat -f '%Lp' "$run")" == 700 ]] || fail 'run directory is not private'
    [[ "$(stat -f '%Lp' "$manifest")" == 600 ]] || fail 'manifest is not private'

    ! grep -Fi 'selects proposal' "$run/action-proposals.prompt.txt" >/dev/null \
        || fail 'selection leaked into Action proposal phase'
    ! grep -Fi 'explicitly accepts' "$run/action-preview.prompt.txt" >/dev/null \
        || fail 'confirmation leaked into Action preview phase'
    grep -Fi 'explicitly accepts' "$run/action-commit.prompt.txt" >/dev/null
    ! grep -Fi 'selects proposal' "$run/interaction-proposals.prompt.txt" >/dev/null \
        || fail 'selection leaked into Interaction proposal phase'
    ! grep -Fi 'explicitly accepts' "$run/interaction-preview.prompt.txt" >/dev/null \
        || fail 'confirmation leaked into Interaction preview phase'
    grep -Fi 'explicitly accepts' "$run/interaction-commit.prompt.txt" >/dev/null
    jq -e '
      .entity_name=="Pip" and .lifecycle=="establish"
      and .current_characterization=="A small grey rat-like traveler."
      and (.proposed_characterization|type)=="string"
      and ((has("entity_id") or has("trait_id") or has("target_entity_id"))|not)
    ' "$run/action-preview.final.json" >/dev/null \
        || fail 'Action preview is not a natural identifier-free establishment'
    jq -e '
      .actor_name=="Pip" and .target_name=="Mara" and .lifecycle=="develop"
      and (.current_characterization|type)=="string"
      and (.proposed_characterization|type)=="string"
      and ((has("entity_id") or has("trait_id") or has("target_entity_id"))|not)
    ' "$run/interaction-preview.final.json" >/dev/null \
        || fail 'Interaction preview is not a natural identifier-free development'

    jq -s -e '[.[]|select(.type=="item.completed")|.item.tool]
        ==["get_world","get_character","list_entity_at_current_place","list_activity_at_current_place"]' \
        "$run/action-proposals.events.jsonl" >/dev/null
    jq -s -e '[.[]|select(.type=="item.completed")|.item.tool]==["submit_action"]' \
        "$run/action-commit.events.jsonl" >/dev/null
    jq -s -e '[.[]|select(.type=="item.completed")|.item.tool]
        ==["get_character","list_entity_at_current_place","list_activity_at_current_place","get_entity_at_current_place"]' \
        "$run/interaction-proposals.events.jsonl" >/dev/null
    jq -s -e '[.[]|select(.type=="item.completed")|.item.tool]==["submit_interaction"]' \
        "$run/interaction-commit.events.jsonl" >/dev/null
    jq -s -e '
      [.[]|select(.type=="item.completed")|.item] as $c
      | [$c[].tool]==["get_character","list_entity_at_current_place","get_entity_at_current_place","list_activity_at_current_place"]
      and ($c[1].result.structured_content.entity[0]|has("current_state")|not)
      and $c[2].arguments.entity_id==$c[1].result.structured_content.entity[0].id
      and $c[2].result.structured_content.current_state.association[0].type=="trait"
      and $c[3].result.structured_content.activity[0].trait_change[0].type=="develop"
    ' "$run/observer.events.jsonl" >/dev/null || fail 'observer did not derive Pip and current Trait through accepted reads'
    ! find "$run" -type f \( -name '*.events.jsonl' -o -name '*.prompt.txt' \) -exec grep -Ein 'codex|gpt-5|openai' {} + >/dev/null \
        || fail 'fake evidence invoked or prompted a model host'
}

test_failure_gates() {
    local mode root manifest
    for mode in malformed-character-result malformed-date-time-result malformed-action-result malformed-activity-page-result \
        premature-action invented-mechanic altered-action-prose changed-action-preview double-action \
        premature-interaction wrong-trait-id target-authored-interaction-prose changed-interaction-preview incomplete-interaction \
        wrong-current-state invented-observer-state ambiguous-cleanup; do
        root="$TEST_ROOT/$mode"
        if run_mode "$mode"; then
            fail "$mode unexpectedly passed"
        fi
        manifest="$(latest_manifest "$root")"
        [[ -n "$manifest" ]] || fail "$mode retained no manifest"
        jq -e '.evidence_kind=="fake_controller_test" and .codex_invoked==false
            and .model_calls==0 and .run_status=="failed"' "$manifest" >/dev/null \
            || fail "$mode did not retain failed token-free evidence"
        case "$mode" in
            malformed-character-result|malformed-date-time-result|malformed-action-result|malformed-activity-page-result)
                grep -F 'violates its runtime outputSchema' "$root/stderr" >/dev/null \
                    || fail "$mode did not fail at the exact runtime output-schema gate"
                ;;
        esac
    done
}

test_schema_policy_fails_before_evidence() {
    local schema_root="$TEST_ROOT/schema" output="$TEST_ROOT/schema-output"
    cp -R "$REPO_DIR/tools/trait-playtest-schema" "$schema_root"
    jq '.properties.marker.uniqueItems=true' "$schema_root/proposals.json" \
        >"$schema_root/proposals.tmp" && mv "$schema_root/proposals.tmp" "$schema_root/proposals.json"
    mkdir -p "$output"
    if AICADIA_INTERNAL_TRAIT_SCHEMA_DIR="$schema_root" \
        AICADIA_TRAIT_PLAYTEST_OUTPUT_ROOT="$output" \
        "$RUNNER" test-internal-run --confirm-fake-controller-test >/dev/null 2>&1; then
        fail 'forbidden schema keyword passed'
    fi
    [[ -z "$(find "$output" -mindepth 1 -print -quit)" ]] \
        || fail 'schema failure created evidence before preflight passed'
}

test_live_gate_and_freeze() {
    local root="$TEST_ROOT/live-gate" digest before after
    mkdir -p "$root"
    digest="$(<"$REPO_DIR/tools/trait-playtest-schema/live-candidate.sha256")"
    before="$(candidate_state_snapshot "$REPO_DIR/.aicadia-trait-playtest")"
    if "$RUNNER" run --confirm-token-spend --confirm-exactly-one-seven-call-paid-candidate >"$root/stdout" 2>"$root/stderr"; then
        fail 'wrong live authorization unexpectedly passed'
    fi
    after="$(candidate_state_snapshot "$REPO_DIR/.aicadia-trait-playtest")"
    grep -F -- '--candidate-digest <audited-sha256>' "$root/stderr" >/dev/null \
        || fail 'obsolete live gate did not fail closed'
    grep -En "MAX_MODEL_CALLS=7|MAX_RETRIES=0|CODEX_MODEL='gpt-5.6-sol'|CODEX_REASONING_EFFORT='high'" "$RUNNER" >/dev/null \
        || fail 'live freeze lost exact calls, retries, model or effort'
    grep -En 'ZERO_TOOL_TOOLS=' "$RUNNER" >/dev/null || fail 'preview phases lost their empty MCP allowlist'
    grep -En 'Codex CLI 0.147.0 exposes no enforceable per-run token ceiling' "$RUNNER" >/dev/null \
        || fail 'token boundary is not honest and explicit'
    [[ "$(grep -Ec -- '--enable mcp_2026_07_28' "$RUNNER")" -ge 2 ]] \
        || fail 'Trait Codex validation and live commands do not both pin MCP 2026-07-28'
    [[ "$after" == "$before" ]] || fail 'rejected authorization changed private candidate state'
    [[ "$digest" =~ ^[0-9a-f]{64}$ ]] || fail 'audited digest is malformed'
}

test_candidate_digest_binds_runtime_build_and_validator() {
    local copy="$TEST_ROOT/candidate-copy" original mutated before after
    copy_candidate_material "$copy"
    original="$("$copy/tools/trait-playtest" test-internal-candidate-digest --confirm-fake-controller-test)"
    printf '%s\n' "$original" >"$copy/tools/trait-playtest-schema/live-candidate.sha256"
    printf '\n// isolated digest drift\n' >>"$copy/src/world/mutation.rs"
    mutated="$("$copy/tools/trait-playtest" test-internal-candidate-digest --confirm-fake-controller-test)"
    [[ "$mutated" != "$original" ]] || fail 'bound World build-input mutation did not change candidate digest'
    before="$(candidate_state_snapshot "$copy/.aicadia-trait-playtest")"
    if (cd "$copy" && tools/trait-playtest run --confirm-token-spend --candidate-digest "$original") \
        >"$copy/old.stdout" 2>"$copy/old.stderr"; then
        fail 'old supplied digest passed after bound build-input drift'
    fi
    after="$(candidate_state_snapshot "$copy/.aicadia-trait-playtest")"
    grep -F 'Trait candidate drift:' "$copy/old.stderr" >/dev/null \
        || fail 'old digest did not fail at token-free drift gate'
    [[ "$after" == "$before" ]] || fail 'digest drift changed private candidate state before model gate'
}

test_owned_preflight_cleanup_after_catalog_failure() {
    local before="$TEST_ROOT/preflight-before" after="$TEST_ROOT/preflight-after" manifest db token
    local portable_bin="$TEST_ROOT/portable-bin" actual_codex
    [[ -n "${DATABASE_URL:-}" ]] || return 0
    actual_codex="$(command -v codex)" || fail 'Codex is unavailable for the live token-free regression'
    mkdir -p "$portable_bin"
    printf '#!/bin/bash\nexec %q "$@"\n' "$actual_codex" >"$portable_bin/portable-codex"
    chmod +x "$portable_bin/portable-codex"
    find "$REPO_DIR/.aicadia-trait-playtest" -maxdepth 2 -name manifest.json -path '*/preflight-*/*' -print 2>/dev/null | sort >"$before"
    if PATH="$portable_bin:$PATH" CODEX_BIN=portable-codex AICADIA_INTERNAL_TRAIT_PREFLIGHT_MODE=fail-after-catalog \
        "$RUNNER" test-internal-live-preflight --confirm-fake-controller-test \
        >"$TEST_ROOT/injected-preflight.stdout" 2>"$TEST_ROOT/injected-preflight.stderr"; then
        fail 'injected post-catalog preflight failure unexpectedly passed'
    fi
    find "$REPO_DIR/.aicadia-trait-playtest" -maxdepth 2 -name manifest.json -path '*/preflight-*/*' -print | sort >"$after"
    manifest="$(comm -13 "$before" "$after" | head -1)"
    [[ -n "$manifest" ]] || fail 'injected preflight retained no new manifest'
    jq -e --arg path "$portable_bin/portable-codex" '.go==false and .codex_invoked==false and .model_calls==0
      and .catalog.status=="live_runtime_equal" and .deployment.status=="dropped"
      and .cleanup.status=="ownership_verified_and_dropped"
      and .codex.path==$path' "$manifest" >/dev/null \
        || fail 'post-catalog failure did not end in ownership-safe terminal cleanup'
    db="$(jq -r '.deployment.database' "$manifest")"; token="$(jq -r '.deployment.ownership_token' "$manifest")"
    if (cd "$REPO_DIR" && cargo run --quiet --bin aicadia-playtest-database -- verify "$db" "$token") >/dev/null 2>&1; then
        fail 'injected preflight database still exists after cleanup'
    fi
}

test_public_cli_version_drift_fails_before_candidate() {
    local root="$TEST_ROOT/cli-drift" copy baseline before after
    copy="$root/candidate-copy"
    copy_candidate_material "$copy"
    baseline="$("$copy/tools/trait-playtest" test-internal-candidate-digest --confirm-fake-controller-test)"
    printf '%s\n' "$baseline" >"$copy/tools/trait-playtest-schema/live-candidate.sha256"
    mkdir -p "$root/bin"
    printf '%s\n' '#!/bin/bash' 'printf "codex-cli 0.148.0\\n"' >"$root/bin/codex"
    chmod +x "$root/bin/codex"
    before="$(candidate_state_snapshot "$copy/.aicadia-trait-playtest")"
    if (cd "$copy" && PATH="$root/bin:$PATH" DATABASE_URL='postgres://must-not-be-used.invalid/postgres' \
        tools/trait-playtest preflight) >"$root/stdout" 2>"$root/stderr"; then
        fail 'public CLI version drift unexpectedly passed'
    fi
    after="$(candidate_state_snapshot "$copy/.aicadia-trait-playtest")"
    grep -F 'Codex must be exactly codex-cli 0.147.0; found codex-cli 0.148.0.' "$root/stderr" >/dev/null \
        || fail 'CLI version drift did not fail at the semantic version boundary'
    [[ "$after" == "$before" ]] || fail 'CLI drift changed private candidate state'
}

test_preflight_and_happy_path
test_failure_gates
test_schema_policy_fails_before_evidence
test_live_gate_and_freeze
test_public_cli_version_drift_fails_before_candidate
test_candidate_digest_binds_runtime_build_and_validator
test_owned_preflight_cleanup_after_catalog_failure
if [[ -n "${DATABASE_URL:-}" ]]; then
    printf 'Trait token-free suite passed without Codex exec or model spend, including owned database failure cleanup.\n'
else
    printf 'Trait fake playtest suite passed without Codex, model spend, database or server.\n'
fi
