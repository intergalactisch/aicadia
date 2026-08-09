#!/usr/bin/env bash
set -euo pipefail

readonly REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readonly RUNNER="$REPO_DIR/tools/agent-playtest"
TEST_TMP="$(mktemp -d)"
trap '[[ "${KEEP_AGENT_PLAYTEST_TMP:-0}" == 1 ]] || rm -rf "$TEST_TMP"' EXIT

fail() {
    printf 'agent-playtest test: %s\n' "$*" >&2
    exit 1
}

assert_eq() {
    [[ "$1" == "$2" ]] || fail "expected [$2], got [$1]"
}

assert_contains() {
    grep -F -- "$2" "$1" >/dev/null || fail "$1 does not contain [$2]"
}

assert_line() {
    grep -Fx -- "$2" "$1" >/dev/null || fail "$1 does not contain exact line [$2]"
}

assert_pair() {
    awk -v first="$2" -v second="$3" 'previous == first && $0 == second { found = 1 } { previous = $0 } END { exit !found }' "$1" \
        || fail "$1 does not contain adjacent lines [$2] [$3]"
}

assert_no_pair() {
    if awk -v first="$2" -v second="$3" 'previous == first && $0 == second { found = 1 } { previous = $0 } END { exit !found }' "$1"; then
        fail "$1 contains forbidden adjacent lines [$2] [$3]"
    fi
}

make_fakes() {
    local fake_dir="$1"
    mkdir -p "$fake_dir/bin" "$fake_dir/state" "$fake_dir/output"

    cat >"$fake_dir/bin/cargo" <<'FAKE'
#!/usr/bin/env bash
set -euo pipefail
state="$(cd "$(dirname "$0")/../state" && pwd)"
mode="$(<"$state/mode")"
args="$*"
if [[ "$args" == 'build --quiet --bin aicadia --bin aicadia-provision-user --bin aicadia-playtest-database' ]]; then
    printf '%s\n' build >>"$state/preflight-actions"
elif [[ "$args" == 'run --quiet --bin aicadia-playtest-database -- probe' ]]; then
    printf '%s\n' probe >>"$state/database-actions"
elif [[ "$args" == run\ --quiet\ --bin\ aicadia-playtest-database\ --\ create\ * ]]; then
    printf 'create %s\n' "${!#}" >>"$state/database-actions"
    [[ "$mode" != 'ambiguous-create' ]] || exit 19
elif [[ "$args" == run\ --quiet\ --bin\ aicadia-playtest-database\ --\ drop\ * ]]; then
    printf 'drop %s\n' "${!#}" >>"$state/database-actions"
elif [[ "$args" == 'run --quiet --bin aicadia' ]]; then
    printf '%s\n' '{"event":"server_ready","address":"127.0.0.1:45678"}'
    trap 'exit 0' TERM INT
    while :; do sleep 1; done
elif [[ "$args" == 'run --quiet --bin aicadia-provision-user' ]]; then
    count=0
    [[ ! -f "$state/provision-count" ]] || count="$(<"$state/provision-count")"
    count=$((count + 1))
    printf '%s\n' "$count" >"$state/provision-count"
    if [[ "$mode" == 'provision-fail' && $count -eq 2 ]]; then
        exit 18
    fi
    if [[ $count -eq 1 ]]; then
        id='11111111-1111-4111-8111-111111111111'
    else
        id='22222222-2222-4222-8222-222222222222'
    fi
    printf '{"id":"%s","created_at":"2026-08-08T12:00:00Z"}\n' "$id"
else
    printf 'unexpected fake cargo call: %s\n' "$args" >&2
    exit 90
fi
FAKE

    cat >"$fake_dir/bin/curl" <<'FAKE'
#!/usr/bin/env bash
set -euo pipefail
state="$(cd "$(dirname "$0")/../state" && pwd)"
args="$*"
[[ "${1:-}" == '--disable' ]] || { printf 'curl --disable was not first\n' >&2; exit 92; }
if [[ "$args" == *'-w %{http_code}'* ]]; then
    printf '404'
elif [[ "$args" == *'/api/openapi.json'* ]]; then
    printf '%s\n' '{"paths":{"/api/world":{"get":{"operationId":"get_world"}},"/api/user":{"get":{"operationId":"get_user"}},"/api/character":{"get":{"operationId":"get_character"},"post":{"operationId":"create_character"}},"/api/entity":{"get":{"operationId":"list_entity"},"post":{"operationId":"create_entity"}},"/api/entity/{entity_id}":{"get":{"operationId":"get_entity"}}}}'
elif [[ "$args" == *'/mcp'* ]]; then
    printf '%s\n' '{"jsonrpc":"2.0","id":1,"result":{"tools":[{"name":"get_world"},{"name":"get_user"},{"name":"get_character"},{"name":"create_character"},{"name":"list_entity"},{"name":"get_entity"},{"name":"create_entity"}]}}'
elif [[ "$args" == *'/api/entity?limit=100'* ]]; then
    if [[ -f "$state/entity.json" ]]; then
        jq '{entity: [{id: .id, name: .name}], next: null}' "$state/entity.json"
    else
        printf '%s\n' '{"entity":[],"next":null}'
    fi
elif [[ "$args" == *'/api/entity/'* ]]; then
    jq . "$state/entity.json"
elif [[ "$args" == *'/api/user'* ]]; then
    if [[ "$args" == *'11111111-1111-4111-8111-111111111111'* ]]; then
        id='11111111-1111-4111-8111-111111111111'
    else
        id='22222222-2222-4222-8222-222222222222'
    fi
    printf '{"id":"%s","created_at":"2026-08-08T12:00:00Z"}\n' "$id"
elif [[ "$args" == *'/api/world'* ]]; then
    printf '%s\n' '{"name":"Aicadia"}'
else
    printf 'unexpected fake curl call: %s\n' "$args" >&2
    exit 91
fi
FAKE

    cat >"$fake_dir/bin/codex-fake" <<'FAKE'
#!/usr/bin/env bash
set -euo pipefail
state="$(cd "$(dirname "$0")/../state" && pwd)"
printf '%s\n' "$*" >>"$state/codex-commands"
case "$*" in
    --version)
        printf '%s\n' 'codex-cli 0.147.0'
        exit 0
        ;;
    'login status')
        printf '%s\n' 'Logged in using ChatGPT'
        exit 0
        ;;
    'debug models')
        printf '%s\n' '{"models":[{"slug":"gpt-5.6-sol","supported_reasoning_levels":[{"effort":"high"}]}]}'
        exit 0
        ;;
    'exec --help')
        printf '%s\n' '--ephemeral --ignore-user-config --ignore-rules --strict-config --skip-git-repo-check --json --output-schema --output-last-message'
        exit 0
        ;;
