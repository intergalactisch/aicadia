#!/usr/bin/env bash
set -euo pipefail

readonly REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readonly LAUNCHER="$REPO_DIR/tools/aicadia-local"
readonly AGENT="$REPO_DIR/tools/aicadia-agent"
readonly ADMIN_DATABASE_URL="${DATABASE_URL:-postgres://localhost:5433/postgres}"
readonly TEST_ROOT="$(mktemp -d)"
readonly STATE_DIR="$TEST_ROOT/state"
readonly FAKE_BIN="$TEST_ROOT/bin"
readonly SOURCE_CODEX_HOME="$TEST_ROOT/source-codex-home"
readonly DATABASE_NAME="aicadia_local_test_${$}_$(date +%s)"
readonly PORT="$((43000 + ($$ % 10000)))"
readonly SECOND_PORT="$((PORT + 1))"
LAUNCHER_PID=''
DATABASE_CLEANUP_ARMED=0

fail() { printf 'aicadia-local test: %s\n' "$*" >&2; exit 1; }

cleanup() {
    local status=$?
    trap - EXIT INT TERM
    if [[ -n "$LAUNCHER_PID" ]] && kill -0 "$LAUNCHER_PID" 2>/dev/null; then
        kill "$LAUNCHER_PID" 2>/dev/null || true
        wait "$LAUNCHER_PID" 2>/dev/null || true
    fi
    if (( DATABASE_CLEANUP_ARMED == 1 )); then
        dropdb --if-exists --maintenance-db="$ADMIN_DATABASE_URL" "$DATABASE_NAME" >/dev/null 2>&1 || true
    fi
    rm -rf "$TEST_ROOT"
    exit "$status"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM

for dependency in psql dropdb curl jq lsof cargo; do
    command -v "$dependency" >/dev/null 2>&1 || fail "missing test dependency: $dependency"
done

existing_test_database="$(psql "$ADMIN_DATABASE_URL" --no-psqlrc --tuples-only --no-align \
    --command "SELECT count(*) FROM pg_database WHERE datname = '$DATABASE_NAME'")"
[[ "$existing_test_database" == 0 ]] || fail "refusing to reuse disposable database $DATABASE_NAME"
DATABASE_CLEANUP_ARMED=1

if env AICADIA_DATABASE_NAME='Bad-name' AICADIA_PORT="$PORT" \
    "$LAUNCHER" --no-open >"$TEST_ROOT/invalid-name.stdout" 2>"$TEST_ROOT/invalid-name.stderr"; then
    fail 'invalid database name unexpectedly succeeded'
fi
grep -F 'AICADIA_DATABASE_NAME must start with a lowercase letter' "$TEST_ROOT/invalid-name.stderr" >/dev/null \
    || fail 'invalid database name failure was not explicit'

if env AICADIA_DATABASE_NAME="$DATABASE_NAME" AICADIA_PORT=0 \
    "$LAUNCHER" --no-open >"$TEST_ROOT/invalid-port.stdout" 2>"$TEST_ROOT/invalid-port.stderr"; then
    fail 'invalid port unexpectedly succeeded'
fi
grep -F 'AICADIA_PORT must be an integer from 1 through 65535' "$TEST_ROOT/invalid-port.stderr" >/dev/null \
    || fail 'invalid port failure was not explicit'

if env PATH=/usr/bin:/bin:/usr/sbin AICADIA_DATABASE_NAME="$DATABASE_NAME" AICADIA_PORT="$PORT" \
    /bin/bash "$LAUNCHER" --no-open >"$TEST_ROOT/dependency.stdout" 2>"$TEST_ROOT/dependency.stderr"; then
    fail 'missing dependency unexpectedly succeeded'
fi
grep -F 'required program is unavailable: cargo' "$TEST_ROOT/dependency.stderr" >/dev/null \
    || fail 'missing dependency failure was not explicit'

mkdir -p "$FAKE_BIN"
mkdir -p "$SOURCE_CODEX_HOME"
printf '%s\n' '{"auth_mode":"fake"}' >"$SOURCE_CODEX_HOME/auth.json"
chmod 600 "$SOURCE_CODEX_HOME/auth.json"
cat >"$FAKE_BIN/codex" <<'FAKE'
#!/usr/bin/env bash
if [[ -n "${AICADIA_AGENT_FAKE_RECORD_DIR:-}" ]]; then
    pwd -P >"$AICADIA_AGENT_FAKE_RECORD_DIR/cwd"
    find . -mindepth 1 -maxdepth 1 -print >"$AICADIA_AGENT_FAKE_RECORD_DIR/cwd-entries"
    printf '%s\n' "$HOME" >"$AICADIA_AGENT_FAKE_RECORD_DIR/home"
    printf '%s\n' "$CODEX_HOME" >"$AICADIA_AGENT_FAKE_RECORD_DIR/codex-home"
    [[ -f "$CODEX_HOME/auth.json" && ! -e "$CODEX_HOME/config.toml" ]] || exit 96
    (cd "$CODEX_HOME" && find . -mindepth 1 -maxdepth 1 -print) >"$AICADIA_AGENT_FAKE_RECORD_DIR/codex-home-entries"
    jq -n --args '$ARGS.positional' -- "$@" >"$AICADIA_AGENT_FAKE_RECORD_DIR/args.json"
    printf '%s\n' "${AICADIA_USER_ID:-}" >"$AICADIA_AGENT_FAKE_RECORD_DIR/user-id"
    exit "${AICADIA_AGENT_FAKE_STATUS:-0}"
fi
printf 'invoked\n' >"$AICADIA_LOCAL_CODEX_MARKER"
exit 97
FAKE
chmod +x "$FAKE_BIN/codex"
readonly CODEX_MARKER="$TEST_ROOT/codex-invoked"

launcher_env=(
    env
    -u AICADIA_USER_ID
    "PATH=$FAKE_BIN:$PATH"
    "CODEX_HOME=$SOURCE_CODEX_HOME"
    "AICADIA_LOCAL_CODEX_MARKER=$CODEX_MARKER"
    "DATABASE_URL=$ADMIN_DATABASE_URL"
    "AICADIA_DATABASE_NAME=$DATABASE_NAME"
    "AICADIA_PORT=$PORT"
    AICADIA_LOCAL_TESTING=1
    "AICADIA_LOCAL_TEST_STATE_DIR=$STATE_DIR"
)

start_launcher() {
    local stdout="$1" stderr="$2"
    "${launcher_env[@]}" "$LAUNCHER" --no-open >"$stdout" 2>"$stderr" &
    LAUNCHER_PID=$!
    for _attempt in $(seq 1 300); do
        if grep -q '^AICADIA_USER_ID=' "$stdout" 2>/dev/null; then
            return 0
        fi
        if ! kill -0 "$LAUNCHER_PID" 2>/dev/null; then
            wait "$LAUNCHER_PID" 2>/dev/null || true
            LAUNCHER_PID=''
            cat "$stderr" >&2
            fail 'launcher exited before handoff output'
        fi
        sleep 0.1
    done
    cat "$stderr" >&2
    fail 'launcher did not print handoff output within 30 seconds'
}

stop_launcher() {
    local pid="$LAUNCHER_PID"
    kill "$pid"
    wait "$pid" 2>/dev/null || true
    LAUNCHER_PID=''
    for _attempt in $(seq 1 100); do
        lsof -nP -iTCP:"$PORT" -sTCP:LISTEN 2>/dev/null | grep -q . || return 0
        sleep 0.1
    done
    fail "owned server still listens on port $PORT after launcher stop"
}

user_count() {
    psql "$ADMIN_DATABASE_URL" --no-psqlrc --tuples-only --no-align \
        --command "\\connect \"$DATABASE_NAME\"" --command 'SELECT count(*) FROM "user"'
}

first_stdout="$TEST_ROOT/first.stdout"
first_stderr="$TEST_ROOT/first.stderr"
start_launcher "$first_stdout" "$first_stderr"
first_user_id="$(sed -n "s/^AICADIA_USER_ID='\([^']*\)' AICADIA_PORT='$PORT' \.\/tools\/aicadia-agent$/\1/p" "$first_stdout")"
[[ "$first_user_id" =~ ^[0-9a-f-]{36}$ ]] || fail 'first start did not print the exact Agent handoff'
grep -Fx "Ledger URL: http://127.0.0.1:$PORT/#user_id=$first_user_id" "$first_stdout" >/dev/null \
    || fail 'first start did not print the exact ledger URL'
