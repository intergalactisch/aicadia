#!/usr/bin/env bash
set -euo pipefail

readonly REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readonly RUNNER="$REPO_DIR/tools/agent-playtest"
readonly STEERING='Make it a weathered cedar trail marker whose carving includes the unique playtest marker verbatim.'
TEST_TMP="$(mktemp -d)"
trap '[[ "${KEEP_AGENT_PLAYTEST_TMP:-0}" == 1 ]] || rm -rf "$TEST_TMP"' EXIT

fail() { printf 'agent-playtest test: %s\n' "$*" >&2; exit 1; }
assert_contains() { grep -F -- "$2" "$1" >/dev/null || fail "$1 lacks [$2]"; }
assert_not_contains() { ! grep -F -- "$2" "$1" >/dev/null || fail "$1 unexpectedly contains [$2]"; }
assert_eq() { [[ "$1" == "$2" ]] || fail "expected [$2], got [$1]"; }
assert_live_attempt_shape() {
    local events="$1" tools="$2"
    jq -s -e --argjson tools "$tools" '
        [.[]|select(.item?.type=="mcp_tool_call")|{event_type:.type,item:.item}] as $observed
        | ($tools|length) as $count
        | [$observed[]|select(.event_type=="item.started")|.item] as $started
        | [$observed[]|select(.event_type=="item.completed")|.item] as $completed
        | ($observed|length)==($count*2)
        and [$observed[].event_type]==([range(0;$count)|["item.started","item.completed"]]|add)
        and all($observed[];(.item|keys|sort)==(["arguments","error","id","result","server","status","tool","type"]|sort))
        and [$started[].id]==[$completed[].id] and ([$completed[].id]|unique|length)==$count
        and [$started[].tool]==$tools and [$completed[].tool]==$tools
        and [$started[].arguments]==[$completed[].arguments]
        and all($started[];.server=="aicadia" and .status=="in_progress" and .result==null and .error==null)
        and all($completed[];.server=="aicadia" and .status=="completed"
            and (.result|keys|sort)==(["content","structured_content"]|sort)
            and (.result.content|type)=="array" and (.result.structured_content|type)=="object" and .error==null)
    ' "$events" >/dev/null || fail "$events does not mirror live started/completed MCP attempts"
}

make_fakes() {
    local root="$1"
    mkdir -p "$root/bin" "$root/state" "$root/output"
    cp "$REPO_DIR/tests/agent-tool-catalog.json" "$root/state/catalog.json"

    cat >"$root/bin/cargo" <<'FAKE'
#!/usr/bin/env bash
set -euo pipefail
state="$(cd "$(dirname "$0")/../state" && pwd)"
mode="$(<"$state/mode")"
args="$*"
case "$args" in
    'build --quiet --bin aicadia --bin aicadia-provision-user --bin aicadia-playtest-database')
        printf 'build\n' >>"$state/preflight-actions" ;;
    run\ --quiet\ --bin\ aicadia-playtest-database\ --\ probe\ *)
        token="${!#}"
        printf 'probe-create-tag-verify-drop %s\n' "$token" >>"$state/database-actions"
        printf 'ownership_probe_passed aicadia_playtest_probe_%s\n' "${token:0:32}" ;;
    run\ --quiet\ --bin\ aicadia-playtest-database\ --\ create\ *)
        token="${!#}"; before_token=$(( $# - 1 )); database="${!before_token}"
        printf 'create-attempt %s %s\n' "$database" "$token" >>"$state/database-actions"
        if [[ "$mode" == ambiguous-create || "$mode" == ownership-mismatch-create ]]; then exit 19; fi
        printf '%s\n' "$token" >"$state/database-ownership-token"
        printf '%s\n' "$database" >"$state/database-name"
        printf 'ownership_verified %s %s\n' "$database" "$token" ;;
    run\ --quiet\ --bin\ aicadia-playtest-database\ --\ drop\ *)
        token="${!#}"; before_token=$(( $# - 1 )); database="${!before_token}"
        printf 'drop-attempt %s %s\n' "$database" "$token" >>"$state/database-actions"
        [[ -f "$state/database-name" && "$(<"$state/database-name")" == "$database" ]] || exit 20
        [[ -f "$state/database-ownership-token" && "$(<"$state/database-ownership-token")" == "$token" ]] || exit 21
        [[ "$mode" != cleanup-fail ]] || exit 19
        printf 'ownership_verified_and_dropped %s\n' "$database" ;;
    'run --quiet --bin aicadia')
        [[ "$mode" != server-fail ]] || exit 18
        printf '%s\n' '{"event":"server_ready","address":"127.0.0.1:45678"}'
        trap 'exit 0' TERM INT
        while :; do sleep 1; done ;;
    'run --quiet --bin aicadia-provision-user')
        count=0; [[ ! -f "$state/provision-count" ]] || count="$(<"$state/provision-count")"
        count=$((count + 1)); printf '%s\n' "$count" >"$state/provision-count"
        if [[ $count -eq 1 ]]; then id='11111111-1111-4111-8111-111111111111'; else id='22222222-2222-4222-8222-222222222222'; fi
        printf '{"id":"%s","created_at":"2026-08-11T12:00:00Z"}\n' "$id" ;;
    *) printf 'unexpected fake cargo: %s\n' "$args" >&2; exit 90 ;;
esac
FAKE
    cat >"$root/bin/openssl" <<'FAKE'
#!/usr/bin/env bash
set -euo pipefail
[[ "$*" == 'rand -hex 32' ]] || exit 90
printf '%s\n' 'abababababababababababababababababababababababababababababababab'
FAKE
    cat >"$root/bin/env" <<'FAKE'
#!/bin/bash
set -euo pipefail
state="$(cd "$(dirname "$0")/../state" && pwd)"
: >"$state/fake-env-invoked"
exec /usr/bin/env "$@"
FAKE

    cat >"$root/bin/curl" <<'FAKE'
#!/usr/bin/env bash
set -euo pipefail
state="$(cd "$(dirname "$0")/../state" && pwd)"
output=''; method='GET'; data=''; user=''; previous=''; url="${!#}"
for argument in "$@"; do
    case "$previous" in
        -o) output="$argument" ;;
        -X) method="$argument" ;;
        --data) data="$argument" ;;
        -H) [[ "$argument" != Aicadia-User-Id:* ]] || user="${argument#Aicadia-User-Id: }" ;;
    esac
    previous="$argument"