esac

if [[ "$*" == *'features list' ]]; then
    for feature in apps auth_elicitation browser_use browser_use_external browser_use_full_cdp_access code_mode code_mode_host computer_use goals hooks image_generation in_app_browser multi_agent multi_agent_v2 plugins remote_plugin shell_snapshot shell_tool skill_mcp_dependency_install skill_search tool_call_mcp_elicitation tool_suggest unified_exec view_image workspace_dependencies; do
        enabled=true
        [[ "$*" != *"--disable $feature"* ]] || enabled=false
        printf '%-40s stable %s\n' "$feature" "$enabled"
    done
    exit 0
fi

if [[ "$*" == *'mcp get aicadia --json' ]]; then
    arguments="$(printf '%s\n' "$@")"
    for required in \
        'suppress_unstable_features_warning=true' \
        'features.code_mode.enabled=true' \
        'features.code_mode.direct_only_tool_namespaces=["mcp__aicadia"]' \
        'web_search="disabled"' 'tools.web_search=false' 'agents.enabled=false' \
        'model_reasoning_effort="high"' 'mcp_servers.aicadia.url="http://127.0.0.1:9/mcp"' \
        'mcp_servers.aicadia.enabled=true' 'mcp_servers.aicadia.required=true' \
        'mcp_servers.aicadia.default_tools_approval_mode="approve"' \
        'mcp_servers.aicadia.env_http_headers={"Aicadia-User-Id"="AICADIA_USER_ID"}'; do
        grep -Fx -- "$required" <<<"$arguments" >/dev/null || exit 97
    done
    if [[ "$*" == *'enabled_tools=["create_entity"]'* ]]; then
        tools='["create_entity"]'
    elif [[ "$*" == *'enabled_tools=["get_user","list_entity","get_entity"]'* ]]; then
        tools='["get_user","list_entity","get_entity"]'
    else
        exit 96
    fi
    jq -n --argjson tools "$tools" '{name:"aicadia",enabled:true,disabled_reason:null,transport:{type:"streamable_http",url:"http://127.0.0.1:9/mcp",bearer_token_env_var:null,http_headers:null,env_http_headers:{"Aicadia-User-Id":"AICADIA_USER_ID"}},enabled_tools:$tools,disabled_tools:null,startup_timeout_sec:null,tool_timeout_sec:null}'
    exit 0