grep -Fx "MCP URL: http://127.0.0.1:$PORT/mcp" "$first_stdout" >/dev/null \
    || fail 'first start did not print the MCP URL'
[[ "$(user_count | tail -1)" == 1 ]] || fail 'first start did not provision exactly one User'
[[ "$(stat -f '%Lp' "$STATE_DIR")" == 700 ]] || fail 'state directory mode is not 0700'
[[ "$(stat -f '%Lp' "$STATE_DIR/profile.json")" == 600 ]] || fail 'profile mode is not 0600'
jq -e --arg database "$DATABASE_NAME" --arg user "$first_user_id" \
    'keys == ["database_name", "user_id", "version"] and .version == 1 and .database_name == $database and .user_id == $user' \
    "$STATE_DIR/profile.json" >/dev/null || fail 'profile content is not exact'
grep -F "$ADMIN_DATABASE_URL" "$STATE_DIR/profile.json" >/dev/null 2>&1 \
    && fail 'profile persisted database credentials'

if "${launcher_env[@]}" "$AGENT" >"$TEST_ROOT/agent-missing.stdout" 2>"$TEST_ROOT/agent-missing.stderr"; then
    fail 'Agent start without User context unexpectedly succeeded'
fi
grep -F 'AICADIA_USER_ID must contain the stable local User UUID' "$TEST_ROOT/agent-missing.stderr" >/dev/null \
    || fail 'missing Agent User context failure was not explicit'

