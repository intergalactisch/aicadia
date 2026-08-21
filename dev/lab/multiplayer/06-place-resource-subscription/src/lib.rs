use std::{borrow::Cow, sync::Arc, time::Duration};

use anyhow::{Context, Result, anyhow, ensure};
use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
    routing::{get, post},
};
use rmcp::{
    ClientLifecycleMode, ClientServiceExt, ServerHandler,
    model::{
        CacheScope, ClientInfo, Implementation, ListResourcesResult, ProtocolVersion,
        ReadResourceRequestParams, ReadResourceResponse, ReadResourceResult, Resource,
        ResourceContents, ServerCapabilities, ServerInfo, ServerNotification, SubscriptionFilter,
    },
    service::{RequestContext, RoleServer, SubscriptionContext},
    transport::{
        StreamableHttpClientTransport,
        streamable_http_client::StreamableHttpClientTransportConfig,
        streamable_http_server::{
            StreamableHttpServerConfig, StreamableHttpService, session::local::LocalSessionManager,
        },
    },
};
use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, watch};
use tokio_util::sync::CancellationToken;

pub const RESOURCE_URI: &str = "lab://place/current";
pub const INITIAL_COLOUR: &str = "brown";
pub const UPDATED_COLOUR: &str = "blue";
pub const SERVER_INSTRUCTIONS: &str = "During this active lab turn, treat lab://place/current as the exact watched World context. Read it once, do not poll it, and when its subscription reports a change, reread it before answering.";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceRepresentation {
    pub place: PlaceFact,
    pub entity: Vec<EntityFact>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceFact {
    pub name: String,
    pub revision: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityFact {
    pub name: String,
    pub properties: EntityProperties,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityProperties {
    pub colour: String,
}

impl PlaceRepresentation {
    fn at(revision: u64, colour: &str) -> Self {
        Self {
            place: PlaceFact {
                name: "Workshop".to_owned(),
                revision,
            },
            entity: vec![EntityFact {
                name: "Table".to_owned(),
                properties: EntityProperties {
                    colour: colour.to_owned(),
                },
            }],
        }
    }

    pub fn colour(&self) -> &str {
        &self.entity[0].properties.colour
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Observation {
    pub sequence: u64,
    pub event: String,
    pub detail: String,
}

#[derive(Debug)]
struct Inner {
    current: PlaceRepresentation,
    advanced: bool,
    next_sequence: u64,
    observations: Vec<Observation>,
}

#[derive(Clone, Debug)]
pub struct LabState {
    inner: Arc<Mutex<Inner>>,
    change: watch::Sender<u64>,
}

impl Default for LabState {
    fn default() -> Self {
        Self::new()
    }
}

impl LabState {
    pub fn new() -> Self {
        let (change, _receiver) = watch::channel(1);
        Self {
            inner: Arc::new(Mutex::new(Inner {
                current: PlaceRepresentation::at(1, INITIAL_COLOUR),
                advanced: false,
                next_sequence: 1,
                observations: Vec::new(),
            })),
            change,
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

    pub async fn current(&self) -> PlaceRepresentation {
        self.inner.lock().await.current.clone()
    }

    pub async fn observations(&self) -> Vec<Observation> {
        self.inner.lock().await.observations.clone()
    }

    pub async fn advance_once(&self) -> Result<PlaceRepresentation> {
        let current = {
            let mut inner = self.inner.lock().await;
            ensure!(!inner.advanced, "fixture may advance exactly once");
            inner.advanced = true;
            inner.current = PlaceRepresentation::at(2, UPDATED_COLOUR);
            inner.current.clone()
        };
        self.record("state_advanced", "revision=2 colour=blue")
            .await;
        self.change.send_replace(2);
        Ok(current)
    }
}

#[derive(Clone, Debug)]
pub struct LabServer {
    state: LabState,
}

impl LabServer {
    pub fn new(state: LabState) -> Self {
        Self { state }
    }
}

impl ServerHandler for LabServer {
    fn supported_protocol_versions(&self) -> Cow<'static, [ProtocolVersion]> {
        Cow::Borrowed(&[ProtocolVersion::V_2026_07_28])
    }

    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(
            ServerCapabilities::builder()
                .enable_resources()
                .enable_resources_subscribe()
                .build(),
        )
        .with_server_info(Implementation::new(
            "aicadia-place-resource-subscription-lab",
            "0.1.0",
        ))
        .with_instructions(SERVER_INSTRUCTIONS)
    }

    async fn list_resources(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, rmcp::ErrorData> {
        self.state.record("resources_listed", RESOURCE_URI).await;
        Ok(ListResourcesResult::with_all_items(vec![
            Resource::new(RESOURCE_URI, "current-lab-place")
                .with_title("Current lab Place")
                .with_description("One bounded simulated Place fixture for subscription testing")
                .with_mime_type("application/json"),
        ]))
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResponse, rmcp::ErrorData> {
        if request.uri != RESOURCE_URI {
            return Err(rmcp::ErrorData::resource_not_found(
                "unknown lab resource",
                None,
            ));
        }
        let current = self.state.current().await;
        self.state
            .record(
                "resource_read",
                format!(
                    "uri={RESOURCE_URI} revision={} colour={}",
                    current.place.revision,
                    current.colour()
                ),
            )
            .await;
        let text = serde_json::to_string(&current)
            .map_err(|error| rmcp::ErrorData::internal_error(error.to_string(), None))?;
        Ok(ReadResourceResult::new(vec![
            ResourceContents::text(text, RESOURCE_URI).with_mime_type("application/json"),
        ])
        .with_ttl_ms(0)
        .with_cache_scope(CacheScope::Private)
        .into())
    }

    fn accepted_subscription_filter(
        &self,
        requested: &SubscriptionFilter,
    ) -> Option<SubscriptionFilter> {
        let accepts_exact_uri = requested
            .resource_subscriptions
            .as_ref()
            .is_some_and(|uris| uris.iter().any(|uri| uri == RESOURCE_URI));
        Some(if accepts_exact_uri {
            SubscriptionFilter::builder()
                .resource_subscription(RESOURCE_URI)
                .build()
        } else {
            SubscriptionFilter::new()
        })
    }

    async fn listen(&self, context: SubscriptionContext) -> Result<(), rmcp::ErrorData> {
        self.state.record("listen_started", RESOURCE_URI).await;
        let mut change = self.state.change.subscribe();
        loop {
            tokio::select! {
                _ = context.cancelled() => {
                    self.state.record("listen_cancelled", RESOURCE_URI).await;
                    return Ok(());
                }
                changed = change.changed() => {
                    if changed.is_err() {
                        return Ok(());
                    }
                    let revision = *change.borrow_and_update();
                    if revision < 2 {
                        continue;
                    }
                    context
                        .sink()
                        .notify_resource_updated(RESOURCE_URI)
                        .await
                        .map_err(|error| rmcp::ErrorData::internal_error(error.to_string(), None))?;
                    self.state
                        .record("notification_written", format!("uri={RESOURCE_URI} revision={revision}"))
                        .await;
                }
            }
        }
    }
}

pub struct RunningServer {
    pub url: String,
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
    let cancellation = CancellationToken::new();
    let handler = LabServer::new(state.clone());
    let service: StreamableHttpService<LabServer, LocalSessionManager> = StreamableHttpService::new(
        move || Ok(handler.clone()),
        Default::default(),
        StreamableHttpServerConfig::default()
            .with_json_response(true)
            .with_sse_keep_alive(Some(Duration::from_millis(100)))
            .with_cancellation_token(cancellation.child_token()),
    );
    let observed_state = state.clone();
    let router =
        axum::Router::new()
            .nest_service("/mcp", service)
            .layer(axum::middleware::from_fn(move |request, next| {
                observe_http_request(observed_state.clone(), request, next)
            }));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .context("bind lab server")?;
    let address = listener.local_addr().context("read lab server address")?;
    let task_cancellation = cancellation.clone();
    let task = tokio::spawn(async move {
        let _ = axum::serve(listener, router)
            .with_graceful_shutdown(async move { task_cancellation.cancelled_owned().await })
            .await;
    });
    Ok(RunningServer {
        url: format!("http://{address}/mcp"),
        state,
        cancellation,
        task,
    })
}

async fn observe_http_request(
    state: LabState,
    request: axum::extract::Request,
    next: Next,
) -> Response {
    if request.uri().path() == "/mcp" {
        let protocol_method = request
            .headers()
            .get("Mcp-Method")
            .and_then(|value| value.to_str().ok())
            .unwrap_or("unknown");
        state
            .record(
                "http_request",
                format!(
                    "http_method={} mcp_method={protocol_method}",
                    request.method()
                ),
            )
            .await;
    }
    next.run(request).await
}

#[derive(Clone)]
struct ControllerState {
    lab: LabState,
    token: Arc<str>,
}

fn controller_authorized(headers: &HeaderMap, expected: &str) -> bool {
    headers
        .get("x-lab-control-token")
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value == expected)
}

async fn controller_state(
    State(state): State<ControllerState>,
    headers: HeaderMap,
) -> Result<Json<PlaceRepresentation>, StatusCode> {
    if !controller_authorized(&headers, &state.token) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(Json(state.lab.current().await))
}

async fn controller_observations(
    State(state): State<ControllerState>,
    headers: HeaderMap,
) -> Result<Json<Vec<Observation>>, StatusCode> {
    if !controller_authorized(&headers, &state.token) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(Json(state.lab.observations().await))
}

async fn controller_advance(
    State(state): State<ControllerState>,
    headers: HeaderMap,
) -> Result<Json<PlaceRepresentation>, (StatusCode, Json<serde_json::Value>)> {
    if !controller_authorized(&headers, &state.token) {
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalServerReady {
    pub mcp_url: String,
    pub control_url: String,
}

pub async fn serve_external(control_token: String) -> Result<ExternalServerReady> {
    ensure!(control_token.len() >= 32, "controller token is too short");
    let state = LabState::new();
    let cancellation = CancellationToken::new();
    let handler = LabServer::new(state.clone());
    let service: StreamableHttpService<LabServer, LocalSessionManager> = StreamableHttpService::new(
        move || Ok(handler.clone()),
        Default::default(),
        StreamableHttpServerConfig::default()
            .with_json_response(true)
            .with_sse_keep_alive(Some(Duration::from_millis(100)))
            .with_cancellation_token(cancellation.child_token()),
    );
    let controller = ControllerState {
        lab: state.clone(),
        token: Arc::from(control_token),
    };
    let observed_state = state.clone();
    let router = axum::Router::new()
        .nest_service("/mcp", service)
        .route("/control/state", get(controller_state))
        .route("/control/observations", get(controller_observations))
        .route("/control/advance", post(controller_advance))
        .with_state(controller)
        .layer(axum::middleware::from_fn(move |request, next| {
            observe_http_request(observed_state.clone(), request, next)
        }));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .context("bind external lab server")?;
    let address = listener.local_addr().context("read external lab address")?;
    let ready = ExternalServerReady {
        mcp_url: format!("http://{address}/mcp"),
        control_url: format!("http://{address}/control"),
    };
    tokio::spawn(async move {
        let _ = axum::serve(listener, router).await;
    });
    Ok(ready)
}

async fn connect(url: &str) -> Result<rmcp::service::RunningService<rmcp::RoleClient, ClientInfo>> {
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

fn parse_representation(result: &ReadResourceResult) -> Result<PlaceRepresentation> {
    ensure!(result.contents.len() == 1, "expected one resource content");
    let ResourceContents::TextResourceContents { uri, text, .. } = &result.contents[0] else {
        return Err(anyhow!("expected text resource content"));
    };
    ensure!(uri == RESOURCE_URI, "unexpected resource URI {uri}");
    serde_json::from_str(text).context("decode Place representation")
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScenarioReport {
    pub scenario: String,
    pub initial: PlaceRepresentation,
    pub final_state: PlaceRepresentation,
    pub notification_uri: Option<String>,
    pub observations: Vec<Observation>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreflightReport {
    pub protocol_version: String,
    pub connected_update: ScenarioReport,
    pub reconnect_recovery: ScenarioReport,
    pub agent_processes_launched: u8,
}

pub async fn connected_update_scenario() -> Result<ScenarioReport> {
    let server = spawn_server().await?;
    let client = connect(&server.url).await?;
    let resources = client.list_resources(None).await?;
    ensure!(
        resources.resources.len() == 1,
        "expected one listed resource"
    );
    ensure!(
        resources.resources[0].uri == RESOURCE_URI,
        "wrong listed URI"
    );

    let mut subscription = client
        .listen(
            SubscriptionFilter::builder()
                .resource_subscription(RESOURCE_URI)
                .build(),
        )
        .await?;
    ensure!(
        subscription
            .acknowledged()
            .resource_subscriptions
            .as_ref()
            .is_some_and(|uris| uris == &[RESOURCE_URI.to_owned()]),
        "server did not acknowledge the exact resource URI"
    );

    let initial = parse_representation(
        &client
            .read_resource(ReadResourceRequestParams::new(RESOURCE_URI))
            .await?,
    )?;
    ensure!(initial.place.revision == 1 && initial.colour() == INITIAL_COLOUR);
    server.state.advance_once().await?;

    let notification = tokio::time::timeout(Duration::from_secs(5), subscription.next())
        .await
        .context("timed out waiting for resource notification")??
        .ok_or_else(|| anyhow!("subscription closed before resource notification"))?;
    let ServerNotification::ResourceUpdatedNotification(notification) = notification else {
        return Err(anyhow!("expected a resource-updated notification"));
    };
    ensure!(notification.params.uri == RESOURCE_URI);
    let final_state = parse_representation(
        &client
            .read_resource(ReadResourceRequestParams::new(RESOURCE_URI))
            .await?,
    )?;
    ensure!(final_state.place.revision == 2 && final_state.colour() == UPDATED_COLOUR);

    subscription.cancel().await?;
    client.cancel().await?;
    let observations = server.state.observations().await;
    server.stop().await?;
    Ok(ScenarioReport {
        scenario: "connected_update".to_owned(),
        initial,
        final_state,
        notification_uri: Some(notification.params.uri),
        observations,
    })
}

pub async fn reconnect_recovery_scenario() -> Result<ScenarioReport> {
    let server = spawn_server().await?;
    let first_client = connect(&server.url).await?;
    let initial = parse_representation(
        &first_client
            .read_resource(ReadResourceRequestParams::new(RESOURCE_URI))
            .await?,
    )?;
    ensure!(initial.place.revision == 1 && initial.colour() == INITIAL_COLOUR);
    first_client.cancel().await?;

    server.state.advance_once().await?;

    let second_client = connect(&server.url).await?;
    let mut subscription = second_client
        .listen(
            SubscriptionFilter::builder()
                .resource_subscription(RESOURCE_URI)
                .build(),
        )
        .await?;
    let final_state = parse_representation(
        &second_client
            .read_resource(ReadResourceRequestParams::new(RESOURCE_URI))
            .await?,
    )?;
    ensure!(final_state.place.revision == 2 && final_state.colour() == UPDATED_COLOUR);

    subscription.cancel().await?;
    second_client.cancel().await?;
    let observations = server.state.observations().await;
    server.stop().await?;
    Ok(ScenarioReport {
        scenario: "reconnect_recovery".to_owned(),
        initial,
        final_state,
        notification_uri: None,
        observations,
    })
}

pub async fn run_preflight() -> Result<PreflightReport> {
    Ok(PreflightReport {
        protocol_version: "2026-07-28".to_owned(),
        connected_update: connected_update_scenario().await?,
        reconnect_recovery: reconnect_recovery_scenario().await?,
        agent_processes_launched: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn connected_update_is_an_exact_hint_followed_by_authoritative_reread() {
        let report = connected_update_scenario()
            .await
            .expect("connected scenario");
        assert_eq!(report.notification_uri.as_deref(), Some(RESOURCE_URI));
        assert_eq!(report.initial.colour(), INITIAL_COLOUR);
        assert_eq!(report.final_state.colour(), UPDATED_COLOUR);
        assert!(
            report
                .observations
                .iter()
                .any(|item| item.event == "listen_started")
        );
        assert!(
            report
                .observations
                .iter()
                .any(|item| item.event == "notification_written")
        );
        assert_eq!(
            report
                .observations
                .iter()
                .filter(|item| item.event == "resource_read")
                .count(),
            2
        );
    }

    #[tokio::test]
    async fn reconnect_recovers_current_truth_without_notification_replay() {
        let report = reconnect_recovery_scenario()
            .await
            .expect("reconnect scenario");
        assert_eq!(report.notification_uri, None);
        assert_eq!(report.initial.place.revision, 1);
        assert_eq!(report.final_state.place.revision, 2);
        assert!(
            !report
                .observations
                .iter()
                .any(|item| item.event == "notification_written")
        );
    }

    #[tokio::test]
    async fn fixture_advances_exactly_once() {
        let state = LabState::new();
        state.advance_once().await.expect("first advance");
        assert!(state.advance_once().await.is_err());
        assert_eq!(state.current().await.colour(), UPDATED_COLOUR);
    }

    #[tokio::test]
    async fn external_controller_is_secret_guarded_and_one_way() {
        let token = "0123456789abcdef0123456789abcdef";
        let ready = serve_external(token.to_owned())
            .await
            .expect("external server");
        let client = reqwest::Client::new();
        let unauthorized = client
            .get(format!("{}/state", ready.control_url))
            .send()
            .await
            .expect("unauthorized request");
        assert_eq!(unauthorized.status(), reqwest::StatusCode::UNAUTHORIZED);

        let initial: PlaceRepresentation = client
            .get(format!("{}/state", ready.control_url))
            .header("x-lab-control-token", token)
            .send()
            .await
            .expect("initial request")
            .error_for_status()
            .expect("initial status")
            .json()
            .await
            .expect("initial body");
        assert_eq!(initial.colour(), INITIAL_COLOUR);

        let updated: PlaceRepresentation = client
            .post(format!("{}/advance", ready.control_url))
            .header("x-lab-control-token", token)
            .send()
            .await
            .expect("advance request")
            .error_for_status()
            .expect("advance status")
            .json()
            .await
            .expect("advance body");
        assert_eq!(updated.colour(), UPDATED_COLOUR);

        let duplicate = client
            .post(format!("{}/advance", ready.control_url))
            .header("x-lab-control-token", token)
            .send()
            .await
            .expect("duplicate request");
        assert_eq!(duplicate.status(), reqwest::StatusCode::CONFLICT);
    }
}
