use sqlx::{Connection, Executor, PgConnection};

const DATABASE_PREFIX: &str = "aicadia_playtest_";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let argument = std::env::args().skip(1).collect::<Vec<_>>();
    let database_url = std::env::var("DATABASE_URL")?;
    let mut connection = PgConnection::connect(&database_url).await?;

    match argument.as_slice() {
        [command] if command == "probe" => {
            let may_create: bool = sqlx::query_scalar(
                "SELECT rolcreatedb OR rolsuper FROM pg_roles WHERE rolname = current_user",
            )
            .fetch_one(&mut connection)
            .await?;
            if !may_create {
                return Err("DATABASE_URL role may not create disposable databases".into());
            }
        }
        [command, database_name] if command == "create" => {
            validate_database_name(database_name)?;
            connection
                .execute(format!("CREATE DATABASE \"{database_name}\"").as_str())
                .await?;
        }
        [command, database_name] if command == "drop" => {
            validate_database_name(database_name)?;
            sqlx::query(
                "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = $1 AND pid <> pg_backend_pid()",
            )
            .bind(database_name)
            .execute(&mut connection)
            .await?;
            connection
                .execute(format!("DROP DATABASE IF EXISTS \"{database_name}\"").as_str())
                .await?;
        }
        _ => {
            return Err(
                "usage: aicadia-playtest-database probe | create <database> | drop <database>"
                    .into(),
            );
        }
    }

    Ok(())
}

fn validate_database_name(value: &str) -> Result<(), &'static str> {
    if value.len() > 63
        || !value.starts_with(DATABASE_PREFIX)
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
    {
        return Err("invalid disposable database name");
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn only_generated_playtest_database_names_are_accepted() {
        assert_eq!(
            validate_database_name("aicadia_playtest_20260808_123"),
            Ok(())
        );
        assert!(validate_database_name("postgres").is_err());
        assert!(validate_database_name("aicadia_playtest_bad-name").is_err());
        assert!(validate_database_name("aicadia_playtest_\";drop database postgres").is_err());
    }
}