fi

mode="$(<"$state/mode")"
count=0
[[ ! -f "$state/codex-count" ]] || count="$(<"$state/codex-count")"
count=$((count + 1))
printf '%s\n' "$count" >"$state/codex-count"
call_dir="$state/call-$count"
mkdir "$call_dir"
printf '%s\n' "$AICADIA_USER_ID" >"$call_dir/user-id"
env | sort >"$call_dir/environment"
pwd >"$call_dir/cwd"
printf '%s\n' "$@" >"$call_dir/argv"

final=''
previous=''
for argument in "$@"; do
    if [[ "$previous" == '--output-last-message' ]]; then final="$argument"; fi
    previous="$argument"
done
prompt="${!#}"
printf '%s\n' "$prompt" >"$call_dir/prompt"
marker="$(sed -n 's/.*\(aicadia-playtest-[0-9A-Za-z-]*\).*/\1/p' <<<"$prompt" | head -1)"
name="Disposable Playtest Entity $marker"
description="Technical cross-User MCP fixture in a disposable World: $marker"
entity_id='aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa'

printf '%s\n' '{"type":"thread.started"}'
code_mode_diagnostic='Code Mode is unavailable because code-mode host is disabled. Code mode will fail closed; enable `features.code_mode_host` and install `codex-code-mode-host`.'
case "$mode" in
    other-error)
        jq -nc --arg message 'Different native tool startup error' \
            '{type:"item.completed",item:{id:"item-0",type:"error",message:$message}}'
        ;;
    code-mode-host-disabled)
        jq -nc --arg message "$code_mode_diagnostic" \
            '{type:"item.completed",item:{id:"item-0",type:"error",message:$message}}'
        ;;
esac
printf '%s\n' '{"type":"turn.started"}'
if [[ $count -eq 1 && "$mode" == 'a-program' ]]; then
    printf '%s\n' '{"type":"item.completed","item":{"id":"item-program","type":"program","status":"completed"}}'
fi
if [[ $count -eq 1 && "$mode" == 'a-exit' ]]; then exit 17; fi
if [[ $count -eq 1 && "$mode" == 'a-timeout' ]]; then
    (
        trap '' TERM
        sleep 10
    ) &
    blocking_child=$!
    printf '%s\n' "$blocking_child" >"$state/timeout-child-pid"
    wait "$blocking_child"
fi
if [[ $count -eq 1 && "$mode" == 'a-malformed' ]]; then
    printf '%s\n' '{"status":"created","marker":"wrong"}' >"$final"
    exit 0
