//! Real rmcp 3.1.1 Streamable HTTP seam for the interest lab.

use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    future::Future,
    num::NonZeroUsize,
    sync::Arc,
    time::Duration,
};

use anyhow::{Context, Result, anyhow, bail};
use axum::Router;
use http::{HeaderName, HeaderValue, request::Parts};
use rmcp::{
    ClientLifecycleMode, ClientServiceExt, RoleClient, RoleServer, ServerHandler,
    model::{
        CacheScope, ClientInfo, Implementation, ProtocolVersion, ReadResourceRequestParams,
        ReadResourceResponse, ReadResourceResult, ResourceContents, ServerCapabilities, ServerInfo,
        ServerNotification, SubscriptionFilter,
    },
    service::{RequestContext, RunningService, Subscription, SubscriptionContext, SubscriptionEnd},
    transport::{
        StreamableHttpClientTransport,
        streamable_http_client::StreamableHttpClientTransportConfig,
        streamable_http_server::{
            StreamableHttpServerConfig, StreamableHttpService, session::local::LocalSessionManager,
        },
    },
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use crate::gateway::{Gateway, HostId, ResourceUri};

pub const LAB_HOST_HEADER: &str = "x-lab-host-id";
pub const DEFAULT_ACTIVITY_LIMIT: usize = 32;

/// Minimal adapter expected from the experiment's authoritative World fixture.
/// It deliberately contains no subscription or transport behavior.
pub trait AuthoritativeResourceReader: Clone + Send + Sync + 'static {
    /// Synchronous structural admission for an exact URI. This must never infer
    /// meaning from resource content.
    fn accepts_resource(&self, resource: &ResourceUri) -> bool;

    /// Perform the actual bounded database read.
    fn read_resource(
        &self,
        resource: &ResourceUri,
        activity_limit: usize,
    ) -> impl Future<Output = Result<AuthoritativeResource>> + Send;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthoritativeResource {
    pub document: Value,
    pub rows: u64,
}

impl AuthoritativeResource {
    pub fn new(document: impl Serialize, rows: u64) -> Result<Self> {
        Ok(Self {
            document: serde_json::to_value(document)?,
            rows,
        })
    }
}

#[derive(Clone)]
pub struct InterestMcpServer<R> {
    gateway: Gateway,
    reader: R,
    activity_limit: usize,
}

impl<R> InterestMcpServer<R>
where
    R: AuthoritativeResourceReader,
{
    pub fn new(gateway: Gateway, reader: R) -> Self {
        Self {
            gateway,
            reader,
            activity_limit: DEFAULT_ACTIVITY_LIMIT,
        }
    }

    pub fn with_activity_limit(mut self, activity_limit: usize) -> Result<Self> {
        if activity_limit == 0 || activity_limit > 256 {
            bail!("activity limit must contain 1..=256 rows");
        }
        self.activity_limit = activity_limit;
        Ok(self)
    }

    pub fn gateway(&self) -> &Gateway {
        &self.gateway
    }

    fn exact_filter(&self, requested: &SubscriptionFilter) -> SubscriptionFilter {
        let mut accepted = Vec::new();
        let mut seen = HashSet::new();
        for raw in requested
            .resource_subscriptions
            .as_deref()
            .unwrap_or_default()
            .iter()
            .take(self.gateway.bounds().max_resources_per_subscription)
        {
            let Ok(resource) = ResourceUri::parse(raw.clone()) else {
                continue;
            };
            if self.reader.accepts_resource(&resource) && seen.insert(resource.clone()) {
                accepted.push(resource.to_string());
            }
        }
        SubscriptionFilter::builder()
            .resource_subscriptions(accepted)
            .build()
    }
}

impl<R> ServerHandler for InterestMcpServer<R>
where
    R: AuthoritativeResourceReader,
{
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
            "aicadia-interest-strategy-lab",
            "0.0.0",
        ))
        .with_instructions(
            "Experimental exact resource invalidation. Notifications require an authoritative resources/read; they never invoke an Agent.",
        )
    }

    fn accepted_subscription_filter(
        &self,
        requested: &SubscriptionFilter,
    ) -> Option<SubscriptionFilter> {
        Some(self.exact_filter(requested))
    }

    async fn listen(&self, context: SubscriptionContext) -> Result<(), rmcp::ErrorData> {
        let host_id = host_id(context.request_context())?;
        let resources = context
            .accepted()
            .resource_subscriptions
            .as_deref()
            .unwrap_or_default()
            .iter()
            .map(|uri| ResourceUri::parse(uri.clone()))
            .collect::<Result<Vec<_>>>()
            .map_err(invalid_params)?;
        let mut registration = self
            .gateway
            .register(host_id, resources)
            .map_err(invalid_params)?;
        let sink = context.sink().clone();

        loop {
            tokio::select! {
                _ = context.cancelled() => return Ok(()),
                resource = registration.recv() => {
                    let Some(resource) = resource else {
                        return Ok(());
                    };
                    if sink.notify_resource_updated(resource.to_string()).await.is_err() {
                        return Ok(());
                    }
                }
            }
        }
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResponse, rmcp::ErrorData> {
        let host_id = host_id(&context)?;
        let resource = ResourceUri::parse(request.uri.clone()).map_err(invalid_params)?;
        if !self.reader.accepts_resource(&resource) {
            return Err(rmcp::ErrorData::resource_not_found(
                "unknown experimental resource",
                None,
            ));
        }

        let cleared = self.gateway.begin_authoritative_read(&host_id, &resource);
        let authoritative = match self
            .reader
            .read_resource(&resource, self.activity_limit)
            .await
        {
            Ok(authoritative) => authoritative,
            Err(error) => {
                if cleared {
                    self.gateway.restore_after_failed_read(&host_id, &resource);
                }
                return Err(rmcp::ErrorData::internal_error(
                    format!("authoritative resource read failed: {error:#}"),
                    None,
                ));
            }
        };
        let text = serde_json::to_string(&authoritative.document).map_err(|error| {
            rmcp::ErrorData::internal_error(format!("serialize resource: {error}"), None)
        })?;
        self.gateway
            .record_authoritative_read(authoritative.rows, text.len() as u64);
        Ok(ReadResourceResult::new(vec![
            ResourceContents::text(text, resource.to_string()).with_mime_type("application/json"),
        ])
        .with_ttl_ms(0)
        .with_cache_scope(CacheScope::Private)
        .into())
    }
}