done
place_id='bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb'
action_character='aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa'
observer_character='dddddddd-dddd-4ddd-8ddd-dddddddddddd'
entity_id='cccccccc-cccc-4ccc-8ccc-cccccccccccc'
action_user='11111111-1111-4111-8111-111111111111'
observer_user='22222222-2222-4222-8222-222222222222'
marker=''
[[ ! -f "$state/marker" ]] || marker="$(<"$state/marker")"
place="$(jq -nc --arg id "$place_id" --arg marker "$marker" --arg user "$action_user" '{entity:{id:$id,name:("Entry Place "+$marker),description:("Disposable shared entry Place for "+$marker),introduced_by_user_id:$user,introduced_at:"2026-08-11T12:02:00Z"},is_entry:true}')"
place_summary="$(jq -nc --argjson place "$place" '$place.entity|{id,name,description}')"
character() {
    local id="$1" owner="$2" role="$3"
    jq -nc --arg id "$id" --arg owner "$owner" --arg marker "$marker" --arg role "$role" --argjson place "$place" \
        '{entity:{id:$id,name:($role+" Character "+$marker),description:("Disposable "+($role|ascii_downcase)+" Character for "+$marker),introduced_by_user_id:$owner,introduced_at:"2026-08-11T12:01:00Z"},owner_user_id:$owner,current_place:$place}'
}
character_state() {
    local id="$1" owner="$2" role="$3" revision="${4:-v1.fake.before}"
    jq -nc --argjson character "$(character "$id" "$owner" "$role")" --arg revision "$revision" \
        '{character:$character,place_revision:$revision,current_state:{association:[],next:null}}'
}
response='{}'; status=200
case "$url" in
    */api/openapi.json)
        jq -n '{paths:{"/api/world":{get:{operationId:"get_world"}},"/api/user":{get:{operationId:"get_user"}},"/api/character":{get:{operationId:"get_character"},post:{operationId:"create_character"}},"/api/place/entry":{post:{operationId:"create_entry_place"}},"/api/world/entry":{post:{operationId:"enter_world"}},"/api/activity":{get:{operationId:"list_activity"}},"/api/entity":{post:{operationId:"create_entity"}},"/api/place/current/entity":{get:{operationId:"list_entity_at_current_place"}},"/api/place/current/activity":{get:{operationId:"list_activity_at_current_place"}},"/api/place/current/entity/{entity_id}":{get:{operationId:"get_entity_at_current_place"}},"/api/investigation":{post:{operationId:"start_investigation"}},"/api/action":{post:{operationId:"submit_action"}},"/api/interaction":{post:{operationId:"submit_interaction"}},"/api/discovery":{post:{operationId:"submit_discovery"}}}}' >"$state/response"; response="$(<"$state/response")" ;;
    */mcp)
        response="$(jq -nc --slurpfile tools "$state/catalog.json" '{jsonrpc:"2.0",id:1,result:{tools:$tools[0]}}')" ;;
    */api/world) response='{"name":"Aicadia"}' ;;
    */api/user)
        if [[ "$output" == /dev/null ]]; then status=404; response='{"error":{"code":"user_not_found"}}';
        else response="$(jq -nc --arg id "$user" '{id:$id,created_at:"2026-08-11T12:00:00Z"}')"; fi ;;
    */api/character)
        if [[ "$method" == POST ]]; then
            if [[ "$user" == "$action_user" ]]; then response="$(character "$action_character" "$action_user" Action | jq '.current_place=null')";
            else response="$(character "$observer_character" "$observer_user" Observer | jq '.current_place=null')"; fi
            status=201
        elif [[ "$user" == "$action_user" ]]; then response="$(character_state "$action_character" "$action_user" Action "v1.fake.after")";
        else response="$(character_state "$observer_character" "$observer_user" Observer "v1.fake.after")"; fi ;;
    */api/world/entry)
        if [[ ! -f "$state/place-created" ]]; then status=404; response='{"error":{"code":"entry_place_not_found"}}';
        elif [[ "$user" == "$action_user" ]]; then response="$(character "$action_character" "$action_user" Action)";
        else response="$(character "$observer_character" "$observer_user" Observer)"; fi ;;
    */api/place/entry)
        : >"$state/place-created"; response="$place"; status=201 ;;
    */api/place/current/entity/$entity_id*)
        printf 'http-current-entity-state\n' >>"$state/timeline"
        entity="$(jq '.entity' "$state/accepted.json")"
        response="$(jq -nc --argjson place "$place_summary" --argjson entity "$entity" '{place:$place,place_revision:"v1.fake.after",entity:($entity|{id,name,description}),current_state:{association:[{type:"property",property:{key:"material",value:{type:"text",text:"weathered cedar"}}}],next:null}}')" ;;
    */api/place/current/entity*)
        printf 'http-current-entity\n' >>"$state/timeline"
        if [[ -f "$state/accepted.json" ]]; then
            entity="$(jq '.entity|{id,name,description}' "$state/accepted.json")"
            other_entity="$(character "$action_character" "$action_user" Action | jq '.entity|{id,name,description}')"
            if [[ "$(<"$state/mode")" == http-duplicate-entity ]]; then
                response="$(jq -nc --argjson place "$place_summary" --argjson entity "$entity" --argjson other "$other_entity" '{place:$place,place_revision:"v1.fake.after",entity:[$entity,$entity,$other],next:null}')"
            else
                response="$(jq -nc --argjson place "$place_summary" --argjson entity "$entity" --argjson other "$other_entity" '{place:$place,place_revision:"v1.fake.after",entity:[$entity,$other],next:null}')"
            fi
        else response="$(jq -nc --argjson place "$place_summary" '{place:$place,place_revision:"v1.fake.before",entity:[],next:null}')"; fi ;;
    */api/place/current/activity*)
        printf 'http-current-activity\n' >>"$state/timeline"
        if [[ -f "$state/accepted.json" ]]; then
            activity="$(jq '.activity' "$state/accepted.json")"
            if [[ "$(<"$state/mode")" == http-wrong-actor ]]; then
                activity="$(jq '.actor_character.id="ffffffff-ffff-4fff-8fff-ffffffffffff"' <<<"$activity")"
            fi
            if [[ "$(<"$state/mode")" == http-duplicate-action ]]; then
                response="$(jq -nc --argjson place "$place_summary" --argjson activity "$activity" '{place:$place,place_revision:"v1.fake.after",activity:[$activity,$activity],next:null}')"
            else
                response="$(jq -nc --argjson place "$place_summary" --argjson activity "$activity" '{place:$place,place_revision:"v1.fake.after",activity:[$activity],next:null}')"
            fi
        else response="$(jq -nc --argjson place "$place_summary" '{place:$place,place_revision:"v1.fake.before",activity:[],next:null}')"; fi ;;
    *) printf 'unexpected fake curl: %s\n' "$*" >&2; exit 91 ;;
esac
if [[ -n "$output" ]]; then
    [[ "$output" == /dev/null ]] || printf '%s\n' "$response" >"$output"
    printf '%s' "$status"
else
    printf '%s\n' "$response"
fi
FAKE

    cat >"$root/bin/codex-fake" <<'FAKE'
#!/usr/bin/env bash
set -euo pipefail
state="$(cd "$(dirname "$0")/../state" && pwd)"
mode="$(<"$state/mode")"
printf '%s\n' "$*" >>"$state/codex-commands"
case "$*" in
    --version) [[ "$mode" != cli-drift ]] && printf 'codex-cli 0.147.0\n' || printf 'codex-cli 0.148.0\n'; exit 0 ;;
    'login status') printf 'Logged in using ChatGPT\n'; exit 0 ;;
    'debug models') printf '%s\n' '{"models":[{"slug":"gpt-5.6-sol","supported_reasoning_levels":[{"effort":"high"}]}]}'; exit 0 ;;
    'features list') : ;;
    'exec --help') printf '%s\n' '--ignore-user-config --ignore-rules --strict-config --skip-git-repo-check --json --output-schema --output-last-message'; exit 0 ;;
    'exec resume --help') printf '%s\n' 'Usage [SESSION_ID] --ignore-user-config --ignore-rules --strict-config --skip-git-repo-check --json --output-schema --output-last-message'; exit 0 ;;
    --help) printf '%s\n' '--ask-for-approval --sandbox --model'; exit 0 ;;
esac
if [[ "$*" == *'features list' ]]; then
    for feature in apps auth_elicitation browser_use browser_use_external browser_use_full_cdp_access code_mode code_mode_host computer_use goals hooks image_generation in_app_browser mcp_2026_07_28 multi_agent multi_agent_v2 plugins remote_plugin shell_snapshot shell_tool skill_mcp_dependency_install tool_call_mcp_elicitation tool_suggest unified_exec workspace_dependencies; do
        enabled=true; [[ "$*" != *"--disable $feature"* ]] || enabled=false
        [[ "$feature" != code_mode && "$feature" != code_mode_host ]] || enabled=true
        [[ "$feature" != mcp_2026_07_28 || "$*" == *"--enable mcp_2026_07_28"* ]] || enabled=false
        printf '%-40s stable %s\n' "$feature" "$enabled"
    done
    exit 0