fi
if [[ $count -eq 1 ]]; then
    jq -n --arg id "$entity_id" --arg name "$name" --arg description "$description" --arg user "$AICADIA_USER_ID" \
        '{id: $id, name: $name, description: $description, introduced_by_user_id: $user, introduced_at: "2026-08-08T12:01:00Z"}' >"$state/entity.json"
    jq -n --arg marker "$marker" --arg id "$entity_id" --arg name "$name" --arg description "$description" \
        '{status: "created", marker: $marker, entity_id: $id, name: $name, description: $description}' >"$final"
    jq -nc --arg name "$name" --arg description "$description" --argjson result "$(<"$state/entity.json")" \
        '{type:"item.completed",item:{id:"item-1",type:"mcp_tool_call",server:"aicadia",tool:"create_entity",arguments:{name:$name,description:$description},result:{content:[{type:"text",text:($result|tojson)}],structured_content:$result},status:"completed",error:null}}'
    if [[ "$mode" == 'a-second-create' ]]; then
        printf '%s\n' '{"type":"item.completed","item":{"id":"item-extra","type":"mcp_tool_call","server":"aicadia","tool":"create_entity","arguments":{"name":"Unmarked","description":"Extra"},"status":"completed","error":null}}'
    fi
else
    if [[ ! -f "$state/entity.json" ]]; then
        jq -n --arg id "$entity_id" --arg name "$name" --arg description "$description" \
            '{id:$id,name:$name,description:$description,introduced_by_user_id:"11111111-1111-4111-8111-111111111111",introduced_at:"2026-08-08T12:01:00Z"}' >"$state/entity.json"
    fi
    if [[ -f "$state/entity.json" ]]; then
        name="$(jq -r '.name' "$state/entity.json")"
        description="$(jq -r '.description' "$state/entity.json")"
    fi
    if [[ "$mode" == 'b-final-fabricated' ]]; then name='Fabricated final'; fi
    jq -n --arg marker "$marker" --arg observer "$AICADIA_USER_ID" --arg id "$entity_id" --arg name "$name" --arg description "$description" \
        '{status: "observed", marker: $marker, observer_user_id: $observer, entity_id: $id, name: $name, description: $description}' >"$final"
    user_result="$(jq -nc --arg id "$AICADIA_USER_ID" '{id:$id,created_at:"2026-08-08T12:00:00Z"}')"
    list_result="$(jq -nc --arg id "$entity_id" --arg name "$(jq -r '.name' "$state/entity.json")" '{entity:[{id:$id,name:$name}],next:null}')"
    entity_result="$(<"$state/entity.json")"
    [[ "$mode" != 'b-result-fabricated' ]] || entity_result="$(jq '.description = "Fabricated result"' "$state/entity.json")"
    list_arguments='{"limit":25}'
    case "$mode" in
        b-cursor) list_arguments='{"cursor":"invented"}' ;;
        b-unexpected-argument) list_arguments='{"limit":25,"unexpected":true}' ;;
        b-boolean-limit) list_arguments='{"limit":true}' ;;
        b-float-limit) list_arguments='{"limit":25.5}' ;;
        b-low-limit) list_arguments='{"limit":0}' ;;
        b-high-limit) list_arguments='{"limit":101}' ;;
    esac
    get_user_event="$(jq -nc --argjson result "$user_result" '{type:"item.completed",item:{id:"item-1",type:"mcp_tool_call",server:"aicadia",tool:"get_user",arguments:{},result:{content:[{type:"text",text:($result|tojson)}],structured_content:$result},status:"completed",error:null}}')"
    list_event="$(jq -nc --argjson result "$list_result" --argjson arguments "$list_arguments" '{type:"item.completed",item:{id:"item-2",type:"mcp_tool_call",server:"aicadia",tool:"list_entity",arguments:$arguments,result:{content:[{type:"text",text:($result|tojson)}],structured_content:$result},status:"completed",error:null}}')"
    get_entity_event="$(jq -nc --argjson result "$entity_result" --arg id "$entity_id" '{type:"item.completed",item:{id:"item-3",type:"mcp_tool_call",server:"aicadia",tool:"get_entity",arguments:{entity_id:$id},result:{content:[{type:"text",text:($result|tojson)}],structured_content:$result},status:"completed",error:null}}')"
    if [[ "$mode" == 'b-order' ]]; then
        printf '%s\n' "$list_event" "$get_user_event" "$get_entity_event"
    else
        printf '%s\n' "$get_user_event" "$list_event" "$get_entity_event"
    fi
fi
printf '%s\n' '{"type":"turn.completed","usage":{"input_tokens":10,"output_tokens":5}}'
FAKE
    chmod +x "$fake_dir/bin/"*
}