if "${launcher_env[@]}" 'AICADIA_USER_ID=00000000-0000-4000-8000-000000000000' \
    "$AGENT" >"$TEST_ROOT/agent-wrong-user.stdout" 2>"$TEST_ROOT/agent-wrong-user.stderr"; then
    fail 'Agent start with mismatched User unexpectedly succeeded'
fi
grep -F 'does not match the stable local profile' "$TEST_ROOT/agent-wrong-user.stderr" >/dev/null \
    || fail 'mismatched Agent User failure was not explicit'

agent_record_dir="$TEST_ROOT/agent-record"
agent_tmp="$TEST_ROOT/agent-tmp"
mkdir -p "$agent_record_dir" "$agent_tmp"
agent_tmp_real="$(cd "$agent_tmp" && pwd -P)"
"${launcher_env[@]}" "AICADIA_USER_ID=$first_user_id" "TMPDIR=$agent_tmp" \
    "AICADIA_AGENT_FAKE_RECORD_DIR=$agent_record_dir" "$AGENT"
agent_cwd="$(cat "$agent_record_dir/cwd")"
agent_root="$(dirname "$agent_cwd")"
agent_home="$(cat "$agent_record_dir/home")"
agent_codex_home="$(cat "$agent_record_dir/codex-home")"
case "$agent_root" in
    "$agent_tmp_real"/aicadia-player.*) ;;
    *) fail 'Agent did not start under its isolated external player root' ;;
esac
[[ "$agent_cwd" == "$agent_root/workspace" ]] \
    || fail 'Agent did not start in its isolated workspace'
[[ "$agent_home" == "$agent_root/home" && "$agent_codex_home" == "$agent_root/home/.codex" ]] \
    || fail 'Agent did not receive its isolated home and Codex configuration'
[[ ! -e "$agent_root" ]] || fail 'Agent player root remained after exit'
[[ ! -s "$agent_record_dir/cwd-entries" ]] || fail 'Agent working directory was not empty at Codex start'
[[ "$(cat "$agent_record_dir/codex-home-entries")" == './auth.json' ]] \
    || fail 'Agent inherited configuration beyond its transient authentication copy'
[[ -f "$SOURCE_CODEX_HOME/auth.json" && ! -e "$SOURCE_CODEX_HOME/config.toml" ]] \
    || fail 'Agent changed its source Codex authentication directory'
[[ "$(cat "$agent_record_dir/user-id")" == "$first_user_id" ]] \
    || fail 'Agent did not receive the stable User context'
jq -e --arg url "mcp_servers.aicadia.url=\"http://127.0.0.1:$PORT/mcp\"" '
    index("--enable") != null
    and index("mcp_2026_07_28") != null
    and index("mcp_servers.aicadia.enabled=true") != null
    and index("mcp_servers.aicadia.required=true") != null
    and index("mcp_servers.aicadia.default_tools_approval_mode=\"approve\"") != null
    and index("mcp_servers.aicadia.env_http_headers={\"Aicadia-User-Id\"=\"AICADIA_USER_ID\"}") != null
    and index($url) != null
    and index("shell_tool") != null
    and index("unified_exec") != null
    and all(.[]; contains("enabled_tools") | not)
    and index("--model") == null
