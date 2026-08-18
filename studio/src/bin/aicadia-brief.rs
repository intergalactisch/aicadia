use std::env;

use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    let pool = match env::var("DATABASE_URL") {
        Ok(database_url) => match PgPoolOptions::new().connect_lazy(&database_url) {
            Ok(pool) => Some(pool),
            Err(error) => {
                eprintln!("aicadia-brief: invalid DATABASE_URL: {error}");
                std::process::exit(1);
            }
        },
        Err(env::VarError::NotPresent) => None,
        Err(error) => {
            eprintln!("aicadia-brief: could not read DATABASE_URL: {error}");
            std::process::exit(1);
        }
    };

    match aicadia_studio::brief::markdown(&aicadia_studio::workspace_root(), pool.as_ref()).await {
        Ok(markdown) => print!("{markdown}"),
        Err(error) => {
            eprintln!("aicadia-brief: {error}");
            std::process::exit(1);
        }
    }
}