run_with_fakes() {
    local fake_dir="$1" runner="${PLAYTEST_RUNNER:-$RUNNER}"
    shift
    printf '%s\n' "${FAKE_MODE:-happy}" >"$fake_dir/state/mode"
    PATH="$fake_dir/bin:/usr/bin:/bin" \
        CODEX_BIN="$fake_dir/bin/codex-fake" \
        DATABASE_URL='postgresql:///fake' \
        AICADIA_DATABASE_NAME='must-not-reach-agent' \
        AICADIA_PORT=9999 \
        FAKE_STATE="$fake_dir/state" \
        FAKE_SECRET='must-not-reach-agent' \
        FAKE_MODE="${FAKE_MODE:-happy}" \
        PGDATABASE='must-not-reach-agent' \
        PGHOST='must-not-reach-agent' \
        PGHOSTADDR='127.0.0.1' \
        PGPASSWORD='must-not-reach-agent' \
        PGPORT=5432 \
        PGSERVICE='must-not-reach-agent' \
        PGSERVICEFILE='must-not-reach-agent' \
        PGSSLMODE=disable \
        PGUSER='must-not-reach-agent' \
        AICADIA_PLAYTEST_OUTPUT_ROOT="$fake_dir/output" \
        AICADIA_AGENT_TIMEOUT_SECONDS=30 \
        AICADIA_PLAYTEST_TEST_MODE=fake \
        AICADIA_PLAYTEST_TEST_TIMEOUT_SECONDS=1 \
        "$runner" "$@"
}

latest_manifest() {
    find "$1/output" -name manifest.json -print -quit
}

test_zero_agent_paths() {
    local fake_dir="$TEST_TMP/zero"
    make_fakes "$fake_dir"
    run_with_fakes "$fake_dir" preflight >/dev/null
    [[ ! -f "$fake_dir/state/codex-count" ]] || fail 'preflight started an Agent'
    grep -vFx 'exec --help' "$fake_dir/state/codex-commands" | grep -E '(^| )exec( |$)' >/dev/null \
        && fail 'preflight invoked codex exec'
    [[ "$(<"$fake_dir/state/database-actions")" == probe ]] || fail 'preflight mutated PostgreSQL'
    assert_line "$fake_dir/state/preflight-actions" build
    assert_eq "$(grep -c 'mcp get aicadia --json' "$fake_dir/state/codex-commands")" '2'
    for arguments in 'run' 'run --wrong' 'preflight extra'; do
        if run_with_fakes "$fake_dir" $arguments >/dev/null 2>&1; then fail "invalid invocation passed: $arguments"; fi
    done
    [[ ! -f "$fake_dir/state/provision-count" ]] || fail 'invalid invocation provisioned Users'
}

test_schema_semantic_preflight_rejects_missing_status_type() {
    local fake_dir="$TEST_TMP/schema-status-type" copied_runner
    make_fakes "$fake_dir"
    mkdir -p "$fake_dir/repo/tools/agent-playtest-schema" "$fake_dir/repo/src/bin"
    cp "$RUNNER" "$fake_dir/repo/tools/agent-playtest"
    cp "$REPO_DIR/tools/agent-playtest-schema/"*.json "$fake_dir/repo/tools/agent-playtest-schema/"
    touch "$fake_dir/repo/Cargo.toml" "$fake_dir/repo/src/bin/aicadia-provision-user.rs" \
        "$fake_dir/repo/src/bin/aicadia-playtest-database.rs"
    jq 'del(.properties.status.type)' "$fake_dir/repo/tools/agent-playtest-schema/create.json" \
        >"$fake_dir/repo/tools/agent-playtest-schema/create.invalid.json"
    mv "$fake_dir/repo/tools/agent-playtest-schema/create.invalid.json" \
        "$fake_dir/repo/tools/agent-playtest-schema/create.json"
    copied_runner="$fake_dir/repo/tools/agent-playtest"
    if PLAYTEST_RUNNER="$copied_runner" run_with_fakes "$fake_dir" preflight >/dev/null 2>&1; then
        fail 'preflight accepted a status const without an explicit string type'
    fi
    [[ ! -f "$fake_dir/state/preflight-actions" ]] || fail 'invalid schema reached the build step'
    [[ ! -f "$fake_dir/state/database-actions" ]] || fail 'invalid schema reached PostgreSQL'
    [[ ! -f "$fake_dir/state/codex-commands" ]] || fail 'invalid schema invoked Codex'
}