fn host_id(context: &RequestContext<RoleServer>) -> Result<HostId, rmcp::ErrorData> {
    let parts = context
        .extensions
        .get::<Parts>()
        .ok_or_else(|| invalid_params("missing HTTP request parts"))?;
    let raw = parts
        .headers
        .get(LAB_HOST_HEADER)
        .ok_or_else(|| invalid_params(format!("missing {LAB_HOST_HEADER} header")))?
        .to_str()
        .map_err(invalid_params)?;
    HostId::new(raw).map_err(invalid_params)
}

fn invalid_params(error: impl std::fmt::Display) -> rmcp::ErrorData {
    rmcp::ErrorData::invalid_params(error.to_string(), None)
}

pub struct LoopbackMcpServer {
    pub url: String,
    cancellation: CancellationToken,
    task: Option<JoinHandle<std::io::Result<()>>>,
}

impl LoopbackMcpServer {
    pub async fn shutdown(mut self) -> Result<()> {
        self.cancellation.cancel();
        if let Some(task) = self.task.take() {
            task.await.context("join loopback MCP server")??;
        }
        Ok(())
    }
}

impl Drop for LoopbackMcpServer {
    fn drop(&mut self) {
        self.cancellation.cancel();
        if let Some(task) = self.task.take() {
            task.abort();
        }
    }
}

pub async fn spawn_loopback<R>(server: InterestMcpServer<R>) -> Result<LoopbackMcpServer>
where
    R: AuthoritativeResourceReader,
{
    spawn_handler_loopback(server).await
}

/// Start any experimental rmcp handler over the same loopback transport. This
/// keeps focused adapter smokes on the identical real protocol seam.
pub async fn spawn_handler_loopback<S>(server: S) -> Result<LoopbackMcpServer>
where
    S: ServerHandler + Clone,
{
    let cancellation = CancellationToken::new();
    let service: StreamableHttpService<S, LocalSessionManager> = StreamableHttpService::new(
        move || Ok(server.clone()),
        Default::default(),
        StreamableHttpServerConfig::default()
            .with_legacy_session_mode(true)
            .with_json_response(true)
            .with_sse_keep_alive(Some(Duration::from_secs(5)))
            .with_cancellation_token(cancellation.child_token()),
    );
    let router = Router::new().nest_service("/mcp", service);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .context("bind loopback MCP listener")?;
    let address = listener.local_addr().context("read MCP listener address")?;
    let server_cancellation = cancellation.clone();
    let task = tokio::spawn(async move {
        axum::serve(listener, router)
            .with_graceful_shutdown(async move {
                server_cancellation.cancelled_owned().await;
            })
            .await
    });
    Ok(LoopbackMcpServer {
        url: format!("http://{address}/mcp"),
        cancellation,
        task: Some(task),
    })
}

