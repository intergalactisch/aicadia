use std::str::FromStr;

use aicadia::{World, wire::UserOutput};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args_os().len() != 1 {
        return Err("aicadia-provision-user takes no arguments".into());
    }

    let database_url = std::env::var("DATABASE_URL")?;
    let database_name = std::env::var("AICADIA_DATABASE_NAME").ok();
    let mut connect_options = PgConnectOptions::from_str(&database_url)?;
    if let Some(database_name) = database_name {
        connect_options = connect_options.database(&database_name);
    }
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect_with(connect_options)
        .await?;
    let user = World::new(pool).create_user().await?;

    println!("{}", serde_json::to_string(&UserOutput::from(user))?);
    Ok(())
}
