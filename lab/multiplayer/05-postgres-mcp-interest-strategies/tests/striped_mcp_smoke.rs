use std::{borrow::Cow, collections::HashMap, num::NonZeroUsize, sync::Arc, time::Duration};

use anyhow::{Context, Result, anyhow};
use postgres_mcp_interest_strategies_lab::{
    fanout::{RecipientId, ResourceId, RouteObservation, StripeCapacity, StripedRouter},
    gateway::{HostId, ResourceUri},
    mcp::{
        AuthoritativeResourceReader, McpLabClient, MemoryResourceReader, spawn_handler_loopback,
    },
};
use rmcp::{
    RoleServer, ServerHandler,
    model::{
        CacheScope, Implementation, ProtocolVersion, ReadResourceRequestParams,
        ReadResourceResponse, ReadResourceResult, ResourceContents, ServerCapabilities, ServerInfo,
        SubscriptionFilter,
    },
    service::{RequestContext, SubscriptionContext},
};
use serde_json::json;
use tokio::{sync::mpsc, time::timeout};
use uuid::Uuid;

const STRIPES: usize = 64;
const TOTAL_CAPACITY: usize = 4_160;
const RESOURCE_ID: ResourceId = ResourceId(1);
const RECIPIENT_ID: RecipientId = RecipientId(1);
const STEP_TIMEOUT: Duration = Duration::from_secs(3);

#[derive(Clone)]
struct StripedMcpTestServer {
    router: Arc<StripedRouter>,
    reader: MemoryResourceReader,
    resource: ResourceUri,
    deliveries: Arc<std::sync::Mutex<HashMap<RecipientId, mpsc::Sender<ResourceUri>>>>,
}

impl StripedMcpTestServer {
    fn route_and_deliver(&self) -> Result<RouteObservation> {
        let observation = self.router.route(RESOURCE_ID);
        let senders = {
            let deliveries = self.deliveries.lock().expect("delivery map poisoned");
            observation
                .newly_dirty_recipients
                .iter()
                .map(|recipient| {
                    deliveries
                        .get(recipient)
                        .cloned()
                        .ok_or_else(|| anyhow!("striped recipient has no MCP delivery sink"))
                })
                .collect::<Result<Vec<_>>>()?
        };
        for sender in senders {
            sender
                .try_send(self.resource.clone())
                .map_err(|error| anyhow!("MCP delivery sink rejected striped update: {error}"))?;
        }
        Ok(observation)
    }
}

impl ServerHandler for StripedMcpTestServer {
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
        .with_server_info(Implementation::new("striped-router-rmcp-smoke", "0.0.0"))
    }

    fn accepted_subscription_filter(
        &self,
        requested: &SubscriptionFilter,
    ) -> Option<SubscriptionFilter> {
        let accepted = requested
            .resource_subscriptions
            .as_deref()
            .unwrap_or_default()
            .iter()
            .filter(|uri| uri.as_str() == self.resource.as_str())
            .cloned()
            .collect::<Vec<_>>();
        Some(
            SubscriptionFilter::builder()
                .resource_subscriptions(accepted)
                .build(),
        )
    }

    async fn listen(&self, context: SubscriptionContext) -> Result<(), rmcp::ErrorData> {
        let (sender, mut receiver) = mpsc::channel(4);
        {
            let mut deliveries = self.deliveries.lock().expect("delivery map poisoned");
            if deliveries.contains_key(&RECIPIENT_ID) {
                return Err(invalid_params("duplicate MCP recipient"));
            }
            deliveries.insert(RECIPIENT_ID, sender);
        }
        match self.router.subscribe(RESOURCE_ID, RECIPIENT_ID) {
            Ok(true) => {}
            Ok(false) => {
                self.deliveries
                    .lock()
                    .expect("delivery map poisoned")
                    .remove(&RECIPIENT_ID);
                return Err(invalid_params("duplicate striped subscription"));
            }
            Err(error) => {
                self.deliveries
                    .lock()
                    .expect("delivery map poisoned")
                    .remove(&RECIPIENT_ID);
                return Err(invalid_params(error));
            }
        }
        let sink = context.sink().clone();

        loop {
            tokio::select! {
                _ = context.cancelled() => break,
                resource = receiver.recv() => {
                    let Some(resource) = resource else { break };
                    if sink.notify_resource_updated(resource.to_string()).await.is_err() {
                        break;
                    }
                }
            }
        }
        self.deliveries
            .lock()
            .expect("delivery map poisoned")
            .remove(&RECIPIENT_ID);
        self.router.unsubscribe(RESOURCE_ID, RECIPIENT_ID);
        Ok(())
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResponse, rmcp::ErrorData> {
        if request.uri != self.resource.as_str() {
            return Err(rmcp::ErrorData::resource_not_found(
                "unknown striped test resource",
                None,
            ));
        }
        self.router
            .begin_authoritative_read(RESOURCE_ID, RECIPIENT_ID);
        let authoritative = self
            .reader
            .read_resource(&self.resource, 1)
            .await
            .map_err(invalid_params)?;
        let text = serde_json::to_string(&authoritative.document).map_err(invalid_params)?;
        Ok(ReadResourceResult::new(vec![
            ResourceContents::text(text, self.resource.to_string())
                .with_mime_type("application/json"),
        ])
        .with_ttl_ms(0)
        .with_cache_scope(CacheScope::Private)
        .into())
    }
}