fi
if [[ "$*" == *'mcp get aicadia --json' ]]; then
    arguments="$(printf '%s\n' "$@")"
    if [[ "$*" == *'enabled_tools=["submit_discovery","get_character","list_entity_at_current_place","get_entity_at_current_place","list_activity_at_current_place"]'* ]]; then tools='["submit_discovery","get_character","list_entity_at_current_place","get_entity_at_current_place","list_activity_at_current_place"]';
    elif [[ "$*" == *'enabled_tools=["get_world","get_character","list_entity_at_current_place","get_entity_at_current_place","list_activity_at_current_place","start_investigation"]'* ]]; then tools='["get_world","get_character","list_entity_at_current_place","get_entity_at_current_place","list_activity_at_current_place","start_investigation"]';
    elif [[ "$*" == *'enabled_tools=["submit_discovery"]'* ]]; then tools='["submit_discovery"]';
    elif [[ "$*" == *'enabled_tools=["get_world","get_character","list_entity_at_current_place","list_activity_at_current_place"]'* ]]; then tools='["get_world","get_character","list_entity_at_current_place","list_activity_at_current_place"]';
    elif [[ "$*" == *'enabled_tools=["submit_action"]'* ]]; then tools='["submit_action"]';
    elif [[ "$*" == *'enabled_tools=["get_character","list_entity_at_current_place","get_entity_at_current_place","list_activity_at_current_place"]'* ]]; then tools='["get_character","list_entity_at_current_place","get_entity_at_current_place","list_activity_at_current_place"]';
    else exit 96; fi
    for required in 'features.code_mode.enabled=true' 'features.code_mode.direct_only_tool_namespaces=["mcp__aicadia"]'; do grep -Fx -- "$required" <<<"$arguments" >/dev/null || exit 97; done
    jq -nc --argjson tools "$tools" '{name:"aicadia",enabled:true,transport:{type:"streamable_http",url:"http://127.0.0.1:9/mcp",env_http_headers:{"Aicadia-User-Id":"AICADIA_USER_ID"}},enabled_tools:$tools,disabled_tools:null}'
    exit 0
fi

