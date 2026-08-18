use std::{
    io::Write,
    net::{Ipv4Addr, SocketAddr},
    str::FromStr,
};

use aicadia::{World, server};
use aicadia_studio as studio;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL")?;
    let database_name = std::env::var("AICADIA_DATABASE_NAME").ok();
    let port = std::env::var("AICADIA_PORT")
        .ok()
        .map(|value| value.parse::<u16>())
        .transpose()?
        .unwrap_or(3000);
    let mut connect_options = PgConnectOptions::from_str(&database_url)?;
    if let Some(database_name) = database_name {
        connect_options = connect_options.database(&database_name);
    }
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect_with(connect_options)
        .await?;
    sqlx::migrate!("../game/migration").run(&pool).await?;

    let listener = tokio::net::TcpListener::bind((Ipv4Addr::LOCALHOST, port)).await?;
    let address: SocketAddr = listener.local_addr()?;
    let world = World::new(pool.clone());
    let app = server::app(world.clone(), address)?.merge(studio::app(world, pool));
    println!(
        "{}",
        serde_json::json!({"event": "server_ready", "address": address.to_string()})
    );
    std::io::stdout().flush()?;

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await?;
    Ok(())
}