fn invalid_params(error: impl std::fmt::Display) -> rmcp::ErrorData {
    rmcp::ErrorData::invalid_params(error.to_string(), None)
}

#[tokio::test]
async fn sixty_four_stripe_candidate_drives_real_rmcp_update_and_read() -> Result<()> {
    assert_eq!(TOTAL_CAPACITY % STRIPES, 0);
    let capacity = StripeCapacity {
        max_subscription_pairs: TOTAL_CAPACITY / STRIPES,
        max_pending_keys: TOTAL_CAPACITY / STRIPES,
    };
    let router = Arc::new(StripedRouter::empty(vec![capacity; STRIPES])?);
    assert_eq!(router.stripe_count(), STRIPES);
    assert_eq!(
        router.configured_capacity().max_subscription_pairs,
        TOTAL_CAPACITY
    );
    assert_eq!(
        router.configured_capacity().max_pending_keys,
        TOTAL_CAPACITY
    );

    let resource = ResourceUri::entity(Uuid::from_u128(0x6401));
    let reader = MemoryResourceReader::default();
    reader.insert(resource.clone(), json!({"revision": 1}));
    let handler = StripedMcpTestServer {
        router: router.clone(),
        reader: reader.clone(),
        resource: resource.clone(),
        deliveries: Arc::default(),
    };
    let server = spawn_handler_loopback(handler.clone()).await?;
    let client = McpLabClient::connect(&server.url, HostId::new("striped-rmcp-host")?).await?;
    let mut subscription = client
        .listen([resource.clone()], NonZeroUsize::new(4).unwrap())
        .await?;
    timeout(STEP_TIMEOUT, async {
        while router.subscription_pair_count() != 1 {
            tokio::task::yield_now().await;
        }
    })
    .await
    .context("striped MCP subscription was not installed")?;

    assert_eq!(
        client.read_resource(&resource).await?.document["revision"],
        1
    );
    reader.insert(resource.clone(), json!({"revision": 2}));
    let first = handler.route_and_deliver()?;
    assert_eq!(first.newly_dirty, 1);
    assert_eq!(first.newly_dirty_recipients, vec![RECIPIENT_ID]);
    assert_eq!(
        timeout(STEP_TIMEOUT, subscription.next_updated()).await??,
        Some(resource.clone())
    );

    let coalesced = handler.route_and_deliver()?;
    assert_eq!(coalesced.newly_dirty, 0);
    assert_eq!(coalesced.coalesced, 1);
    assert!(coalesced.newly_dirty_recipients.is_empty());
    assert_eq!(
        client.read_resource(&resource).await?.document["revision"],
        2
    );
    assert_eq!(router.pending_count(), 0);

    reader.insert(resource.clone(), json!({"revision": 3}));
    assert_eq!(handler.route_and_deliver()?.newly_dirty, 1);
    assert_eq!(
        timeout(STEP_TIMEOUT, subscription.next_updated()).await??,
        Some(resource.clone())
    );
    assert_eq!(
        client.read_resource(&resource).await?.document["revision"],
        3
    );

    subscription.cancel().await?;
    timeout(STEP_TIMEOUT, async {
        while router.subscription_pair_count() != 0 || router.pending_count() != 0 {
            tokio::task::yield_now().await;
        }
    })
    .await
    .context("striped MCP teardown did not clear bounded state")?;
    client.close().await?;
    server.shutdown().await?;
    Ok(())
}