assert_two_agent_contract() {
    local fake_dir="$1" argv_a argv_b manifest database
    assert_eq "$(<"$fake_dir/state/codex-count")" '2'
    assert_eq "$(<"$fake_dir/state/provision-count")" '2'
    assert_eq "$(find "$fake_dir/output" -name agent-a.exit-status -exec sed -n '1p' {} \;)" "${EXPECTED_A_EXIT:-0}"
    assert_eq "$(find "$fake_dir/output" -name agent-b.exit-status -exec sed -n '1p' {} \;)" '0'
    assert_eq "$(<"$fake_dir/state/call-1/user-id")" '11111111-1111-4111-8111-111111111111'
    assert_eq "$(<"$fake_dir/state/call-2/user-id")" '22222222-2222-4222-8222-222222222222'
    [[ "$(<"$fake_dir/state/call-1/cwd")" != "$(<"$fake_dir/state/call-2/cwd")" ]] || fail 'Agents shared cwd'
    [[ "$(<"$fake_dir/state/call-1/cwd")" != "$REPO_DIR"* ]] || fail 'Agent A loaded project config'
    [[ "$(<"$fake_dir/state/call-2/cwd")" != "$REPO_DIR"* ]] || fail 'Agent B loaded project config'
    argv_a="$fake_dir/state/call-1/argv"
    argv_b="$fake_dir/state/call-2/argv"
    for argv in "$argv_a" "$argv_b"; do
        assert_line "$argv" 'gpt-5.6-sol'
        assert_line "$argv" 'model_reasoning_effort="high"'
        assert_line "$argv" 'agents.enabled=false'
        assert_line "$argv" 'web_search="disabled"'
        assert_line "$argv" 'tools.web_search=false'
        assert_line "$argv" 'suppress_unstable_features_warning=true'
        assert_line "$argv" 'features.code_mode.enabled=true'
        assert_line "$argv" 'features.code_mode.direct_only_tool_namespaces=["mcp__aicadia"]'
        assert_line "$argv" '--ephemeral'
        assert_line "$argv" '--ignore-user-config'
        assert_line "$argv" '--ignore-rules'
        assert_line "$argv" '--strict-config'
        assert_line "$argv" 'multi_agent'
        assert_pair "$argv" '--enable' 'code_mode_host'
        assert_no_pair "$argv" '--disable' 'code_mode_host'
        assert_no_pair "$argv" '--disable' 'code_mode'
        assert_line "$argv" 'shell_tool'
        assert_line "$argv" 'unified_exec'
        assert_line "$argv" 'browser_use'
        assert_line "$argv" 'computer_use'
        assert_line "$argv" 'plugins'
    done
    for environment in "$fake_dir/state/call-1/environment" "$fake_dir/state/call-2/environment"; do
        for forbidden in DATABASE_URL AICADIA_DATABASE_NAME AICADIA_PORT AICADIA_AGENT_TIMEOUT_SECONDS \
            AICADIA_PLAYTEST_OUTPUT_ROOT AICADIA_PLAYTEST_TEST_MODE AICADIA_PLAYTEST_TEST_TIMEOUT_SECONDS \
            CODEX_BIN FAKE_STATE FAKE_SECRET FAKE_MODE PGDATABASE PGHOST PGHOSTADDR PGPASSWORD PGPORT \
            PGSERVICE PGSERVICEFILE PGSSLMODE PGUSER; do
            grep -q "^${forbidden}=" "$environment" \
                && fail "Agent inherited forbidden environment variable $forbidden: $environment"
        done
        grep -E '^FAKE_[^=]*=' "$environment" >/dev/null \
            && fail "Agent inherited a FAKE_* environment variable: $environment"
    done
    assert_line "$argv_a" 'mcp_servers.aicadia.enabled_tools=["create_entity"]'
    assert_line "$argv_b" 'mcp_servers.aicadia.enabled_tools=["get_user","list_entity","get_entity"]'
    grep -F 'create_entity"]' "$argv_b" >/dev/null && fail 'Agent B received create_entity'
    manifest="$(latest_manifest "$fake_dir")"
    [[ -n "$manifest" ]] || fail 'manifest was not retained'
    assert_eq "$(jq -r '.model' "$manifest")" 'gpt-5.6-sol'
    assert_eq "$(jq -r '.reasoning_effort' "$manifest")" 'high'
    assert_eq "$(jq -r '.cleanup.status' "$manifest")" 'dropped'
    database="$(jq -r '.deployment.database' "$manifest")"
    assert_line "$fake_dir/state/database-actions" "create $database"
    assert_line "$fake_dir/state/database-actions" "drop $database"
}