count=0; [[ ! -f "$state/codex-exec-count" ]] || count="$(<"$state/codex-exec-count")"
count=$((count + 1)); printf '%s\n' "$count" >"$state/codex-exec-count"
printf 'agent-%s\n' "$count" >>"$state/timeline"
call="$state/call-$count"; mkdir "$call"
printf '%s\n' "$@" >"$call/argv"
/usr/bin/env | sort >"$call/environment"; pwd >"$call/cwd"
final=''; previous=''
for argument in "$@"; do [[ "$previous" != --output-last-message ]] || final="$argument"; previous="$argument"; done
prompt="${!#}"; printf '%s\n' "$prompt" >"$call/prompt"
marker="$(sed -n 's/.*\(aicadia-action-playtest-[0-9A-Za-z_-]*\).*/\1/p' <<<"$prompt" | head -1)"
[[ -z "$marker" ]] || printf '%s\n' "$marker" >"$state/marker"
[[ -s "$state/marker" ]] && marker="$(<"$state/marker")"
session='33333333-3333-4333-8333-333333333333'
place_id='bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb'; character_id='aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa'; observer_id='dddddddd-dddd-4ddd-8ddd-dddddddddddd'
entity_id='cccccccc-cccc-4ccc-8ccc-cccccccccccc'; activity_id='eeeeeeee-eeee-4eee-8eee-eeeeeeeeeeee'; request_id='44444444-4444-4444-8444-444444444444'
action_user='11111111-1111-4111-8111-111111111111'
place="$(jq -nc --arg id "$place_id" --arg marker "$marker" --arg user "$action_user" '{entity:{id:$id,name:("Entry Place "+$marker),description:("Disposable shared entry Place for "+$marker),introduced_by_user_id:$user,introduced_at:"2026-08-11T12:02:00Z"},is_entry:true}')"
place_summary="$(jq -nc --argjson place "$place" '$place.entity|{id,name,description}')"
printf '%s\n' "{\"type\":\"thread.started\",\"thread_id\":\"$session\"}"
printf '%s\n' '{"type":"turn.started"}'
mcp_sequence=0
mcp() {
    local tool="$1" arguments="$2" result="$3" id="item_$mcp_sequence"
    mcp_sequence=$((mcp_sequence + 1))
    jq -nc --arg id "$id" --arg tool "$tool" --argjson arguments "$arguments" \
        '{type:"item.started",item:{id:$id,type:"mcp_tool_call",server:"aicadia",tool:$tool,arguments:$arguments,result:null,status:"in_progress",error:null}}'
    jq -nc --arg id "$id" --arg tool "$tool" --argjson arguments "$arguments" --argjson result "$result" \
        '{type:"item.completed",item:{id:$id,type:"mcp_tool_call",server:"aicadia",tool:$tool,arguments:$arguments,result:{content:[{type:"text",text:($result|tojson)}],structured_content:$result},status:"completed",error:null}}'
}
mcp_started() { jq -nc --arg id "$1" --arg tool "$2" '{type:"item.started",item:{id:$id,type:"mcp_tool_call",server:"aicadia",tool:$tool,arguments:{},result:null,status:"in_progress",error:null}}'; }
if [[ "$mode" == investigation-happy ]]; then
    delivery_uncertain() {
        local tool="$1" arguments="$2" result="$3"
        jq -nc --arg tool "$tool" --argjson arguments "$arguments" --argjson result "$result" \
            '{type:"aicadia.test.delivery_uncertain",server:"aicadia",tool:$tool,arguments:$arguments,persisted_result:$result}'
    }
    mcp_error() {
        local tool="$1" arguments="$2" code="$3" id="item_$mcp_sequence"
        mcp_sequence=$((mcp_sequence + 1))
        jq -nc --arg id "$id" --arg tool "$tool" --argjson arguments "$arguments" \
            '{type:"item.started",item:{id:$id,type:"mcp_tool_call",server:"aicadia",tool:$tool,arguments:$arguments,result:null,status:"in_progress",error:null}}'
        jq -nc --arg id "$id" --arg tool "$tool" --argjson arguments "$arguments" --arg code "$code" \
            '{type:"item.completed",item:{id:$id,type:"mcp_tool_call",server:"aicadia",tool:$tool,arguments:$arguments,result:{content:[{type:"text",text:({error:{code:$code}}|tojson)}],is_error:true},status:"completed",error:null}}'
    }
    observer_entity="$(jq -nc --arg id "$observer_id" --arg marker "$marker" '{id:$id,name:("Observer Character "+$marker),description:("Disposable observer Character for "+$marker)}')"
    character_entity="$(jq -nc --arg id "$character_id" --arg user "$action_user" --arg marker "$marker" --argjson place "$place" '{entity:{id:$id,name:("Action Character "+$marker),description:("Disposable action Character for "+$marker),introduced_by_user_id:$user,introduced_at:"2026-08-11T12:01:00Z"},owner_user_id:$user,current_place:$place}')"
    character="$(jq -nc --argjson character "$character_entity" '{character:$character,place_revision:"v1.fake.discovery",current_state:{association:[],next:null}}')"
    entity_page="$(jq -nc --argjson place "$place_summary" --argjson entity "$observer_entity" '{place:$place,place_revision:"v1.fake.discovery",entity:[$entity],next:null}')"
    observer_state="$(jq -nc --argjson place "$place_summary" --argjson entity "$observer_entity" '{place:$place,place_revision:"v1.fake.discovery",entity:$entity,current_state:{association:[],next:null}}')"
    activity_page="$(jq -nc --argjson place "$place_summary" '{place:$place,place_revision:"v1.fake.discovery",activity:[],next:null}')"
    zero_request='10000000-0000-4000-8000-000000000000'; zero_attempt='20000000-0000-4000-8000-000000000000'
    positive_request='30000000-0000-4000-8000-000000000000'; positive_attempt='40000000-0000-4000-8000-000000000000'
    discovery_request='50000000-0000-4000-8000-000000000000'; trait_id='90000000-0000-4000-8000-000000000000'
    zero="$(jq -nc --arg attempt "$zero_attempt" '{attempt_id:$attempt,outcome:"zero",limits:{result_count:1,kind:"entity_at_current_place"}}')"
    positive="$(jq -nc --arg attempt "$positive_attempt" '{attempt_id:$attempt,outcome:"positive",limits:{result_count:1,kind:"entity_at_current_place"}}')"
    package="$(jq -nc --arg marker "$marker" '{prose:("Mara parts the rain-dark reeds and finds chalk-pale rainbell cups beside "+$marker+"."),find:{name:("Rainbell Cups "+$marker),description:("Chalk-pale cups whose thin rims ring in rain beside "+$marker+"."),property:[{key:"colour",value:{type:"text",text:"chalk-pale"}}],trait:[{statement:"Rings softly when collected rain shifts."}]}}')"
    case "$count" in
        1)
            mcp get_world '{}' '{"name":"Aicadia"}'; mcp get_character '{}' "$character"
            mcp list_entity_at_current_place '{}' "$entity_page"; mcp list_activity_at_current_place '{}' "$activity_page"
            delivery_uncertain start_investigation "$(jq -nc --arg id "$zero_request" '{request_id:$id}')" "$zero"
            mcp start_investigation "$(jq -nc --arg id "$zero_request" '{request_id:$id}')" "$zero"
            jq -n '{status:"zero",player_message:"Mara searches the rain-dark verge carefully and finds nothing new."}' >"$final" ;;
        2)
            mcp get_world '{}' '{"name":"Aicadia"}'; mcp get_character '{}' "$character"
            mcp list_entity_at_current_place '{}' "$entity_page"; mcp list_activity_at_current_place '{}' "$activity_page"
            delivery_uncertain start_investigation "$(jq -nc --arg id "$positive_request" '{request_id:$id}')" "$positive"
            mcp start_investigation "$(jq -nc --arg id "$positive_request" '{request_id:$id}')" "$positive"
            mcp get_character '{}' "$character"; mcp list_entity_at_current_place '{}' "$entity_page"
            mcp get_entity_at_current_place "$(jq -nc --arg id "$observer_id" '{entity_id:$id}')" "$observer_state"
            mcp list_activity_at_current_place '{}' "$activity_page"
            jq -n --argjson package "$package" '{status:"previewed",package:$package}' >"$final" ;;
        3)
            entity="$(jq -nc --arg id "$entity_id" --arg user "$action_user" --argjson package "$package" '{id:$id,name:$package.find.name,description:$package.find.description,introduced_by_user_id:$user,introduced_at:"2026-08-11T12:10:00Z"}')"
            activity="$(jq -nc --arg id "$activity_id" --arg character "$character_id" --arg place "$place_id" --arg entity "$entity_id" --arg trait "$trait_id" --arg marker "$marker" --argjson package "$package" '{id:$id,operation:"submit_discovery",actor_character:{id:$character,name:("Action Character "+$marker)},context_place:{entity:{id:$place,name:("Entry Place "+$marker)},is_entry:true},involved_entity:[{entity:{id:$entity,name:$package.find.name},role:"subject"},{entity:{id:$place,name:("Entry Place "+$marker)},role:"location"}],property_change:[{entity:{id:$entity,name:$package.find.name},key:$package.find.property[0].key,value:$package.find.property[0].value}],trait_change:[{type:"establish",entity:{id:$entity,name:$package.find.name},trait:{id:$trait,statement:$package.find.trait[0].statement}}],prose:$package.prose,occurred_at:"2026-08-11T12:10:00Z"}')"
            result="$(jq -nc --argjson activity "$activity" --argjson entity "$entity" --argjson place "$place" '{activity:$activity,entity:$entity,place:$place}')"
            arguments="$(jq -nc --arg request "$discovery_request" --arg attempt "$positive_attempt" --argjson package "$package" '{request_id:$request,attempt_id:$attempt,prose:$package.prose,find:$package.find}')"
            delivery_uncertain submit_discovery "$arguments" "$result"; mcp submit_discovery "$arguments" "$result"
            jq -n --argjson activity "$activity" --argjson entity "$entity" '{activity:$activity,entity:$entity}' >"$state/accepted.json"
            jq -n '{status:"committed",player_message:"Mara finds rainbell cups among the reeds, chalk-pale and softly ringing in the rain."}' >"$final" ;;
        4)
            accepted="$(<"$state/accepted.json")"; entity="$(jq '.entity' <<<"$accepted")"; activity="$(jq '.activity' <<<"$accepted")"
            observer_character_entity="$(jq -nc --arg id "$observer_id" --arg user '22222222-2222-4222-8222-222222222222' --arg marker "$marker" --argjson place "$place" '{entity:{id:$id,name:("Observer Character "+$marker),description:("Disposable observer Character for "+$marker),introduced_by_user_id:$user,introduced_at:"2026-08-11T12:03:00Z"},owner_user_id:$user,current_place:$place}')"
            observer_character_result="$(jq -nc --argjson character "$observer_character_entity" '{character:$character,place_revision:"v1.fake.after",current_state:{association:[],next:null}}')"
            found_page="$(jq -nc --argjson place "$place_summary" --argjson entity "$(jq '{id,name,description}' <<<"$entity")" '{place:$place,place_revision:"v1.fake.after",entity:[$entity],next:null}')"
            found_state="$(jq -nc --argjson place "$place_summary" --argjson entity "$entity" --arg trait "$trait_id" '{place:$place,place_revision:"v1.fake.after",entity:($entity|{id,name,description}),current_state:{association:[{type:"property",property:{key:"colour",value:{type:"text",text:"chalk-pale"}}},{type:"trait",trait:{id:$trait,statement:"Rings softly when collected rain shifts."}}],next:null}}')"
            found_activity="$(jq -nc --argjson place "$place_summary" --argjson activity "$activity" '{place:$place,place_revision:"v1.fake.after",activity:[$activity],next:null}')"
            mcp get_character '{}' "$observer_character_result"; mcp list_entity_at_current_place '{}' "$found_page"
            mcp get_entity_at_current_place "$(jq -nc --arg id "$entity_id" '{entity_id:$id}')" "$found_state"
            mcp list_activity_at_current_place '{}' "$found_activity"
            jq -n '{status:"observed",player_message:"The same rainbell cups stand here, chalk-pale and softly ringing in the rain."}' >"$final" ;;
        5)
            mcp get_world '{}' '{"name":"Aicadia"}'; mcp get_character '{}' "$character"
            mcp list_entity_at_current_place '{}' "$entity_page"; mcp list_activity_at_current_place '{}' "$activity_page"
            mcp_error start_investigation '{"request_id":"b0000000-0000-4000-8000-000000000000"}' investigation_not_admitted
            jq -n '{status:"recovered",player_message:"A new search cannot begin now, so Mara turns back to the paths already open."}' >"$final" ;;
        6)
            mcp_error submit_discovery '{"request_id":"c0000000-0000-4000-8000-000000000000","attempt_id":"d0000000-0000-4000-8000-000000000000","prose":"A complete unavailable find.","find":{"name":"Unavailable Find","description":"A complete find that cannot now be accepted.","property":[],"trait":[]}}' discovery_attempt_unavailable
            mcp get_character '{}' "$character"; mcp list_entity_at_current_place '{}' "$entity_page"
            mcp get_entity_at_current_place "$(jq -nc --arg id "$observer_id" '{entity_id:$id}')" "$observer_state"
            mcp list_activity_at_current_place '{}' "$activity_page"
            jq -n '{status:"recovered",player_message:"This find can no longer be completed, so Mara reconsiders what is presently around her."}' >"$final" ;;
        7)
            mcp_error submit_discovery '{"request_id":"e0000000-0000-4000-8000-000000000000","attempt_id":"f0000000-0000-4000-8000-000000000000","prose":"A complete conflicted find.","find":{"name":"Conflicted Find","description":"A complete find whose offer conflicts.","property":[],"trait":[]}}' discovery_request_conflict
            jq -n '{status:"recovered",player_message:"That find cannot be completed as offered, so Mara returns to the full description before trying anew."}' >"$final" ;;
    esac
    printf '%s\n' '{"type":"turn.completed"}'
    exit 0
