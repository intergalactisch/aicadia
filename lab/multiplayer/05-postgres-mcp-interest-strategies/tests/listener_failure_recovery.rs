use std::{num::NonZeroUsize, time::Duration};

use anyhow::{Context, Result, bail};
use postgres_mcp_interest_strategies_lab::{
    gateway::{Gateway, GatewayBounds, HostId, ResourceUri},
    mcp::{InterestMcpServer, McpLabClient, spawn_loopback},
    strategy::{ChangeScope, ResourceKey, Strategy},
    world::{ChangeRequest, NOTIFY_CHANNEL, ResourceMutation, SeedResource, World},
};
use serde_json::json;
use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::time::timeout;
use uuid::Uuid;

const STEP_TIMEOUT: Duration = Duration::from_secs(3);
const TEST_TIMEOUT: Duration = Duration::from_secs(15);

#[sqlx::test(migrations = "./migration")]
async fn fatal_listener_loss_ends_mcp_stream_and_replacement_baseline_recovers(
    pool: PgPool,
) -> Result<()> {
    timeout(TEST_TIMEOUT, run_failure_scenario(pool))
        .await
        .context("fatal-listener recovery exceeded 15 seconds")?
}

async fn run_failure_scenario(pool: PgPool) -> Result<()> {
    let area_id = Uuid::from_u128(0xf001);
    let place_id = Uuid::from_u128(0xf002);
    let entity_id = Uuid::from_u128(0xf003);
    let missed_change_id = Uuid::from_u128(0xf004);
    let entity_uri = ResourceUri::entity(entity_id);
    let world = World::new(pool.clone());
    world
        .seed_resources(&[
            SeedResource::new(ResourceKey::World, None, "World", json!({})),
            SeedResource::new(
                ResourceKey::Area(area_id),
                Some(ResourceKey::World),
                "Area",
                json!({}),
            ),
            SeedResource::new(
                ResourceKey::Place(place_id),
                Some(ResourceKey::Area(area_id)),
                "Place",
                json!({}),
            ),
            SeedResource::new(
                ResourceKey::Entity(entity_id),
                Some(ResourceKey::Place(place_id)),
                "Tree",
                json!({"revision": 0}),
            ),
        ])
        .await?;

    let listener_pool = dedicated_listener_pool(&pool).await?;
    let gateway =
        Gateway::connect_with(&listener_pool, NOTIFY_CHANNEL, GatewayBounds::default()).await?;
    let server = spawn_loopback(InterestMcpServer::new(gateway.clone(), world.clone())).await?;
    let client = McpLabClient::connect(&server.url, HostId::new("listener-loss-host")?).await?;
    let mut subscription = client
        .listen([entity_uri.clone()], NonZeroUsize::new(4).unwrap())
        .await?;

    let initial = client.read_resource(&entity_uri).await?;
    assert_eq!(initial.document["current"]["state"]["revision"], 0);
    assert!(
        initial.document["recent_activities"]
            .as_array()
            .unwrap()
            .is_empty()
    );

    timeout(STEP_TIMEOUT, listener_pool.close())
        .await
        .context("dedicated listener pool did not close")?;
    timeout(STEP_TIMEOUT, async {
        loop {
            let metrics = gateway.metrics();
            if metrics.database_listener_errors == 1 && metrics.active_subscriptions == 0 {
                break;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .context("gateway did not expose fatal listener failure")?;
    assert_eq!(
        timeout(STEP_TIMEOUT, subscription.next_updated())
            .await
            .context("old MCP stream did not end")??,
        None
    );
    assert_eq!(gateway.metrics().listener_failure_terminations, 1);
    let mut stale_attempt = client
        .listen([entity_uri.clone()], NonZeroUsize::new(4).unwrap())
        .await?;
    assert!(
        timeout(STEP_TIMEOUT, stale_attempt.next_updated())
            .await
            .context("post-failure subscription did not terminate")?
            .is_err(),
        "failed gateway must not leave a new subscription healthy-looking"
    );
    drop(stale_attempt);

    world
        .apply_change(
            Strategy::ExactOnly,
            ChangeRequest {
                change_id: missed_change_id,
                operation: "listener_offline_tree_change".to_owned(),
                scope: ChangeScope::Local,
                primary_entity_id: Some(entity_id),
                primary_place_id: Some(place_id),
                affected_place_ids: vec![place_id],
                mutations: vec![ResourceMutation::new(
                    ResourceKey::Entity(entity_id),
                    Some(ResourceKey::Place(place_id)),
                    "Tree",
                    json!({"revision": 1}),
                )],
            },
        )
        .await?;
    assert_eq!(gateway.metrics().database_notifications, 0);

    drop(subscription);
    client.close().await?;
    server.shutdown().await?;
    gateway.shutdown().await;

    let replacement_pool = dedicated_listener_pool(&pool).await?;
    let replacement_gateway =
        Gateway::connect_with(&replacement_pool, NOTIFY_CHANNEL, GatewayBounds::default()).await?;
    let replacement_server =
        spawn_loopback(InterestMcpServer::new(replacement_gateway.clone(), world)).await?;
    let replacement_client = McpLabClient::connect(
        &replacement_server.url,
        HostId::new("replacement-listener-host")?,
    )
    .await?;
    let replacement_subscription = replacement_client
        .listen([entity_uri.clone()], NonZeroUsize::new(4).unwrap())
        .await?;

    // The missed hint is not replayed. Listen first, then recover solely through
    // the bounded authoritative baseline.
    let recovered = replacement_client.read_resource(&entity_uri).await?;
    assert_eq!(recovered.document["current"]["state"]["revision"], 1);
    let activities = recovered.document["recent_activities"]
        .as_array()
        .context("authoritative baseline activities")?;
    if !activities
        .iter()
        .any(|activity| activity["id"] == missed_change_id.to_string())
    {
        bail!("replacement baseline did not contain the Activity missed while offline");
    }
    assert_eq!(replacement_gateway.metrics().database_notifications, 0);

    replacement_subscription.cancel().await?;
    replacement_client.close().await?;
    replacement_server.shutdown().await?;
    replacement_gateway.shutdown().await;
    timeout(STEP_TIMEOUT, replacement_pool.close())
        .await
        .context("replacement listener pool did not close")?;
    Ok(())
}

async fn dedicated_listener_pool(source: &PgPool) -> Result<PgPool> {
    PgPoolOptions::new()
        .max_connections(1)
        .connect_with((*source.connect_options()).clone())
        .await
        .context("connect dedicated PgListener pool")
}