test_happy_path() {
    local fake_dir="$TEST_TMP/happy"
    make_fakes "$fake_dir"
    FAKE_MODE=happy run_with_fakes "$fake_dir" run --confirm-token-spend >/dev/null
    assert_two_agent_contract "$fake_dir"
    assert_eq "$(jq -r '.validation.status' "$(latest_manifest "$fake_dir")")" 'passed'
}

test_provisioning_failure_starts_no_agent_and_retains_evidence() {
    local fake_dir="$TEST_TMP/provision-fail" manifest
    make_fakes "$fake_dir"
    if FAKE_MODE=provision-fail run_with_fakes "$fake_dir" run --confirm-token-spend >/dev/null 2>&1; then
        fail 'provisioning failure unexpectedly passed'
    fi
    [[ ! -f "$fake_dir/state/codex-count" ]] || fail 'provisioning failure started an Agent'
    manifest="$(latest_manifest "$fake_dir")"
    assert_eq "$(jq -r '.provisioning.user_a' "$manifest")" '11111111-1111-4111-8111-111111111111'
    assert_eq "$(jq -r '.cleanup.status' "$manifest")" 'dropped'
}

test_a_failure_still_runs_b_once() {
    local mode fake_dir started elapsed child_pid
    for mode in a-exit a-timeout a-malformed; do
        fake_dir="$TEST_TMP/$mode"
        make_fakes "$fake_dir"
        started=$SECONDS
        if FAKE_MODE="$mode" run_with_fakes "$fake_dir" run --confirm-token-spend >/dev/null 2>&1; then
            fail "$mode unexpectedly passed"
        fi
        elapsed=$((SECONDS - started))
        case "$mode" in
            a-exit) EXPECTED_A_EXIT=17 assert_two_agent_contract "$fake_dir" ;;
            a-timeout)
                EXPECTED_A_EXIT=142 assert_two_agent_contract "$fake_dir"
                [[ $elapsed -lt 9 ]] || fail "timeout case waited $elapsed seconds for its 10-second blocking child"
                child_pid="$(<"$fake_dir/state/timeout-child-pid")"
                if kill -0 "$child_pid" 2>/dev/null; then
                    fail "timeout left blocking Agent child $child_pid running"
                fi
                ;;
            *) assert_two_agent_contract "$fake_dir" ;;
        esac
    done
}

test_second_unmarked_create_is_rejected() {
    local fake_dir="$TEST_TMP/second-create"
    make_fakes "$fake_dir"
    if FAKE_MODE=a-second-create run_with_fakes "$fake_dir" run --confirm-token-spend >/dev/null 2>&1; then
        fail 'second unmarked create_entity was accepted'
    fi
    assert_two_agent_contract "$fake_dir"
    assert_eq "$(jq -r '.validation.status' "$(latest_manifest "$fake_dir")")" 'failed'
}