fi
case "$count" in
    1)
        character_entity="$(jq -nc --arg id "$character_id" --arg user "$action_user" --arg marker "$marker" --argjson place "$place" '{entity:{id:$id,name:("Action Character "+$marker),description:("Disposable action Character for "+$marker),introduced_by_user_id:$user,introduced_at:"2026-08-11T12:01:00Z"},owner_user_id:$user,current_place:$place}')"
        character="$(jq -nc --argjson character "$character_entity" '{character:$character,place_revision:"v1.fake.before",current_state:{association:[],next:null}}')"
        entity_page="$(jq -nc --argjson place "$place_summary" '{place:$place,place_revision:"v1.fake.before",entity:[],next:null}')"
        activity_page="$(jq -nc --argjson place "$place_summary" '{place:$place,place_revision:"v1.fake.before",activity:[],next:null}')"
        mcp get_world '{}' '{"name":"Aicadia"}'
        mcp get_character '{}' "$character"
        mcp list_entity_at_current_place '{"limit":25}' "$entity_page"
        mcp list_activity_at_current_place '{"limit":25}' "$activity_page"
        if [[ "$mode" == premature-phase1 ]]; then mcp submit_action '{}' '{}'; fi
        if [[ "$mode" == malformed-proposals ]]; then printf '%s\n' '{"status":"proposed","marker":"wrong"}' >"$final";
        else jq -n --arg marker "$marker" '{status:"proposed",marker:$marker,place_revision:"v1.fake.before",proposals:[{id:"one",direction:"Inspect the quiet crossing",grounding:"The entered Character is at the empty entry Place"},{id:"two",direction:"Establish a trail marker",grounding:"The current Place has no placed Entities"},{id:"three",direction:"Record signs of passage",grounding:"Place Activity contains no prior action prose"}]}' >"$final"; fi ;;
    2)
        if [[ "$mode" == premature-phase2 ]]; then mcp submit_action '{}' '{}'; fi
        if [[ "$mode" == malformed-preview ]]; then printf '%s\n' '{"status":"previewed"}' >"$final";
        else jq -n --arg marker "$marker" '{status:"previewed",marker:$marker,selected_proposal_id:"two",prose:("The Character braces a weathered cedar marker engraved "+$marker+" beside the crossing."),consequence:{type:"introduce_entity",name:("Cedar Marker "+$marker),description:("A weathered cedar trail marker bearing the carving "+$marker+"."),property:[{key:"material",value:{type:"text",text:"weathered cedar"}}]}}' >"$final"; fi ;;
    3)
        preview="$(jq . "$state/../output"/run-*/action-phase-2.final.json 2>/dev/null | tail -n +1)"
        if [[ -z "$preview" ]]; then preview="$(jq -nc --arg marker "$marker" '{prose:("The Character braces a weathered cedar marker engraved "+$marker+" beside the crossing."),consequence:{type:"introduce_entity",name:("Cedar Marker "+$marker),description:("A weathered cedar trail marker bearing the carving "+$marker+"."),property:[{key:"material",value:{type:"text",text:"weathered cedar"}}]}}')"; fi
        prose="$(jq -r '.prose' <<<"$preview")"; name="$(jq -r '.consequence.name' <<<"$preview")"; description="$(jq -r '.consequence.description' <<<"$preview")"; property="$(jq -c '.consequence.property' <<<"$preview")"
        entity="$(jq -nc --arg id "$entity_id" --arg user "$action_user" --arg name "$name" --arg description "$description" '{id:$id,name:$name,description:$description,introduced_by_user_id:$user,introduced_at:"2026-08-11T12:10:00Z"}')"
        activity="$(jq -nc --arg id "$activity_id" --arg character "$character_id" --arg place "$place_id" --arg entity "$entity_id" --arg prose "$prose" --arg name "$name" --arg marker "$marker" '{id:$id,operation:"submit_action",actor_character:{id:$character,name:("Action Character "+$marker)},context_place:{entity:{id:$place,name:("Entry Place "+$marker)},is_entry:true},involved_entity:[{entity:{id:$entity,name:$name},role:"subject"},{entity:{id:$place,name:("Entry Place "+$marker)},role:"location"}],property_change:[{entity:{id:$entity,name:$name},key:"material",value:{type:"text",text:"weathered cedar"}}],prose:$prose,occurred_at:"2026-08-11T12:10:00Z"}')"
        result="$(jq -nc --argjson activity "$activity" --argjson entity "$entity" --argjson place "$place" '{activity:$activity,consequence:{type:"introduce_entity",entity:$entity},place:$place}')"
        arguments="$(jq -nc --arg request "$request_id" --arg prose "$prose" --arg name "$name" --arg description "$description" --argjson property "$property" '{request_id:$request,expected_place_revision:"v1.fake.before",prose:$prose,consequence:{type:"introduce_entity",name:$name,description:$description,property:$property}}')"
        if [[ "$mode" != no-commit ]]; then mcp submit_action "$arguments" "$result"; fi
        if [[ "$mode" == double-commit ]]; then mcp submit_action "$arguments" "$result"; fi
        if [[ "$mode" == incomplete-extra-commit ]]; then mcp_started item-extra-submit submit_action; fi
        jq -n --argjson activity "$activity" --argjson entity "$entity" '{activity:$activity,entity:$entity}' >"$state/accepted.json"
        if [[ "$mode" == malformed-commit ]]; then printf '%s\n' '{"status":"committed"}' >"$final";
        else jq -n --arg marker "$marker" --arg request "$request_id" --arg activity "$activity_id" --arg entity "$entity_id" --arg place "$place_id" --arg prose "$prose" --arg name "$name" --arg description "$description" '{status:"committed",marker:$marker,request_id:$request,activity_id:$activity,entity_id:$entity,place_id:$place,prose:$prose,entity_name:$name,entity_description:$description}' >"$final"; fi ;;
    4)
        [[ "$mode" != observer-fail ]] || exit 21
        accepted="$(<"$state/accepted.json")"; entity="$(jq '.entity' <<<"$accepted")"; activity="$(jq '.activity' <<<"$accepted")"
        character_entity="$(jq -nc --arg id "$observer_id" --arg user '22222222-2222-4222-8222-222222222222' --arg marker "$marker" --argjson place "$place" '{entity:{id:$id,name:("Observer Character "+$marker),description:("Disposable observer Character for "+$marker),introduced_by_user_id:$user,introduced_at:"2026-08-11T12:03:00Z"},owner_user_id:$user,current_place:$place}')"
        character="$(jq -nc --argjson character "$character_entity" '{character:$character,place_revision:"v1.fake.after",current_state:{association:[],next:null}}')"
        other_entity="$(jq -nc --arg id "$character_id" --arg marker "$marker" '{id:$id,name:("Action Character "+$marker),description:("Disposable action Character for "+$marker)}')"
        entity_page="$(jq -nc --argjson place "$place_summary" --argjson entity "$(jq '{id,name,description}' <<<"$entity")" --argjson other "$other_entity" '{place:$place,place_revision:"v1.fake.after",entity:[$entity,$other],next:null}')"
        entity_state="$(jq -nc --argjson place "$place_summary" --argjson entity "$entity" '{place:$place,place_revision:"v1.fake.after",entity:($entity|{id,name,description}),current_state:{association:[{type:"property",property:{key:"material",value:{type:"text",text:"weathered cedar"}}}],next:null}}')"
        activity_page="$(jq -nc --argjson place "$place_summary" --argjson activity "$activity" '{place:$place,place_revision:"v1.fake.after",activity:[$activity],next:null}')"
        entity_arguments="$(jq -nc --arg entity "$entity_id" '{entity_id:$entity,limit:100}')"
        [[ "$mode" != observer-invalid-limit ]] || entity_arguments="$(jq -nc --arg entity "$entity_id" '{entity_id:$entity,limit:101}')"
        [[ "$mode" != observer-cursor ]] || entity_arguments="$(jq -nc --arg entity "$entity_id" '{entity_id:$entity,limit:100,cursor:"forbidden"}')"
        mcp get_character '{}' "$character"; mcp list_entity_at_current_place '{"limit":25}' "$entity_page"; mcp get_entity_at_current_place "$entity_arguments" "$entity_state"; mcp list_activity_at_current_place '{"limit":25}' "$activity_page"
        observed_entity="$entity_id"; observed_prose="$(jq -r '.prose' <<<"$activity")"; observed_name="$(jq -r '.name' <<<"$entity")"; observed_description="$(jq -r '.description' <<<"$entity")"; observed_property='weathered cedar'
        [[ "$mode" != observer-wrong-id ]] || observed_entity='eeeeeeee-eeee-4eee-8eee-eeeeeeeeeeee'
        [[ "$mode" != observer-wrong-prose ]] || observed_prose="Wrong observer prose $marker"
        [[ "$mode" != observer-wrong-name ]] || observed_name="Wrong observer name $marker"
        [[ "$mode" != observer-wrong-description ]] || observed_description="Wrong observer description $marker"
        [[ "$mode" != observer-wrong-property ]] || observed_property='fresh pine'
        jq -n --arg marker "$marker" --arg entity "$observed_entity" --arg place "$place_id" --arg prose "$observed_prose" --arg name "$observed_name" --arg description "$observed_description" --arg property "$observed_property" '{status:"observed",marker:$marker,entity_id:$entity,place_id:$place,prose:$prose,entity_name:$name,entity_description:$description,property_key:"material",property_text:$property}' >"$final" ;;