pub struct McpLabClient {
    host_id: HostId,
    service: RunningService<RoleClient, ClientInfo>,
}

impl McpLabClient {
    pub async fn connect(url: impl Into<String>, host_id: HostId) -> Result<Self> {
        let header_name = HeaderName::from_static(LAB_HOST_HEADER);
        let header_value = HeaderValue::from_str(host_id.as_str()).context("host header value")?;
        let mut headers = HashMap::new();
        headers.insert(header_name, header_value);
        let transport = StreamableHttpClientTransport::from_config(
            StreamableHttpClientTransportConfig::with_uri(url.into()).custom_headers(headers),
        );
        let service = ClientInfo::default()
            .serve_with_lifecycle(
                transport,
                ClientLifecycleMode::Discover {
                    preferred_versions: vec![ProtocolVersion::V_2026_07_28],
                },
            )
            .await
            .context("connect rmcp Streamable HTTP client")?;
        Ok(Self { host_id, service })
    }

    pub fn host_id(&self) -> &HostId {
        &self.host_id
    }

    pub async fn listen(
        &self,
        resources: impl IntoIterator<Item = ResourceUri>,
        notification_capacity: NonZeroUsize,
    ) -> Result<McpResourceSubscription> {
        let requested: Vec<_> = resources.into_iter().collect();
        let filter = SubscriptionFilter::builder()
            .resource_subscriptions(requested.iter().map(ResourceUri::to_string))
            .build();
        let subscription = self
            .service
            .listen_with_capacity(filter, notification_capacity)
            .await
            .context("open subscriptions/listen")?;
        let acknowledged = subscription
            .acknowledged()
            .resource_subscriptions
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(ResourceUri::parse)
            .collect::<Result<HashSet<_>>>()?;
        let expected: HashSet<_> = requested.into_iter().collect();
        if acknowledged != expected {
            bail!(
                "server acknowledged a different exact resource set: expected {expected:?}, got {acknowledged:?}"
            );
        }
        Ok(McpResourceSubscription {
            acknowledged,
            inner: subscription,
        })
    }

    pub async fn read_resource(&self, resource: &ResourceUri) -> Result<McpResourceRead> {
        let result = self
            .service
            .read_resource(ReadResourceRequestParams::new(resource.to_string()))
            .await
            .context("resources/read")?;
        let Some(ResourceContents::TextResourceContents {
            uri,
            mime_type,
            text,
            ..
        }) = result.contents.into_iter().next()
        else {
            bail!("resource response did not contain one text document");
        };
        if uri != resource.as_str() || mime_type.as_deref() != Some("application/json") {
            bail!("resource response URI or media type did not match the request");
        }
        let bytes = text.len() as u64;
        let document = serde_json::from_str(&text).context("decode authoritative resource JSON")?;
        Ok(McpResourceRead { document, bytes })
    }