test_b_evidence_failures_are_rejected() {
    local mode fake_dir
    for mode in b-final-fabricated b-result-fabricated b-order b-cursor \
        b-unexpected-argument b-boolean-limit b-float-limit b-low-limit b-high-limit; do
        fake_dir="$TEST_TMP/$mode"
        make_fakes "$fake_dir"
        if FAKE_MODE="$mode" run_with_fakes "$fake_dir" run --confirm-token-spend >/dev/null 2>&1; then
            fail "$mode unexpectedly passed"
        fi
        assert_two_agent_contract "$fake_dir"
        assert_eq "$(jq -r '.validation.status' "$(latest_manifest "$fake_dir")")" 'failed'
    done
}

test_native_tool_evidence_failures_are_rejected() {
    local mode fake_dir
    for mode in code-mode-host-disabled other-error a-program; do
        fake_dir="$TEST_TMP/$mode"
        make_fakes "$fake_dir"
        if FAKE_MODE="$mode" run_with_fakes "$fake_dir" run --confirm-token-spend >/dev/null 2>&1; then
            fail "$mode unexpectedly passed"
        fi
        assert_two_agent_contract "$fake_dir"
        assert_eq "$(jq -r '.validation.status' "$(latest_manifest "$fake_dir")")" 'failed'
    done
}

test_ambiguous_create_is_dropped_from_initial_manifest_intent() {
    local fake_dir="$TEST_TMP/ambiguous-create" manifest database
    make_fakes "$fake_dir"
    if FAKE_MODE=ambiguous-create run_with_fakes "$fake_dir" run --confirm-token-spend >/dev/null 2>&1; then
        fail 'ambiguous database create unexpectedly passed'
    fi
    manifest="$(latest_manifest "$fake_dir")"
    database="$(jq -r '.deployment.database' "$manifest")"
    assert_eq "$(stat -f '%Lp' "$manifest")" '600'
    assert_eq "$(jq -r '.cleanup.intent' "$manifest")" 'drop_if_exists'
    assert_line "$fake_dir/state/database-actions" "create $database"
    assert_line "$fake_dir/state/database-actions" "drop $database"
    [[ ! -f "$fake_dir/state/codex-count" ]] || fail 'ambiguous create started an Agent'
}

test_term_signal_uses_cleanup_route() {
    local fake_dir="$TEST_TMP/signal" runner_pid manifest database
    make_fakes "$fake_dir"
    printf '%s\n' signal >"$fake_dir/state/mode"
    env PATH="$fake_dir/bin:/usr/bin:/bin" \
        CODEX_BIN="$fake_dir/bin/codex-fake" \
        DATABASE_URL='postgresql:///fake' \
        FAKE_STATE="$fake_dir/state" \
        FAKE_SECRET='must-not-reach-agent' \
        AICADIA_PLAYTEST_OUTPUT_ROOT="$fake_dir/output" \
        AICADIA_AGENT_TIMEOUT_SECONDS=30 \
        AICADIA_PLAYTEST_TEST_MODE=fake \
        AICADIA_PLAYTEST_TEST_TIMEOUT_SECONDS=1 \
        "$RUNNER" run --confirm-token-spend >/dev/null 2>&1 &
    runner_pid=$!
    for _ in $(seq 1 100); do
        [[ -f "$fake_dir/state/database-actions" ]] && grep -q '^create ' "$fake_dir/state/database-actions" && break
        sleep 0.02
    done
    kill -TERM "$runner_pid"
    wait "$runner_pid" 2>/dev/null || true
    manifest="$(latest_manifest "$fake_dir")"
    database="$(jq -r '.deployment.database' "$manifest")"
    assert_line "$fake_dir/state/database-actions" "drop $database"
    assert_eq "$(jq -r '.cleanup.status' "$manifest")" 'dropped'
    assert_eq "$(jq -r '.run_status' "$manifest")" 'interrupted'
}

test_zero_agent_paths
test_schema_semantic_preflight_rejects_missing_status_type
test_happy_path
test_provisioning_failure_starts_no_agent_and_retains_evidence
test_a_failure_still_runs_b_once
test_second_unmarked_create_is_rejected
test_b_evidence_failures_are_rejected
test_native_tool_evidence_failures_are_rejected
test_ambiguous_create_is_dropped_from_initial_manifest_intent
test_term_signal_uses_cleanup_route
printf 'agent-playtest fake integration tests passed\n'
