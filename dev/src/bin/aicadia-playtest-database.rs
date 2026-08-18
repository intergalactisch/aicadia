use sqlx::{Connection, Executor, PgConnection};

const DATABASE_PREFIX: &str = "aicadia_playtest_";
const OWNERSHIP_PREFIX: &str = "aicadia-playtest-owner:";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let argument = std::env::args().skip(1).collect::<Vec<_>>();
    let database_url = std::env::var("DATABASE_URL")?;
    let mut connection = PgConnection::connect(&database_url).await?;

    match argument.as_slice() {
        [command, ownership_token] if command == "probe" => {
            validate_ownership_token(ownership_token)?;
            let database_name = format!("{DATABASE_PREFIX}probe_{}", &ownership_token[..32]);
            create_owned_database(&mut connection, &database_name, ownership_token).await?;
            drop_owned_database(&mut connection, &database_name, ownership_token).await?;
            println!("ownership_probe_passed {database_name}");
        }
        [command, database_name, ownership_token] if command == "create" => {
            create_owned_database(&mut connection, database_name, ownership_token).await?;
            println!("ownership_verified {database_name} {ownership_token}");
        }
        [command, database_name, ownership_token] if command == "verify" => {
            require_database_ownership(&mut connection, database_name, ownership_token).await?;
            println!("ownership_verified {database_name} {ownership_token}");
        }
        [command, database_name, ownership_token] if command == "drop" => {
            drop_owned_database(&mut connection, database_name, ownership_token).await?;
            println!("ownership_verified_and_dropped {database_name}");
        }
        _ => {
            return Err(
                "usage: aicadia-playtest-database probe <ownership-token> | create|verify|drop <database> <ownership-token>"
                    .into(),
            );
        }
    }

    Ok(())
}

async fn create_owned_database(
    connection: &mut PgConnection,
    database_name: &str,
    ownership_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    validate_database_name(database_name)?;
    validate_ownership_token(ownership_token)?;

    connection
        .execute(format!("CREATE DATABASE \"{database_name}\"").as_str())
        .await?;

    let ownership_comment = ownership_comment(ownership_token);
    if let Err(error) = connection
        .execute(
            format!("COMMENT ON DATABASE \"{database_name}\" IS '{ownership_comment}'").as_str(),
        )
        .await
    {
        return Err(format!(
            "database {database_name} was created but ownership tagging failed; automatic cleanup is forbidden and manual inspection is required: {error}"
        )
        .into());
    }

    require_database_ownership(connection, database_name, ownership_token).await
}

async fn require_database_ownership(
    connection: &mut PgConnection,
    database_name: &str,
    ownership_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    validate_database_name(database_name)?;
    validate_ownership_token(ownership_token)?;
    let actual_comment: Option<String> = sqlx::query_scalar(
        "SELECT shobj_description(oid, 'pg_database') FROM pg_database WHERE datname = $1",
    )
    .bind(database_name)
    .fetch_optional(connection)
    .await?
    .flatten();
    let expected_comment = ownership_comment(ownership_token);

    if actual_comment.as_deref() != Some(expected_comment.as_str()) {
        return Err(format!(
            "database {database_name} does not carry the expected playtest ownership token; automatic cleanup is forbidden"
        )
        .into());
    }
    Ok(())
}

async fn drop_owned_database(
    connection: &mut PgConnection,
    database_name: &str,
    ownership_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    require_database_ownership(connection, database_name, ownership_token).await?;
    sqlx::query(
        "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = $1 AND pid <> pg_backend_pid()",
    )
    .bind(database_name)
    .execute(&mut *connection)
    .await?;
    connection
        .execute(format!("DROP DATABASE \"{database_name}\"").as_str())
        .await?;
    Ok(())
}

fn ownership_comment(ownership_token: &str) -> String {
    format!("{OWNERSHIP_PREFIX}{ownership_token}")
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

fn validate_ownership_token(value: &str) -> Result<(), &'static str> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err("invalid disposable database ownership token");
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::*;

    static DATABASE_SEQUENCE: AtomicU64 = AtomicU64::new(1);
    const TOKEN_A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const TOKEN_B: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

    fn test_database_name() -> String {
        format!(
            "{DATABASE_PREFIX}helper_{}_{}",
            std::process::id(),
            DATABASE_SEQUENCE.fetch_add(1, Ordering::Relaxed)
        )
    }

    #[test]
    fn names_and_unguessable_tokens_are_strict() {
        assert_eq!(
            validate_database_name("aicadia_playtest_20260808_123"),
            Ok(())
        );
        assert!(validate_database_name("postgres").is_err());
        assert!(validate_database_name("aicadia_playtest_bad-name").is_err());
        assert!(validate_database_name("aicadia_playtest_\";drop database postgres").is_err());
        assert_eq!(validate_ownership_token(TOKEN_A), Ok(()));
        assert!(validate_ownership_token("short").is_err());
        assert!(
            validate_ownership_token(
                "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
            )
            .is_err()
        );
        assert!(
            validate_ownership_token(
                "gggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg"
            )
            .is_err()
        );
    }

    #[tokio::test]
    async fn existing_prefixed_database_and_token_mismatch_never_drop() {
        let Ok(database_url) = std::env::var("DATABASE_URL") else {
            return;
        };
        let database_name = test_database_name();
        let mut connection = PgConnection::connect(&database_url).await.unwrap();
        create_owned_database(&mut connection, &database_name, TOKEN_A)
            .await
            .unwrap();

        let collision = create_owned_database(&mut connection, &database_name, TOKEN_B).await;
        assert!(collision.is_err());
        let wrong_drop = drop_owned_database(&mut connection, &database_name, TOKEN_B).await;
        assert!(wrong_drop.is_err());
        require_database_ownership(&mut connection, &database_name, TOKEN_A)
            .await
            .unwrap();

        drop_owned_database(&mut connection, &database_name, TOKEN_A)
            .await
            .unwrap();
        let still_exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_database WHERE datname = $1)")
                .bind(&database_name)
                .fetch_one(&mut connection)
                .await
                .unwrap();
        assert!(!still_exists);
    }
}