    pub async fn close(mut self) -> Result<()> {
        self.service
            .close_with_timeout(Duration::from_secs(5))
            .await
            .context("join rmcp client")?
            .ok_or_else(|| anyhow!("rmcp client close timed out"))?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct McpResourceRead {
    pub document: Value,
    pub bytes: u64,
}

pub struct McpResourceSubscription {
    acknowledged: HashSet<ResourceUri>,
    inner: Subscription,
}

impl McpResourceSubscription {
    pub fn acknowledged(&self) -> &HashSet<ResourceUri> {
        &self.acknowledged
    }

    pub async fn next_updated(&mut self) -> Result<Option<ResourceUri>> {
        let Some(notification) = self.inner.next().await? else {
            return match self.inner.end() {
                Some(SubscriptionEnd::Graceful(_)) => Ok(None),
                _ => Err(anyhow!("subscription ended abruptly")),
            };
        };
        match notification {
            ServerNotification::ResourceUpdatedNotification(update) => {
                ResourceUri::parse(update.params.uri).map(Some)
            }
            other => bail!("unexpected subscription notification: {other:?}"),
        }
    }

    pub async fn cancel(mut self) -> Result<()> {
        self.inner.cancel().await.context("cancel subscription")?;
        Ok(())
    }
}

/// Small reader useful for protocol-only tests. The real scenario adapter must
/// use the PostgreSQL World implementation instead.
#[derive(Clone, Default)]
pub struct MemoryResourceReader {
    documents: Arc<std::sync::RwLock<HashMap<ResourceUri, Value>>>,
}

impl MemoryResourceReader {
    pub fn insert(&self, resource: ResourceUri, document: Value) {
        self.documents
            .write()
            .expect("memory reader poisoned")
            .insert(resource, document);
    }
}

impl AuthoritativeResourceReader for MemoryResourceReader {
    fn accepts_resource(&self, resource: &ResourceUri) -> bool {
        self.documents
            .read()
            .expect("memory reader poisoned")
            .contains_key(resource)
    }

    async fn read_resource(
        &self,
        resource: &ResourceUri,
        _activity_limit: usize,
    ) -> Result<AuthoritativeResource> {
        self.documents
            .read()
            .expect("memory reader poisoned")
            .get(resource)
            .cloned()
            .ok_or_else(|| anyhow!("resource not found"))
            .map(|document| AuthoritativeResource { document, rows: 1 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gateway::GatewayBounds;
    use crate::strategy::{ChangeScope, ResourceKey, Strategy};
    use crate::world::{ChangeRequest, NOTIFY_CHANNEL, ResourceMutation, SeedResource, World};
    use serde_json::json;
    use sqlx::PgPool;
    use uuid::Uuid;

    #[tokio::test]
    async fn real_streamable_http_listen_update_and_authoritative_read() -> Result<()> {
        let gateway = Gateway::new(GatewayBounds::default())?;
        let reader = MemoryResourceReader::default();
        let resource = ResourceUri::entity(Uuid::from_u128(7));
        reader.insert(resource.clone(), json!({"version": 1, "activities": []}));
        let server =
            spawn_loopback(InterestMcpServer::new(gateway.clone(), reader.clone())).await?;
        let client = McpLabClient::connect(&server.url, HostId::new("host-7")?).await?;
        let mut subscription = client
            .listen([resource.clone()], NonZeroUsize::new(2).unwrap())
            .await?;

        assert_eq!(gateway.route(resource.clone()).emitted, 1);
        assert_eq!(subscription.next_updated().await?, Some(resource.clone()));
        assert_eq!(gateway.route(resource.clone()).coalesced, 1);
        assert_eq!(
            client.read_resource(&resource).await?.document["version"],
            1
        );
        assert!(!gateway.is_dirty(client.host_id(), &resource));
        assert_eq!(gateway.metrics().authoritative_reads, 1);

        subscription.cancel().await?;
        client.close().await?;
        server.shutdown().await?;
        Ok(())
    }

    #[tokio::test]
    async fn server_acknowledges_only_exact_known_resources() -> Result<()> {
        let gateway = Gateway::new(GatewayBounds::default())?;
        let reader = MemoryResourceReader::default();
        let known = ResourceUri::place(Uuid::from_u128(1));
        let unknown = ResourceUri::place(Uuid::from_u128(2));
        reader.insert(known.clone(), json!({"place": "known"}));
        let server = InterestMcpServer::new(gateway, reader);
        let requested = SubscriptionFilter::builder()
            .resource_subscriptions([known.to_string(), unknown.to_string()])
            .tools_list_changed()
            .build();
        let accepted = server.exact_filter(&requested);
        assert_eq!(
            accepted.resource_subscriptions,
            Some(vec![known.to_string()])
        );
        assert_eq!(accepted.tools_list_changed, None);
        server.gateway.shutdown().await;
        Ok(())
    }

    #[tokio::test]
    async fn pending_bound_ends_real_mcp_stream_and_requires_new_listen_baseline() -> Result<()> {
        let bounds = GatewayBounds {
            max_pending_keys: 1,
            ..GatewayBounds::default()
        };
        let gateway = Gateway::new(bounds)?;
        let reader = MemoryResourceReader::default();
        let first = ResourceUri::entity(Uuid::from_u128(41));
        let second = ResourceUri::entity(Uuid::from_u128(42));
        reader.insert(first.clone(), json!({"version": 1}));
        reader.insert(second.clone(), json!({"version": 1}));
        let server =
            spawn_loopback(InterestMcpServer::new(gateway.clone(), reader.clone())).await?;
        let host = HostId::new("overloaded-host")?;
        let client = McpLabClient::connect(&server.url, host.clone()).await?;
        let mut subscription = client
            .listen(
                [first.clone(), second.clone()],
                NonZeroUsize::new(2).unwrap(),
            )
            .await?;

        reader.insert(first.clone(), json!({"version": 2, "change": "first"}));
        assert_eq!(gateway.route(first.clone()).emitted, 1);
        assert_eq!(subscription.next_updated().await?, Some(first.clone()));
        reader.insert(
            second.clone(),
            json!({"version": 2, "change": "lost-while-overloaded"}),
        );
        let overflow = gateway.route(second.clone());
        assert_eq!(overflow.terminated, 1);
        assert_eq!(subscription.next_updated().await?, None);
        assert_eq!(gateway.metrics().active_subscriptions, 0);
        assert_eq!(gateway.metrics().pending_keys, 0);

        drop(subscription);
        client.close().await?;
        let client = McpLabClient::connect(&server.url, host).await?;
        let subscription = client
            .listen(
                [first.clone(), second.clone()],
                NonZeroUsize::new(2).unwrap(),
            )
            .await?;
        assert_eq!(client.read_resource(&first).await?.document["version"], 2);
        let recovered = client.read_resource(&second).await?.document;
        assert_eq!(recovered["version"], 2);
        assert_eq!(recovered["change"], "lost-while-overloaded");
        assert_eq!(gateway.metrics().overload_terminations, 1);

        subscription.cancel().await?;
        client.close().await?;
        server.shutdown().await?;
        Ok(())
    }

    #[sqlx::test(migrations = "./migration")]
    async fn postgres_commit_notify_mcp_listen_and_read_form_one_real_chain(
        pool: PgPool,
    ) -> Result<()> {
        let area = Uuid::from_u128(101);
        let place = Uuid::from_u128(102);
        let entity = Uuid::from_u128(103);
        let world = World::new(pool.clone());
        world
            .seed_resources(&[
                SeedResource::new(ResourceKey::World, None, "World", json!({})),
                SeedResource::new(
                    ResourceKey::Area(area),
                    Some(ResourceKey::World),
                    "Area",
                    json!({}),
                ),
                SeedResource::new(
                    ResourceKey::Place(place),
                    Some(ResourceKey::Area(area)),
                    "Place",
                    json!({}),
                ),
                SeedResource::new(
                    ResourceKey::Entity(entity),
                    Some(ResourceKey::Place(place)),
                    "Tree",
                    json!({"state":"standing"}),
                ),
            ])
            .await?;

        let gateway =
            Gateway::connect_with(&pool, NOTIFY_CHANNEL, GatewayBounds::default()).await?;
        let server = spawn_loopback(InterestMcpServer::new(gateway.clone(), world.clone())).await?;
        let client = McpLabClient::connect(&server.url, HostId::new("postgres-host")?).await?;
        let entity_uri = ResourceUri::from(ResourceKey::Entity(entity));
        let mut subscription = client
            .listen([entity_uri.clone()], NonZeroUsize::new(2).unwrap())
            .await?;

        world
            .apply_change(
                Strategy::PlaceAndExact,
                ChangeRequest {
                    change_id: Uuid::new_v4(),
                    operation: "fell_tree".to_owned(),
                    scope: ChangeScope::Local,
                    primary_entity_id: Some(entity),
                    primary_place_id: Some(place),
                    affected_place_ids: vec![place],
                    mutations: vec![ResourceMutation::new(
                        ResourceKey::Entity(entity),
                        Some(ResourceKey::Place(place)),
                        "Tree",
                        json!({"state":"fallen"}),
                    )],
                },
            )
            .await?;

        let updated =
            tokio::time::timeout(Duration::from_secs(2), subscription.next_updated()).await??;
        assert_eq!(updated, Some(entity_uri.clone()));
        let read = client.read_resource(&entity_uri).await?;
        assert_eq!(read.document["current"]["state"]["state"], "fallen");
        assert_eq!(
            read.document["recent_activities"].as_array().unwrap().len(),
            1
        );
        let metrics = gateway.metrics();
        assert_eq!(metrics.database_notifications, 1);
        assert_eq!(metrics.emitted_hints, 1);
        assert_eq!(metrics.authoritative_reads, 1);
        assert_eq!(metrics.pending_keys, 0);

        subscription.cancel().await?;
        client.close().await?;
        server.shutdown().await?;
        gateway.shutdown().await;
        Ok(())
    }
}
