use std::{
    borrow::Cow,
    collections::HashSet,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

use anyhow::{Context, Result, anyhow, ensure};
use axum::{
    Json, Router,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::Html,
    routing::{get, post},
};
use rmcp::{
    ClientLifecycleMode, ClientServiceExt, ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Json as McpJson},
    model::{
        CallToolRequestParams, CallToolResult, ClientInfo, Implementation, JsonObject,
        ProtocolVersion, ServerCapabilities, ServerInfo,
    },
    service::{Peer, RoleClient},
    tool, tool_handler, tool_router,
    transport::{
        StreamableHttpClientTransport,
        streamable_http_client::StreamableHttpClientTransportConfig,
        streamable_http_server::{
            StreamableHttpServerConfig, StreamableHttpService, session::local::LocalSessionManager,
        },
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use tokio::sync::{Mutex, Semaphore, watch};
use tokio_util::sync::CancellationToken;

pub const SUBJECT: &str = "lab://entity/table";
pub const INITIAL_DESCRIPTION: &str = "brown wooden table";
pub const UPDATED_DESCRIPTION: &str = "blue wooden table";
pub const MAX_WATCHES: usize = 8;
pub const MAX_WAIT_MS: u64 = 30_000;
pub const SERVER_INSTRUCTIONS: &str = include_str!("../fixture/server-instructions.txt");
const COMPONENT_HTML: &str = include_str!("../component/index.html");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct SubjectState {
    pub subject: String,
    pub version: u64,
    pub description: String,
}

impl SubjectState {
    fn initial() -> Self {
        Self {
            subject: SUBJECT.to_owned(),
            version: 1,
            description: INITIAL_DESCRIPTION.to_owned(),
        }
    }

    fn updated() -> Self {
        Self {
            subject: SUBJECT.to_owned(),
            version: 2,
            description: UPDATED_DESCRIPTION.to_owned(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct KnownSubject {
    pub subject: String,
    pub known_version: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct WaitInput {
    pub watches: Vec<KnownSubject>,
    pub max_wait_ms: u64,
}

impl WaitInput {
    pub fn table(version: u64, max_wait_ms: u64) -> Self {
        Self {
            watches: vec![KnownSubject {
                subject: SUBJECT.to_owned(),
                known_version: version,
            }],
            max_wait_ms,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct WaitResult {
    pub changed_subjects: Vec<String>,
    pub timed_out: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Observation {
    pub sequence: u64,
    pub event: String,
    pub detail: String,
}

#[derive(Debug)]
struct Inner {
    current: SubjectState,
    advanced: bool,
    next_sequence: u64,
    observations: Vec<Observation>,
}

#[derive(Clone, Debug)]
pub struct RaceGate {
    registered: Arc<Semaphore>,
    proceed: Arc<Semaphore>,
}

impl Default for RaceGate {
    fn default() -> Self {
        Self::new()
    }
}

impl RaceGate {
    pub fn new() -> Self {
        Self {
            registered: Arc::new(Semaphore::new(0)),
            proceed: Arc::new(Semaphore::new(0)),
        }
    }

    pub async fn wait_until_registered(&self) -> Result<()> {
        self.registered
            .acquire()
            .await
            .map_err(|_| anyhow!("race gate registration closed"))?
            .forget();
        Ok(())
    }

    pub fn release_recheck(&self) {
        self.proceed.add_permits(1);
    }

    async fn registered_then_wait(&self) -> Result<()> {
        self.registered.add_permits(1);
        self.proceed
            .acquire()
            .await
            .map_err(|_| anyhow!("race gate recheck closed"))?
            .forget();
        Ok(())
    }
}

struct ActiveWaitGuard(Arc<AtomicUsize>);

impl Drop for ActiveWaitGuard {
    fn drop(&mut self) {
        self.0.fetch_sub(1, Ordering::SeqCst);
    }
}

#[derive(Clone, Debug)]
pub struct LabState {
    inner: Arc<Mutex<Inner>>,
    changed_version: watch::Sender<u64>,
    active_waiters: Arc<AtomicUsize>,
}

impl Default for LabState {
    fn default() -> Self {
        Self::new()
    }
}

impl LabState {
    pub fn new() -> Self {
        let (changed_version, _receiver) = watch::channel(1);
        Self {
            inner: Arc::new(Mutex::new(Inner {
                current: SubjectState::initial(),
                advanced: false,
                next_sequence: 1,
                observations: Vec::new(),
            })),
            changed_version,
            active_waiters: Arc::new(AtomicUsize::new(0)),
        }
    }

    async fn record(&self, event: &str, detail: impl Into<String>) {
        let mut inner = self.inner.lock().await;
        let sequence = inner.next_sequence;
        inner.next_sequence += 1;
        inner.observations.push(Observation {
            sequence,
            event: event.to_owned(),
            detail: detail.into(),
        });
    }

    pub async fn current(&self) -> SubjectState {
        self.inner.lock().await.current.clone()
    }

    pub async fn read_for(&self, adapter: &str) -> SubjectState {
        let current = self.current().await;
        self.record(
            "state_read",
            format!(
                "adapter={adapter} subject={} version={}",
                current.subject, current.version
            ),
        )
        .await;
        current
    }

    pub async fn observations(&self) -> Vec<Observation> {
        self.inner.lock().await.observations.clone()
    }

    pub fn active_waiters(&self) -> usize {
        self.active_waiters.load(Ordering::SeqCst)
    }

    pub async fn advance_once(&self) -> Result<SubjectState> {
        let current = {
            let mut inner = self.inner.lock().await;
            ensure!(!inner.advanced, "fixture may advance exactly once");
            inner.advanced = true;
            inner.current = SubjectState::updated();
            inner.current.clone()
        };
        self.record(
            "state_advanced",
            format!("subject={SUBJECT} version=2 description={UPDATED_DESCRIPTION}"),
        )
        .await;
        self.changed_version.send_replace(2);
        Ok(current)
    }

    fn validate_wait(input: &WaitInput) -> Result<()> {
        ensure!(
            !input.watches.is_empty(),
            "at least one subject is required"
        );
        ensure!(
            input.watches.len() <= MAX_WATCHES,
            "at most {MAX_WATCHES} subjects may be watched"
        );
        ensure!(
            (1..=MAX_WAIT_MS).contains(&input.max_wait_ms),
            "max_wait_ms must be between 1 and {MAX_WAIT_MS}"
        );
        let mut unique = HashSet::with_capacity(input.watches.len());
        for watch in &input.watches {
            ensure!(watch.known_version > 0, "known_version must be positive");
            ensure!(unique.insert(&watch.subject), "subjects must be unique");
            ensure!(watch.subject == SUBJECT, "unknown lab subject");
        }
        Ok(())
    }

    async fn differing_subjects(&self, input: &WaitInput) -> Vec<String> {
        let current = self.current().await;
        input
            .watches
            .iter()
            .filter(|watch| {
                watch.subject == current.subject && watch.known_version != current.version
            })
            .map(|watch| watch.subject.clone())
            .collect()
    }

    pub async fn wait_for_change(&self, adapter: &str, input: WaitInput) -> Result<WaitResult> {
        self.wait_internal(adapter, input, None, true).await
    }

    pub async fn wait_for_change_at_registration(
        &self,
        adapter: &str,
        input: WaitInput,
        gate: RaceGate,
    ) -> Result<WaitResult> {
        self.wait_internal(adapter, input, Some(gate), true).await
    }

    async fn wait_unrecorded(&self, input: WaitInput) -> Result<WaitResult> {
        self.wait_internal("load", input, None, false).await
    }

    async fn wait_internal(
        &self,
        adapter: &str,
        input: WaitInput,
        gate: Option<RaceGate>,
        record: bool,
    ) -> Result<WaitResult> {
        Self::validate_wait(&input)?;

        let mut changes = self.changed_version.subscribe();
        self.active_waiters.fetch_add(1, Ordering::SeqCst);
        let _active = ActiveWaitGuard(self.active_waiters.clone());
        if record {
            self.record(
                "wait_registered",
                format!(
                    "adapter={adapter} subject={} known_version={} max_wait_ms={}",
                    input.watches[0].subject, input.watches[0].known_version, input.max_wait_ms
                ),
            )
            .await;
        }

        if let Some(gate) = gate {
            gate.registered_then_wait().await?;
        }

        let already_changed = self.differing_subjects(&input).await;
        if !already_changed.is_empty() {
            let result = WaitResult {
                changed_subjects: already_changed,
                timed_out: false,
            };
            if record {
                self.record(
                    "wait_returned",
                    format!("adapter={adapter} result=changed source=current_version"),
                )
                .await;
            }
            return Ok(result);
        }

        let deadline = tokio::time::Instant::now() + Duration::from_millis(input.max_wait_ms);
        loop {
            match tokio::time::timeout_at(deadline, changes.changed()).await {
                Ok(Ok(())) => {
                    changes.borrow_and_update();
                    let changed_subjects = self.differing_subjects(&input).await;
                    if changed_subjects.is_empty() {
                        continue;
                    }
                    let result = WaitResult {
                        changed_subjects,
                        timed_out: false,
                    };
                    if record {
                        self.record(
                            "wait_returned",
                            format!("adapter={adapter} result=changed source=hint"),
                        )
                        .await;
                    }
                    return Ok(result);
                }
                Ok(Err(_)) => return Err(anyhow!("change notifier closed")),
                Err(_) => {
                    let result = WaitResult {
                        changed_subjects: Vec::new(),
                        timed_out: true,
                    };
                    if record {
                        self.record("wait_returned", format!("adapter={adapter} result=timeout"))
                            .await;
                    }
                    return Ok(result);
                }
            }
        }
    }
}

fn input_schema<T: JsonSchema + 'static>() -> Arc<JsonObject> {
    rmcp::handler::server::common::schema_for_input::<T>()
        .unwrap_or_else(|error| panic!("invalid fixed lab input schema: {error}"))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct SubjectInput {
    pub subject: String,
}

#[derive(Clone)]
pub struct LabMcp {
    state: LabState,
    tool_router: ToolRouter<Self>,
}

impl LabMcp {
    pub fn new(state: LabState) -> Self {
        Self {
            state,
            tool_router: Self::tool_router(),
        }
    }

    fn decode<T: DeserializeOwned>(input: JsonObject) -> std::result::Result<T, CallToolResult> {
        serde_json::from_value(serde_json::Value::Object(input)).map_err(|error| {
            CallToolResult::error(vec![rmcp::model::ContentBlock::text(format!(
                "invalid lab input: {error}"
            ))])
        })
    }

    fn error(error: anyhow::Error) -> CallToolResult {
        CallToolResult::error(vec![rmcp::model::ContentBlock::text(error.to_string())])
    }
}

#[tool_router]
impl LabMcp {
    #[tool(
        description = "Read the complete current state of the one bounded lab subject. Use the exact returned subject and version when waiting for a change.",
        input_schema = input_schema::<SubjectInput>(),
        annotations(
            title = "Get current lab state",
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    async fn get_state(
        &self,
        input: JsonObject,
    ) -> std::result::Result<McpJson<SubjectState>, CallToolResult> {
        let input: SubjectInput = Self::decode(input)?;
        if input.subject != SUBJECT {
            return Err(Self::error(anyhow!("unknown lab subject")));
        }
        Ok(McpJson(self.state.read_for("mcp").await))
    }

    #[tool(
        description = "Wait once, for at most thirty seconds, until an exact lab subject differs from the supplied known version. The result contains no new state. If it reports a changed subject, call get_state again before answering. Never poll.",
        input_schema = input_schema::<WaitInput>(),
        annotations(
            title = "Wait for one lab change",
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    async fn wait_for_change(
        &self,
        input: JsonObject,
    ) -> std::result::Result<McpJson<WaitResult>, CallToolResult> {
        let input: WaitInput = Self::decode(input)?;
        self.state
            .wait_for_change("mcp", input)
            .await
            .map(McpJson)
            .map_err(Self::error)
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for LabMcp {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_protocol_version(ProtocolVersion::V_2026_07_28)
            .with_server_info(Implementation::new(
                "aicadia-host-independent-change-wait-lab",
                "0.1.0",
            ))
            .with_instructions(SERVER_INSTRUCTIONS)
    }

    fn supported_protocol_versions(&self) -> Cow<'static, [ProtocolVersion]> {
        Cow::Borrowed(&[ProtocolVersion::V_2026_07_28])
    }
}

#[derive(Clone)]
struct AppState {
    lab: LabState,
    controller_token: Arc<str>,
}

fn adapter(headers: &HeaderMap) -> &str {
    headers
        .get("x-lab-client")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("http")
}

async fn component() -> Html<&'static str> {
    Html(COMPONENT_HTML)
}

async fn http_state(State(state): State<AppState>, headers: HeaderMap) -> Json<SubjectState> {
    Json(state.lab.read_for(adapter(&headers)).await)
}

async fn http_wait(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(input): Json<WaitInput>,
) -> std::result::Result<Json<WaitResult>, (StatusCode, Json<serde_json::Value>)> {
    state
        .lab
        .wait_for_change(adapter(&headers), input)
        .await
        .map(Json)
        .map_err(|error| {
            (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": error.to_string()})),
            )
        })
}

fn controller_authorized(headers: &HeaderMap, expected: &str) -> bool {
    headers
        .get("x-lab-control-token")
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value == expected)
}

async fn controller_state(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> std::result::Result<Json<SubjectState>, StatusCode> {
    if !controller_authorized(&headers, &state.controller_token) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(Json(state.lab.current().await))
}

async fn controller_observations(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> std::result::Result<Json<Vec<Observation>>, StatusCode> {
    if !controller_authorized(&headers, &state.controller_token) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(Json(state.lab.observations().await))
}

async fn controller_advance(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> std::result::Result<Json<SubjectState>, (StatusCode, Json<serde_json::Value>)> {
    if !controller_authorized(&headers, &state.controller_token) {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error": "unauthorized"})),
        ));
    }
    state.lab.advance_once().await.map(Json).map_err(|error| {
        (
            StatusCode::CONFLICT,
            Json(serde_json::json!({"error": error.to_string()})),
        )
    })
}

fn router(state: LabState, controller_token: Arc<str>, cancellation: CancellationToken) -> Router {
    let mcp = LabMcp::new(state.clone());
    let service: StreamableHttpService<LabMcp, LocalSessionManager> = StreamableHttpService::new(
        move || Ok(mcp.clone()),
        Default::default(),
        StreamableHttpServerConfig::default()
            .with_json_response(true)
            .with_cancellation_token(cancellation.child_token()),
    );
    let app_state = AppState {
        lab: state,
        controller_token,
    };
    Router::new()
        .route("/", get(component))
        .route("/state", get(http_state))
        .route("/wait", post(http_wait))
        .route("/control/state", get(controller_state))
        .route("/control/observations", get(controller_observations))
        .route("/control/advance", post(controller_advance))
        .with_state(app_state)
        .nest_service("/mcp", service)
}

pub struct RunningServer {
    pub base_url: String,
    pub mcp_url: String,
    pub control_url: String,
    pub controller_token: String,
    pub state: LabState,
    cancellation: CancellationToken,
    task: tokio::task::JoinHandle<()>,
}

impl RunningServer {
    pub async fn stop(self) -> Result<()> {
        self.cancellation.cancel();
        tokio::time::timeout(Duration::from_secs(5), self.task)
            .await
            .context("server did not stop within five seconds")?
            .context("server task panicked")?;
        Ok(())
    }
}

pub async fn spawn_server() -> Result<RunningServer> {
    let state = LabState::new();
    let controller_token = "0123456789abcdef0123456789abcdef".to_owned();
    let cancellation = CancellationToken::new();
    let app = router(
        state.clone(),
        Arc::from(controller_token.clone()),
        cancellation.clone(),
    );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .context("bind lab server")?;
    let address = listener.local_addr().context("read lab server address")?;
    let task_cancellation = cancellation.clone();
    let task = tokio::spawn(async move {
        let _ = axum::serve(listener, app)
            .with_graceful_shutdown(async move { task_cancellation.cancelled_owned().await })
            .await;
    });
    let base_url = format!("http://{address}");
    Ok(RunningServer {
        mcp_url: format!("{base_url}/mcp"),
        control_url: format!("{base_url}/control"),
        base_url,
        controller_token,
        state,
        cancellation,
        task,
    })
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalServerReady {
    pub base_url: String,
    pub mcp_url: String,
    pub control_url: String,
}

pub async fn serve_external(controller_token: String) -> Result<ExternalServerReady> {
    ensure!(
        controller_token.len() >= 32,
        "controller token is too short"
    );
    let state = LabState::new();
    let cancellation = CancellationToken::new();
    let app = router(state, Arc::from(controller_token), cancellation.clone());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .context("bind external lab server")?;
    let address = listener.local_addr().context("read external lab address")?;
    let base_url = format!("http://{address}");
    tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });
    Ok(ExternalServerReady {
        mcp_url: format!("{base_url}/mcp"),
        control_url: format!("{base_url}/control"),
        base_url,
    })
}

async fn connect_mcp(url: &str) -> Result<rmcp::service::RunningService<RoleClient, ClientInfo>> {
    let transport = StreamableHttpClientTransport::from_config(
        StreamableHttpClientTransportConfig::with_uri(url.to_owned()),
    );
    ClientInfo::default()
        .serve_with_lifecycle(
            transport,
            ClientLifecycleMode::Discover {
                preferred_versions: vec![ProtocolVersion::V_2026_07_28],
            },
        )
        .await
        .map_err(Into::into)
}

fn arguments<T: Serialize>(value: &T) -> Result<JsonObject> {
    let serde_json::Value::Object(object) = serde_json::to_value(value)? else {
        return Err(anyhow!("lab arguments must encode as an object"));
    };
    Ok(object)
}

fn parse_tool_result<T: DeserializeOwned>(result: CallToolResult) -> Result<T> {
    ensure!(result.is_error != Some(true), "MCP tool returned an error");
    let value = result
        .structured_content
        .context("MCP tool omitted structured content")?;
    serde_json::from_value(value).context("decode MCP structured result")
}

async fn mcp_get_state(peer: &Peer<RoleClient>) -> Result<SubjectState> {
    let result = peer
        .call_tool(
            CallToolRequestParams::new("get_state").with_arguments(arguments(&SubjectInput {
                subject: SUBJECT.to_owned(),
            })?),
        )
        .await?;
    parse_tool_result(result)
}

async fn mcp_wait(peer: &Peer<RoleClient>, input: WaitInput) -> Result<WaitResult> {
    let result = peer
        .call_tool(CallToolRequestParams::new("wait_for_change").with_arguments(arguments(&input)?))
        .await?;
    parse_tool_result(result)
}

async fn wait_for_active(state: &LabState, expected: usize) -> Result<()> {
    tokio::time::timeout(Duration::from_secs(2), async {
        while state.active_waiters() < expected {
            tokio::task::yield_now().await;
        }
    })
    .await
    .context("waiters did not register")?;
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdapterScenario {
    pub adapter: String,
    pub initial: SubjectState,
    pub wait_result: WaitResult,
    pub final_state: SubjectState,
    pub observations: Vec<Observation>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreflightReport {
    pub protocol_version: String,
    pub http: AdapterScenario,
    pub mcp: AdapterScenario,
    pub hot_waiters: usize,
    pub agent_processes_launched: u8,
}

pub async fn http_scenario() -> Result<AdapterScenario> {
    let server = spawn_server().await?;
    let client = reqwest::Client::new();
    let initial: SubjectState = client
        .get(format!("{}/state", server.base_url))
        .header("x-lab-client", "terminal")
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    ensure!(initial.version == 1 && initial.description == INITIAL_DESCRIPTION);

    let wait_client = client.clone();
    let wait_url = format!("{}/wait", server.base_url);
    let wait = tokio::spawn(async move {
        wait_client
            .post(wait_url)
            .header("x-lab-client", "terminal")
            .json(&WaitInput::table(1, 2_000))
            .send()
            .await?
            .error_for_status()?
            .json::<WaitResult>()
            .await
            .map_err(anyhow::Error::from)
    });
    wait_for_active(&server.state, 1).await?;
    server.state.advance_once().await?;
    let wait_result = wait.await.context("HTTP wait task panicked")??;
    ensure!(
        wait_result
            == WaitResult {
                changed_subjects: vec![SUBJECT.to_owned()],
                timed_out: false,
            }
    );
    let final_state: SubjectState = client
        .get(format!("{}/state", server.base_url))
        .header("x-lab-client", "terminal")
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    ensure!(final_state.version == 2 && final_state.description == UPDATED_DESCRIPTION);
    let observations = server.state.observations().await;
    server.stop().await?;
    Ok(AdapterScenario {
        adapter: "terminal_http".to_owned(),
        initial,
        wait_result,
        final_state,
        observations,
    })
}

pub async fn mcp_scenario() -> Result<AdapterScenario> {
    let server = spawn_server().await?;
    let client = connect_mcp(&server.mcp_url).await?;
    let initial = mcp_get_state(client.peer()).await?;
    ensure!(initial.version == 1 && initial.description == INITIAL_DESCRIPTION);

    let wait_peer = client.peer().clone();
    let wait = tokio::spawn(async move { mcp_wait(&wait_peer, WaitInput::table(1, 2_000)).await });
    wait_for_active(&server.state, 1).await?;
    server.state.advance_once().await?;
    let wait_result = wait.await.context("MCP wait task panicked")??;
    ensure!(
        wait_result
            == WaitResult {
                changed_subjects: vec![SUBJECT.to_owned()],
                timed_out: false,
            }
    );
    let final_state = mcp_get_state(client.peer()).await?;
    ensure!(final_state.version == 2 && final_state.description == UPDATED_DESCRIPTION);
    client.cancel().await?;
    let observations = server.state.observations().await;
    server.stop().await?;
    Ok(AdapterScenario {
        adapter: "official_mcp_client".to_owned(),
        initial,
        wait_result,
        final_state,
        observations,
    })
}

pub async fn hot_waiter_scenario(waiters: usize) -> Result<()> {
    let state = LabState::new();
    let mut tasks = Vec::with_capacity(waiters);
    for _ in 0..waiters {
        let state = state.clone();
        tasks.push(tokio::spawn(async move {
            state.wait_unrecorded(WaitInput::table(1, 2_000)).await
        }));
    }
    wait_for_active(&state, waiters).await?;
    state.advance_once().await?;
    for task in tasks {
        let result = task.await.context("hot waiter task panicked")??;
        ensure!(!result.timed_out && result.changed_subjects == [SUBJECT]);
    }
    ensure!(state.active_waiters() == 0, "hot waiters did not clean up");
    ensure!(
        state
            .observations()
            .await
            .iter()
            .filter(|item| item.event == "state_advanced")
            .count()
            == 1,
        "hot scenario changed state more than once"
    );
    Ok(())
}

pub async fn run_preflight() -> Result<PreflightReport> {
    let http = http_scenario().await?;
    let mcp = mcp_scenario().await?;
    hot_waiter_scenario(10_000).await?;
    Ok(PreflightReport {
        protocol_version: "2026-07-28".to_owned(),
        http,
        mcp,
        hot_waiters: 10_000,
        agent_processes_launched: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn change_before_wait_returns_immediately_from_current_version() {
        let state = LabState::new();
        state.advance_once().await.expect("advance");
        let result = state
            .wait_for_change("test", WaitInput::table(1, 1_000))
            .await
            .expect("wait");
        assert_eq!(result.changed_subjects, [SUBJECT]);
        assert!(!result.timed_out);
    }

    #[tokio::test]
    async fn change_after_registration_wakes_once() {
        let state = LabState::new();
        let waiting = state.clone();
        let task = tokio::spawn(async move {
            waiting
                .wait_for_change("test", WaitInput::table(1, 1_000))
                .await
        });
        wait_for_active(&state, 1).await.expect("registered");
        state.advance_once().await.expect("advance");
        let result = task.await.expect("task").expect("wait");
        assert_eq!(result.changed_subjects, [SUBJECT]);
        assert_eq!(state.active_waiters(), 0);
    }

    #[tokio::test]
    async fn commit_during_registration_is_not_missed() {
        let state = LabState::new();
        let gate = RaceGate::new();
        let waiting = state.clone();
        let task_gate = gate.clone();
        let task = tokio::spawn(async move {
            waiting
                .wait_for_change_at_registration("test", WaitInput::table(1, 1_000), task_gate)
                .await
        });
        gate.wait_until_registered().await.expect("registered");
        state.advance_once().await.expect("advance in race");
        gate.release_recheck();
        let result = task.await.expect("task").expect("wait");
        assert_eq!(result.changed_subjects, [SUBJECT]);
        assert!(!result.timed_out);
    }

    #[tokio::test]
    async fn timeout_is_bounded_and_carries_no_state() {
        let state = LabState::new();
        let result = state
            .wait_for_change("test", WaitInput::table(1, 10))
            .await
            .expect("wait");
        assert!(result.timed_out);
        assert!(result.changed_subjects.is_empty());
        assert_eq!(state.current().await, SubjectState::initial());
    }

    #[tokio::test]
    async fn disconnect_then_old_version_recovers_without_replay() {
        let state = LabState::new();
        let waiting = state.clone();
        let task = tokio::spawn(async move {
            waiting
                .wait_for_change("disconnected", WaitInput::table(1, 5_000))
                .await
        });
        wait_for_active(&state, 1).await.expect("registered");
        task.abort();
        let _ = task.await;
        assert_eq!(state.active_waiters(), 0);
        state
            .advance_once()
            .await
            .expect("advance while disconnected");
        let result = state
            .wait_for_change("reconnected", WaitInput::table(1, 1_000))
            .await
            .expect("reconnect wait");
        assert_eq!(result.changed_subjects, [SUBJECT]);
    }

    #[tokio::test]
    async fn latest_version_coalesces_repeated_hints() {
        let state = LabState::new();
        state.advance_once().await.expect("advance");
        state.changed_version.send_replace(2);
        state.changed_version.send_replace(2);
        let result = state
            .wait_for_change("test", WaitInput::table(1, 1_000))
            .await
            .expect("wait");
        assert_eq!(result.changed_subjects, [SUBJECT]);
        assert_eq!(state.current().await.version, 2);
    }

    #[tokio::test]
    async fn invalid_bounds_fail_before_registration() {
        let state = LabState::new();
        let too_many = WaitInput {
            watches: (0..=MAX_WATCHES)
                .map(|index| KnownSubject {
                    subject: format!("lab://subject/{index}"),
                    known_version: 1,
                })
                .collect(),
            max_wait_ms: 1,
        };
        assert!(state.wait_for_change("test", too_many).await.is_err());
        assert!(
            state
                .wait_for_change("test", WaitInput::table(1, MAX_WAIT_MS + 1))
                .await
                .is_err()
        );
        assert_eq!(state.active_waiters(), 0);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn ten_thousand_hot_waiters_share_one_change() {
        hot_waiter_scenario(10_000)
            .await
            .expect("hot waiter scenario");
    }

    #[tokio::test]
    async fn terminal_http_adapter_waits_then_rereads() {
        let report = http_scenario().await.expect("HTTP scenario");
        assert_eq!(report.initial.version, 1);
        assert_eq!(report.final_state.version, 2);
        assert!(
            report
                .observations
                .iter()
                .any(|item| item.event == "wait_registered")
        );
    }

    #[tokio::test]
    async fn official_mcp_adapter_waits_then_rereads() {
        let report = mcp_scenario().await.expect("MCP scenario");
        assert_eq!(report.initial.version, 1);
        assert_eq!(report.final_state.version, 2);
        assert_eq!(report.wait_result.changed_subjects, [SUBJECT]);
    }

    #[tokio::test]
    async fn controller_is_secret_guarded_and_one_way() {
        let server = spawn_server().await.expect("server");
        let client = reqwest::Client::new();
        let unauthorized = client
            .get(format!("{}/state", server.control_url))
            .send()
            .await
            .expect("unauthorized request");
        assert_eq!(unauthorized.status(), reqwest::StatusCode::UNAUTHORIZED);
        let updated: SubjectState = client
            .post(format!("{}/advance", server.control_url))
            .header("x-lab-control-token", &server.controller_token)
            .send()
            .await
            .expect("advance")
            .error_for_status()
            .expect("advance status")
            .json()
            .await
            .expect("advance body");
        assert_eq!(updated.version, 2);
        let duplicate = client
            .post(format!("{}/advance", server.control_url))
            .header("x-lab-control-token", &server.controller_token)
            .send()
            .await
            .expect("duplicate");
        assert_eq!(duplicate.status(), reqwest::StatusCode::CONFLICT);
        server.stop().await.expect("stop");
    }
}