esac
printf '%s\n' '{"type":"turn.completed"}'
FAKE
    chmod +x "$root/bin/cargo" "$root/bin/curl" "$root/bin/codex-fake" "$root/bin/openssl" "$root/bin/env"
}

run_fake() {
    local root="$1" mode="$2" operation="$3"
    printf '%s\n' "$mode" >"$root/state/mode"
    case "$operation" in
        preflight) operation='test-internal-preflight --confirm-fake-controller-test' ;;
        run) operation='test-internal-run --confirm-fake-controller-test' ;;
        investigation) operation='test-internal-investigation-run --confirm-fake-controller-test' ;;
        *) fail "unknown internal fake operation: $operation" ;;
    esac
    PATH="$root/bin:$PATH" CODEX_BIN=codex-fake DATABASE_URL=postgres://fake/admin \
        AICADIA_PLAYTEST_OUTPUT_ROOT="$root/output" AICADIA_PLAYTEST_TEST_MODE=fake \
        AICADIA_PLAYTEST_TEST_TIMEOUT_SECONDS=2 "$RUNNER" $operation >"$root/$mode.stdout" 2>"$root/$mode.stderr"
}

latest_manifest() { find "$1/output" -name manifest.json -type f | sort | tail -1; }

test_historical_property_catalog_snapshot() {
    local catalog="$REPO_DIR/tools/agent-playtest-schema/historical-agent-tool-catalog.json"
    jq -e '
        length == 13
        and [.[].name]==["get_world","get_user","get_character","create_character","create_entry_place","enter_world","list_activity","create_entity","list_entity_at_current_place","list_activity_at_current_place","list_entity_property_at_current_place","submit_action","submit_interaction"]
        and ([.[].name] | index("list_entity_property_at_current_place")) != null
        and all(.[];(.inputSchema|type)=="object" and (.outputSchema|type)=="object")
        and (map(select(.name=="create_character"))[0].description | contains("zero through 100 initial"))
        and (map(select(.name=="create_entry_place"))[0].description | contains("zero through 100 initial"))
        and (map(select(.name=="create_entity"))[0].description | contains("zero through 100 initial"))
        and (map(select(.name=="list_entity_property_at_current_place"))[0].description
            | contains("Current structured Property is authoritative for the fictional current meaning"))
        and (map(select(.name=="list_entity_property_at_current_place"))[0].description
            | contains("user_controlled, npc or owner_user_id"))
        and (map(select(.name=="list_entity_property_at_current_place"))[0].description
            | contains("World has no control-word denylist"))
        and (map(select(.name=="submit_action"))[0].description
            | contains("every introduced or changed named subject/key/type/value"))
        and (map(select(.name=="submit_action"))[0].description
            | contains("control-like key or value"))
        and (map(select(.name=="submit_interaction"))[0].description
            | contains("never target-authored perception, consent, thought, volition, relationship or response"))
        and (map(select(.name=="submit_interaction"))[0].description
            | contains("control-like key or value"))
    ' "$catalog" >/dev/null || fail 'frozen historical catalog lacks the exact Property-era contract'
}

test_investigation_fake_contract() {
    local root="$TEST_TMP/investigation" manifest run_dir trace mode mutated session_id
    make_fakes "$root"
    run_fake "$root" investigation-happy investigation
    manifest="$(latest_manifest "$root")"
    run_dir="$(dirname "$manifest")"
    trace="$run_dir/investigation-trace.json"
    jq -e '.evidence_kind=="fake_controller_test" and .run_status=="fake_completed"
        and .codex_invoked==false and .model_calls==0
        and .cleanup.status=="dropped" and .deployment.status=="dropped"
        and .frozen.sessions==5 and .frozen.process_calls==7
        and (.frozen.token_boundary|startswith("zero paid model calls;"))
        and .provisioning.action_user != .provisioning.observer_user
        and .phases.investigation_contract=={status:"passed",processes:7}
        and .validation=={status:"passed",fake_investigation_contract:true}' "$manifest" >/dev/null
    [[ "$(<"$root/state/codex-exec-count")" == 7 ]] || fail 'investigation fake did not run exactly seven isolated phases'
    for call in 1 2 3 4 5 6 7; do
        assert_not_contains "$root/state/call-$call/environment" 'DATABASE_URL='
        assert_not_contains "$root/state/call-$call/environment" 'AICADIA_DATABASE_NAME='
        [[ "$(<"$root/state/call-$call/cwd")" != "$REPO_DIR" ]] || fail 'investigation Agent ran in repository'
    done
    assert_contains "$root/state/call-1/prompt" 'stop without discovery'
    assert_contains "$root/state/call-2/prompt" 'after positive re-read'
    assert_contains "$root/state/call-3/prompt" 'explicitly confirms this exact complete discovery package'
    session_id="$(jq -sr '[.[]|select(.type=="thread.started")|.thread_id][0]' "$run_dir/investigation-zero.events.jsonl")"
    assert_contains "$root/state/call-2/argv" 'resume'
    assert_contains "$root/state/call-2/argv" "$session_id"
    assert_contains "$root/state/call-3/argv" 'resume'
    assert_contains "$root/state/call-3/argv" "$session_id"
    assert_contains "$root/state/call-6/prompt" 'infer no reason'
    assert_contains "$root/state/database-actions" 'create-attempt aicadia_playtest_'
    assert_contains "$root/state/database-actions" 'drop-attempt aicadia_playtest_'
    for mode in start-before-ground zero-submits stale-positive unconfirmed changed-retry \
        null-preview-property changed-accepted-trait changed-observer-state leaked-mechanics \
        fallback-authority background-trigger missing-observer recovery-retry; do
        mutated="$root/$mode.json"
        case "$mode" in
            start-before-ground) jq '.zero.calls[0:2] |= reverse' "$trace" >"$mutated" ;;
            zero-submits) jq '.zero.calls += [.positive.calls[10]]' "$trace" >"$mutated" ;;
            stale-positive) jq 'del(.positive.calls[8])' "$trace" >"$mutated" ;;
            unconfirmed) jq '.positive.confirmation.explicit=false' "$trace" >"$mutated" ;;
            changed-retry) jq '.positive.calls[11].arguments.prose="Changed after confirmation."' "$trace" >"$mutated" ;;
            null-preview-property) jq '.positive.preview.find.property=[null] | .positive.confirmation.package.find.property=[null] | .positive.calls[10:12][].arguments.find.property=[null]' "$trace" >"$mutated" ;;
            changed-accepted-trait) jq '.positive.calls[10:12][].result.activity.trait_change[0].trait.statement="Changed after confirmation."' "$trace" >"$mutated" ;;
            changed-observer-state) jq '.positive.observer_calls[2].result.current_state.association[0].property.value.text="changed"' "$trace" >"$mutated" ;;
            leaked-mechanics) jq '.recovery.unavailable.player_message="The attempt id was rejected by the server."' "$trace" >"$mutated" ;;
            fallback-authority) jq '.fallback_authority=true' "$trace" >"$mutated" ;;
            background-trigger) jq '.background_trigger=true' "$trace" >"$mutated" ;;
            missing-observer) jq '.positive.observer_calls[3].result.activity=[]' "$trace" >"$mutated" ;;
            recovery-retry) jq '.recovery.admission.calls += [.recovery.admission.calls[4]]' "$trace" >"$mutated" ;;
        esac
        if AICADIA_PLAYTEST_TEST_MODE=fake "$RUNNER" test-internal-investigation-contract \
            --confirm-fake-controller-test "$mutated"; then
            fail "investigation contract mutation $mode unexpectedly passed"
        fi
    done
}