' "$agent_record_dir/args.json" >/dev/null || fail 'Agent received an unexpected current MCP command'
contract_argument="$(jq -r '.[] | select(startswith("developer_instructions="))' "$agent_record_dir/args.json")"
[[ -n "$contract_argument" ]] || fail 'Agent did not receive the player contract as developer instructions'
served_contract="$(curl --silent --fail \
    --header 'Content-Type: application/json' \
    --header 'Accept: application/json, text/event-stream' \
    --header 'MCP-Protocol-Version: 2026-07-28' \
    --header 'Mcp-Method: server/discover' \
    --data '{"jsonrpc":"2.0","id":1,"method":"server/discover","params":{"_meta":{"io.modelcontextprotocol/protocolVersion":"2026-07-28","io.modelcontextprotocol/clientInfo":{"name":"aicadia-local-test","version":"1"},"io.modelcontextprotocol/clientCapabilities":{}}}}' \
    "http://127.0.0.1:$PORT/mcp" | jq -ec '.result.instructions | select(type == "string")')" \
    || fail 'the served player contract could not be read for comparison'
[[ "${contract_argument#developer_instructions=}" == "$served_contract" ]] \
    || fail 'Agent developer instructions differ from the served player contract'

if "${launcher_env[@]}" "AICADIA_USER_ID=$first_user_id" "AICADIA_PORT=$SECOND_PORT" \
    "TMPDIR=$agent_tmp" "AICADIA_AGENT_FAKE_RECORD_DIR=$agent_record_dir" \
    "$AGENT" >"$TEST_ROOT/agent-unavailable.stdout" 2>"$TEST_ROOT/agent-unavailable.stderr"; then
    fail 'Agent start without reachable Aicadia unexpectedly succeeded'
fi
grep -F 'Aicadia is not available' "$TEST_ROOT/agent-unavailable.stderr" >/dev/null \
    || fail 'unavailable Aicadia failure was not explicit'

concurrent_stdout="$TEST_ROOT/concurrent.stdout"
concurrent_stderr="$TEST_ROOT/concurrent.stderr"
if "${launcher_env[@]}" "AICADIA_PORT=$SECOND_PORT" \
    "$LAUNCHER" --no-open >"$concurrent_stdout" 2>"$concurrent_stderr"; then
    fail 'concurrent launch unexpectedly succeeded'
fi
grep -F 'another Aicadia local launcher is already active' "$concurrent_stderr" >/dev/null \
    || fail 'concurrent launch failure was not explicit'
kill -0 "$LAUNCHER_PID" 2>/dev/null || fail 'concurrent attempt stopped the active launcher'
lsof -nP -iTCP:"$SECOND_PORT" -sTCP:LISTEN 2>/dev/null | grep -q . \
    && fail 'concurrent attempt left a second server running'
[[ "$(user_count | tail -1)" == 1 ]] || fail 'concurrent attempt provisioned another User'

stop_launcher
database_after_stop="$(psql "$ADMIN_DATABASE_URL" --no-psqlrc --tuples-only --no-align \
    --command "SELECT count(*) FROM pg_database WHERE datname = '$DATABASE_NAME'")"
[[ "$database_after_stop" == 1 ]] || fail 'normal stop removed the database'

mv "$STATE_DIR/profile.json" "$TEST_ROOT/missing-profile.json"
if "${launcher_env[@]}" "$LAUNCHER" --no-open \
    >"$TEST_ROOT/missing.stdout" 2>"$TEST_ROOT/missing.stderr"; then
    fail 'existing database with a missing profile unexpectedly succeeded'
fi
grep -F 'is missing; refusing to provision another User' "$TEST_ROOT/missing.stderr" >/dev/null \
    || fail 'missing profile failure was not explicit'
[[ "$(user_count | tail -1)" == 1 ]] || fail 'missing profile failure provisioned another User'
mv "$TEST_ROOT/missing-profile.json" "$STATE_DIR/profile.json"