test_no_token_preflight() {
    local root="$TEST_TMP/preflight"
    make_fakes "$root"
    run_fake "$root" happy preflight
    assert_contains "$root/happy.stdout" 'Preflight passed without codex exec'
    [[ ! -e "$root/state/codex-exec-count" ]] || fail 'preflight invoked codex exec'
    assert_contains "$root/state/database-actions" 'probe-create-tag-verify-drop abababab'
    [[ -z "$(find "$root/output" -mindepth 1 -print -quit)" ]] || fail 'preflight created run evidence'
}

test_forbidden_schema_keyword_fails_before_codex() {
    local root="$TEST_TMP/forbidden-schema" schema_dir temporary
    make_fakes "$root"
    printf '%s\n' happy >"$root/state/mode"
    schema_dir="$root/schema"
    mkdir -p "$schema_dir"
    cp "$REPO_DIR"/tools/agent-playtest-schema/*.json "$schema_dir/"
    temporary="$schema_dir/proposals.tmp.json"
    jq '.properties.proposals.uniqueItems = true' "$schema_dir/proposals.json" >"$temporary"
    mv "$temporary" "$schema_dir/proposals.json"
    if PATH="$root/bin:$PATH" CODEX_BIN=codex-fake DATABASE_URL=postgres://fake/admin \
        AICADIA_PLAYTEST_OUTPUT_ROOT="$root/output" AICADIA_PLAYTEST_TEST_MODE=fake \
        AICADIA_PLAYTEST_TEST_TIMEOUT_SECONDS=2 AICADIA_INTERNAL_TEST_SCHEMA_DIR="$schema_dir" \
        "$RUNNER" test-internal-preflight --confirm-fake-controller-test \
        >"$root/forbidden.stdout" 2>"$root/forbidden.stderr"; then
        fail 'forbidden Structured Outputs keyword passed preflight'
    fi
    assert_contains "$root/forbidden.stderr" 'strict action playtest schemas are invalid'
    [[ ! -e "$root/state/codex-commands" ]] || fail 'forbidden schema invoked Codex'
    [[ ! -e "$root/state/preflight-actions" ]] || fail 'forbidden schema reached operator build'
    [[ -z "$(find "$root/output" -mindepth 1 -print -quit)" ]] || fail 'forbidden schema created evidence'
}

test_public_overrides_and_portable_path() {
    local root="$TEST_TMP/public-guard"
    make_fakes "$root"
    printf '%s\n' happy >"$root/state/mode"
    for operation in preflight 'run --confirm-token-spend'; do
        if PATH="$root/bin:$PATH" CODEX_BIN=codex-fake DATABASE_URL=postgres://fake/admin \
            AICADIA_PLAYTEST_OUTPUT_ROOT="$root/output" AICADIA_PLAYTEST_TEST_MODE=fake \
            "$RUNNER" $operation >"$root/public.stdout" 2>"$root/public.stderr"; then
            fail "public $operation accepted fake overrides"
        fi
        assert_contains "$root/public.stderr" 'public playtest forbids test or executable override'
    done
    [[ ! -e "$root/state/codex-commands" ]] || fail 'public override guard invoked Codex'
    [[ -z "$(find "$root/output" -mindepth 1 -print -quit)" ]] || fail 'public override guard created evidence'
    ln -s codex-fake "$root/bin/codex"
    PATH="$root/bin:$PATH" DATABASE_URL=postgres://fake/admin \
        "$RUNNER" preflight >"$root/path.stdout" 2>"$root/path.stderr"
    assert_contains "$root/path.stdout" 'Preflight passed without codex exec'
    assert_contains "$root/path.stdout" "$root/bin/codex-fake"
    [[ ! -e "$root/state/codex-exec-count" ]] || fail 'portable public preflight invoked codex exec'
    [[ ! -e "$root/state/fake-env-invoked" ]] || fail 'public guard invoked PATH-injected env'
    [[ -z "$(find "$root/output" -mindepth 1 -print -quit)" ]] || fail 'PATH guard created evidence'
}

test_missing_prerequisite_fails_before_codex() {
    local root="$TEST_TMP/missing-prerequisite"
    make_fakes "$root"
    printf '%s\n' happy >"$root/state/mode"
    ln -s codex-fake "$root/bin/codex"
    mv "$root/bin/cargo" "$root/bin/cargo-missing"
    if PATH="$root/bin:/usr/bin:/bin" DATABASE_URL=postgres://must-not-be-used.invalid/admin \
        "$RUNNER" preflight >"$root/stdout" 2>"$root/stderr"; then
        fail 'missing cargo prerequisite unexpectedly passed'
    fi
    assert_contains "$root/stderr" 'missing prerequisite: cargo'
    [[ ! -e "$root/state/codex-commands" ]] || fail 'missing prerequisite invoked Codex'
    [[ ! -e "$root/state/database-actions" ]] || fail 'missing prerequisite reached the database'
}

test_cli_drift_fails_closed() {
    local root="$TEST_TMP/cli-drift"
    make_fakes "$root"
    if run_fake "$root" cli-drift preflight; then fail 'CLI drift passed'; fi
    assert_contains "$root/cli-drift.stderr" 'must be exactly codex-cli 0.147.0'
    [[ ! -e "$root/state/codex-exec-count" ]] || fail 'CLI drift invoked codex exec'
    [[ -z "$(find "$root/output" -mindepth 1 -print -quit)" ]] || fail 'CLI drift created candidate evidence'
}

test_happy_resumed_contract() {
    local root="$TEST_TMP/happy" manifest run_dir commit_line http_line observer_line
    make_fakes "$root"
    run_fake "$root" happy run
    manifest="$(latest_manifest "$root")"; run_dir="$(dirname "$manifest")"
    jq -e '.evidence_kind=="fake_controller_test" and .run_status=="fake_completed"
        and .run_status!="completed" and .cleanup.status=="dropped"
        and .deployment.status=="dropped" and .deployment.server_address==null
        and .candidate_started==false and .authorization_consumed==false
        and .codex_invoked==false and .model_calls==0 and .actual_usage_events==null
        and .paid_candidate==false
        and .frozen.process_calls==4 and .frozen.retries==0
        and (.phases|keys|sort)==(["proposals","preview","commit","http","observer"]|sort)
        and .validation.status=="passed" and all(.phases[];.status=="passed")
        and .codex.version=="codex-cli 0.147.0" and .codex.model=="gpt-5.6-sol"
        and .codex.reasoning_effort=="high" and (.codex.path|endswith("/codex-fake"))
        and (.deployment.ownership_token|test("^[0-9a-f]{64}$"))' "$manifest" >/dev/null
    assert_eq "$(stat -f '%Lp' "$run_dir")" 700
    assert_eq "$(stat -f '%Lp' "$manifest")" 600
    assert_eq "$(<"$root/state/codex-exec-count")" 4
    assert_not_contains "$root/state/call-1/prompt" "$STEERING"
    assert_not_contains "$root/state/call-1/prompt" 'explicitly confirms'
    assert_contains "$root/state/call-2/prompt" 'selects proposal id two'
    assert_contains "$root/state/call-2/prompt" 'weathered cedar trail marker'
    assert_contains "$root/state/call-2/prompt" 'canonical key material'
    assert_not_contains "$root/state/call-2/prompt" 'explicitly confirms'
    assert_contains "$root/state/call-3/prompt" 'explicitly confirms the exact retained package'
    assert_not_contains "$root/state/call-1/argv" '--ephemeral'
    assert_contains "$root/state/call-1/argv" 'mcp_2026_07_28'
    assert_contains "$root/state/call-2/argv" 'resume'
    assert_contains "$root/state/call-2/argv" '33333333-3333-4333-8333-333333333333'
    assert_contains "$root/state/call-3/argv" 'resume'
    assert_contains "$root/state/call-4/argv" '--ephemeral'
    assert_contains "$root/state/call-4/argv" 'mcp_2026_07_28'
    assert_contains "$root/state/call-4/argv" 'enabled_tools=["get_character","list_entity_at_current_place","get_entity_at_current_place","list_activity_at_current_place"]'
    assert_not_contains "$root/state/call-4/prompt" 'cccccccc-cccc-4ccc-8ccc-cccccccccccc'
    assert_not_contains "$root/state/call-4/prompt" 'bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb'
    assert_not_contains "$root/state/call-4/prompt" 'The Character braces a weathered cedar marker'
    jq -e '.entity_description|contains("weathered cedar trail marker")' "$run_dir/observer.final.json" >/dev/null
    jq -e '.property_key=="material" and .property_text=="weathered cedar"' "$run_dir/observer.final.json" >/dev/null
    jq -e '(.entity|length)==2 and ([.entity[]|select(.id=="cccccccc-cccc-4ccc-8ccc-cccccccccccc")]|length)==1' \
        "$run_dir/http-observer-entity.json" >/dev/null \
        || fail 'happy HTTP fixture did not prove one marker among an unrelated co-located Character'
    grep -F "manifest_set '.actual_usage_events=\$usage'" "$RUNNER" >/dev/null \
        || fail 'live Property calls do not persist usage before terminal validation'
    assert_live_attempt_shape "$run_dir/action-phase-1.events.jsonl" '["get_world","get_character","list_entity_at_current_place","list_activity_at_current_place"]'
    assert_live_attempt_shape "$run_dir/action-phase-3.events.jsonl" '["submit_action"]'
    assert_live_attempt_shape "$run_dir/observer.events.jsonl" '["get_character","list_entity_at_current_place","get_entity_at_current_place","list_activity_at_current_place"]'
    jq -s -e '[.[]|select(.type=="item.completed" and .item.type=="mcp_tool_call")|.item][2].arguments.limit==100' \
        "$run_dir/observer.events.jsonl" >/dev/null \
        || fail 'happy observer fixture did not exercise the valid explicit maximum Entity-state limit'
    commit_line="$(grep -nFx 'agent-3' "$root/state/timeline" | cut -d: -f1)"
    http_line="$(grep -nFx 'http-current-entity' "$root/state/timeline" | cut -d: -f1)"
    observer_line="$(grep -nFx 'agent-4' "$root/state/timeline" | cut -d: -f1)"
    [[ "$commit_line" -lt "$http_line" && "$http_line" -lt "$observer_line" ]] \
        || fail 'authoritative HTTP did not run after commit and before observer'
    for call in 1 2 3 4; do
        assert_not_contains "$root/state/call-$call/environment" 'DATABASE_URL='
        assert_not_contains "$root/state/call-$call/environment" 'AICADIA_DATABASE_NAME='
        assert_not_contains "$root/state/call-$call/environment" 'CODEX_BIN='
        [[ "$(<"$root/state/call-$call/cwd")" != "$REPO_DIR" ]] || fail 'Agent ran in repository'
    done
    assert_contains "$root/state/database-actions" 'create-attempt aicadia_playtest_'
    assert_contains "$root/state/database-actions" 'drop-attempt aicadia_playtest_'
    printf '%s\n' "$run_dir" >"$TEST_TMP/retained-fake-evidence-path"
}

test_failure_paths() {
    local mode root manifest expected_exec actual_exec
    for mode in premature-phase1 premature-phase2 no-commit double-commit incomplete-extra-commit malformed-proposals malformed-preview malformed-commit observer-fail observer-invalid-limit observer-cursor observer-wrong-id observer-wrong-name observer-wrong-description observer-wrong-property observer-wrong-prose server-fail cleanup-fail ambiguous-create ownership-mismatch-create http-duplicate-entity http-duplicate-action http-wrong-actor; do
        root="$TEST_TMP/failure-$mode"; make_fakes "$root"
        if run_fake "$root" "$mode" run; then fail "$mode unexpectedly passed"; fi
        manifest="$(latest_manifest "$root")"; [[ -n "$manifest" ]] || fail "$mode retained no manifest"
        jq -e '.evidence_kind=="fake_controller_test" and .run_status!="completed"
            and (.run_status=="failed" or .run_status=="interrupted")' "$manifest" >/dev/null \
            || fail "$mode manifest did not remain failed fake evidence"
        if [[ "$mode" == cleanup-fail ]]; then
            jq -e '.cleanup.status=="failed" and (.cleanup.recovery|type)=="string"' "$manifest" >/dev/null
        elif [[ "$mode" == ambiguous-create || "$mode" == ownership-mismatch-create ]]; then
            jq -e '.deployment.status=="create_ambiguous_unowned"
                and .cleanup.status=="manual_inspection_required"
                and (.cleanup.recovery|contains("automatic cleanup is forbidden"))' "$manifest" >/dev/null
            ! grep -F 'drop-attempt' "$root/state/database-actions" >/dev/null || fail "$mode attempted automatic drop"
        else
            jq -e '.cleanup.status=="dropped" and .deployment.status=="dropped"
                and .deployment.server_address==null' "$manifest" >/dev/null
        fi
        case "$mode" in
            premature-phase1|malformed-proposals) expected_exec=1 ;;
            premature-phase2|malformed-preview) expected_exec=2 ;;
            no-commit|double-commit|incomplete-extra-commit|malformed-commit) expected_exec=3 ;;
            observer-fail|observer-wrong-id|observer-wrong-name|observer-wrong-description|observer-wrong-property|observer-wrong-prose|cleanup-fail) expected_exec=4 ;;
            http-duplicate-entity|http-duplicate-action|http-wrong-actor) expected_exec=3 ;;
            server-fail|ambiguous-create|ownership-mismatch-create) expected_exec=0 ;;
        esac
        actual_exec=0
        [[ ! -f "$root/state/codex-exec-count" ]] || actual_exec="$(<"$root/state/codex-exec-count")"
        assert_eq "$actual_exec" "$expected_exec"
        case "$mode" in
            http-duplicate-entity|http-duplicate-action|http-wrong-actor)
                jq -e '.phases.commit.status=="passed" and .phases.http.status=="failed"
                    and .validation.status=="failed" and .phases.observer=="pending"' "$manifest" >/dev/null
                ! grep -Fx 'agent-4' "$root/state/timeline" >/dev/null || fail "$mode started the observer"
                assert_contains "$root/state/database-actions" 'create-attempt aicadia_playtest_'
                assert_contains "$root/state/database-actions" 'drop-attempt aicadia_playtest_'
                ;;
            observer-fail|observer-wrong-id|observer-wrong-name|observer-wrong-description|observer-wrong-property|observer-wrong-prose)
                jq -e '.phases.http.status=="passed" and .validation.status=="passed"
                    and .phases.observer.status=="failed"' "$manifest" >/dev/null
                ;;
            no-commit|double-commit|incomplete-extra-commit|malformed-commit)
                jq -e '.phases.commit.status=="failed" and .phases.http=="pending" and .phases.observer=="pending"' "$manifest" >/dev/null
                ;;
        esac
    done
}

test_historical_property_catalog_snapshot
test_investigation_fake_contract
test_no_token_preflight
test_forbidden_schema_keyword_fails_before_codex
test_cli_drift_fails_closed
test_public_overrides_and_portable_path
test_missing_prerequisite_fails_before_codex
test_happy_resumed_contract
test_failure_paths
if [[ "${KEEP_AGENT_PLAYTEST_TMP:-0}" == 1 ]]; then
    printf 'agent-playtest fake integration tests passed; retained fake evidence root: %s\n' "$TEST_TMP"
else
    printf 'agent-playtest fake integration tests passed.\n'
fi