second_stdout="$TEST_ROOT/second.stdout"
second_stderr="$TEST_ROOT/second.stderr"
start_launcher "$second_stdout" "$second_stderr"
second_user_id="$(sed -n "s/^AICADIA_USER_ID='\([^']*\)' AICADIA_PORT='$PORT' \.\/tools\/aicadia-agent$/\1/p" "$second_stdout")"
[[ "$second_user_id" == "$first_user_id" ]] || fail 'restart changed the stable User id'
[[ "$(user_count | tail -1)" == 1 ]] || fail 'restart provisioned another User'
verified_user="$(curl --silent --show-error --header "Aicadia-User-Id: $first_user_id" \
    "http://127.0.0.1:$PORT/api/user")"
jq -e --arg id "$first_user_id" '.id == $id' <<<"$verified_user" >/dev/null \
    || fail 'restart did not expose the profiled User'

occupied_stdout="$TEST_ROOT/occupied.stdout"
occupied_stderr="$TEST_ROOT/occupied.stderr"
if "${launcher_env[@]}" "$LAUNCHER" --no-open >"$occupied_stdout" 2>"$occupied_stderr"; then
    fail 'occupied port launch unexpectedly succeeded'
fi
grep -F "loopback port $PORT is already occupied" "$occupied_stderr" >/dev/null \
    || fail 'occupied port failure was not explicit'
kill -0 "$LAUNCHER_PID" 2>/dev/null || fail 'occupied-port attempt stopped a server it did not own'
stop_launcher

cp "$STATE_DIR/profile.json" "$TEST_ROOT/good-profile.json"
printf '%s\n' '{not-json' >"$STATE_DIR/profile.json"
chmod 600 "$STATE_DIR/profile.json"
if "${launcher_env[@]}" "$LAUNCHER" --no-open >"$TEST_ROOT/corrupt.stdout" 2>"$TEST_ROOT/corrupt.stderr"; then
    fail 'corrupt profile launch unexpectedly succeeded'
fi
grep -F 'is corrupt or has an unsupported version' "$TEST_ROOT/corrupt.stderr" >/dev/null \
    || fail 'corrupt profile failure was not explicit'
[[ "$(user_count | tail -1)" == 1 ]] || fail 'corrupt profile failure provisioned another User'

jq '.user_id = "00000000-0000-4000-8000-000000000000"' "$TEST_ROOT/good-profile.json" >"$STATE_DIR/profile.json"
chmod 600 "$STATE_DIR/profile.json"
if "${launcher_env[@]}" "$LAUNCHER" --no-open >"$TEST_ROOT/stale.stdout" 2>"$TEST_ROOT/stale.stderr"; then
    fail 'stale profile launch unexpectedly succeeded'
fi
grep -F 'profiled User verification failed with HTTP 404' "$TEST_ROOT/stale.stderr" >/dev/null \
    || fail 'stale profile failure was not explicit'
for _attempt in $(seq 1 100); do
    lsof -nP -iTCP:"$PORT" -sTCP:LISTEN 2>/dev/null | grep -q . || break
    sleep 0.1
done
lsof -nP -iTCP:"$PORT" -sTCP:LISTEN 2>/dev/null | grep -q . \
    && fail 'stale-profile failure left its owned server running'
[[ "$(user_count | tail -1)" == 1 ]] || fail 'stale profile failure provisioned another User'

[[ ! -e "$CODEX_MARKER" ]] || fail 'launcher invoked Codex'
grep -E '(dropdb|DROP[[:space:]]+DATABASE|reset)' "$LAUNCHER" >/dev/null \
    && fail 'shipped launcher contains a database drop or reset path'
grep -Fx '/.aicadia-local/' "$REPO_DIR/.gitignore" >/dev/null \
    || fail 'local profile directory is not ignored'

printf 'aicadia-local lifecycle: passed\n'
printf 'database=%s\n' "$DATABASE_NAME"
printf 'user_id=%s\n' "$first_user_id"
printf 'restart_user_id=%s\n' "$second_user_id"
printf 'normal_stop_preserved_database=true\n'
printf 'missing_profile_failed_closed=true\n'
printf 'concurrent_launch_failed_closed=true\n'
printf 'corrupt_profile_failed_closed=true\n'
printf 'stale_profile_failed_closed=true\n'
printf 'occupied_port_failed_closed=true\n'
printf 'isolated_agent_handoff=true\n'
printf 'codex_invoked=false\n'
