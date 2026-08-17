//! Read-only development projection for the local Aicadia Studio.

pub mod agent;
pub mod home;
pub mod model;
pub mod plan;
pub mod record;
pub mod register;

pub mod live;
mod page;

use std::{path::PathBuf, sync::Arc};

use axum::{
    Json, Router,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
    routing::get,
};
use serde::Serialize;
use sqlx::PgPool;

use crate::World;

const STUDIO_CSS: &str = include_str!("../../web/studio.css");
const STUDIO_JS: &str = include_str!("../../web/studio.js");

#[derive(Clone)]
struct StudioState {
    world: World,
    pool: PgPool,
    repository_root: Arc<PathBuf>,
}

pub fn app(world: World, pool: PgPool) -> Router {
    let state = StudioState {
        world,
        pool,
        repository_root: Arc::new(PathBuf::from(env!("CARGO_MANIFEST_DIR"))),
    };

    let mut router = Router::new()
        .route("/", get(page::overview))
        .route("/game", get(page::game))
        .route("/development", get(page::development))
        .route("/live", get(page::live))
        .route("/doc/{*path}", get(page::document))
        .route("/jump", get(page::jump))
        .route("/live/storage/{table}", get(page::stub_table))
        .route("/assets/studio.css", get(stylesheet))
        .route("/assets/studio.js", get(script))
        .route("/studio/api/live/entity/{entity_id}", get(live::get_entity))
        .route(
            "/studio/api/live/activity/{activity_id}",
            get(live::get_activity),
        )
        .route("/studio/api/live/character", get(live::list_character))
        .route("/studio/api/live/place", get(live::list_place))
        .route("/studio/api/live/storage", get(live::get_storage))
        .route(
            "/studio/api/live/storage/snapshot",
            get(live::download_storage),
        );
    for path in page::stub_path() {
        router = router.route(path, get(page::stub));
    }

    router.with_state(state)
}

async fn stylesheet() -> impl IntoResponse {
    (
        [
            (header::CONTENT_TYPE, "text/css; charset=utf-8"),
            (header::CACHE_CONTROL, "no-cache"),
        ],
        STUDIO_CSS,
    )
}

async fn script() -> impl IntoResponse {
    (
        [
            (header::CONTENT_TYPE, "text/javascript; charset=utf-8"),
            (header::CACHE_CONTROL, "no-cache"),
        ],
        STUDIO_JS,
    )
}

/// Every way a Studio read can fail. Public because `live` is a public module
/// whose reads return it.
#[derive(Debug)]
pub enum StudioError {
    Database(sqlx::Error),
    InvalidLimit,
    NotFound,
    Source(std::io::Error),
    SourceTooLarge(&'static str),
    SchemaTooLarge(&'static str),
}

impl From<sqlx::Error> for StudioError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}

impl From<std::io::Error> for StudioError {
    fn from(error: std::io::Error) -> Self {
        Self::Source(error)
    }
}

#[derive(Serialize)]
struct StudioErrorBody {
    error: &'static str,
}

impl IntoResponse for StudioError {
    fn into_response(self) -> Response {
        let (status, error) = match self {
            Self::InvalidLimit => (StatusCode::BAD_REQUEST, "limit must be between 1 and 100"),
            Self::NotFound => (StatusCode::NOT_FOUND, "World record was not found"),
            Self::Database(database_error) => {
                eprintln!("Studio database read failed: {database_error}");
                (
                    StatusCode::SERVICE_UNAVAILABLE,
                    "live World data is unavailable",
                )
            }
            Self::Source(source_error) => {
                eprintln!("Studio source read failed: {source_error}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "repository source is unavailable",
                )
            }
            Self::SourceTooLarge(path) => {
                eprintln!("Studio source exceeds its read bound: {path}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "repository source exceeds its read bound",
                )
            }
            Self::SchemaTooLarge(subject) => {
                eprintln!("Studio schema projection exceeds its {subject} bound");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "database schema exceeds its Studio read bound",
                )
            }
        };
        (status, Json(StudioErrorBody { error })).into_response()
    }
}

/// One empty directory for a test that needs a repository on disk.
#[cfg(test)]
pub(crate) fn test_directory(name: &str) -> PathBuf {
    use std::sync::atomic::{AtomicUsize, Ordering};

    static SEQUENCE: AtomicUsize = AtomicUsize::new(0);
    let directory = std::env::temp_dir().join(format!(
        "aicadia-studio-{name}-{}-{}",
        std::process::id(),
        SEQUENCE.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).expect("the test directory is created");
    directory
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_browser_assets_carry_presentation_only() {
        assert!(STUDIO_CSS.contains("--compass"));
        assert!(STUDIO_JS.contains("data-copy"));
        for forbidden in ["fetch(", "XMLHttpRequest", "/studio/api/", "/api/"] {
            assert!(
                !STUDIO_JS.contains(forbidden),
                "the enhancement script must not carry {forbidden}"
            );
        }
    }

    fn repository() -> record::Repository {
        record::Repository::load(env!("CARGO_MANIFEST_DIR"))
            .expect("the governed roots parse within their read bounds")
    }

    #[test]
    fn the_real_repository_projects_every_governed_record_and_lints_without_panicking() {
        let repository = repository();

        assert!(
            repository.record().len() > 150,
            "only {} governed records were found",
            repository.record().len()
        );
        assert!(
            repository
                .record()
                .iter()
                .all(|record| !record.title.is_empty())
        );
        assert!(repository.get("AGENTS.md").is_some());
        assert!(repository.get("docs/README.md").is_some());
        assert!(!repository.in_home("capability").is_empty());
        assert!(!repository.in_home("migration").is_empty());

        let finding = repository.lint();
        assert!(
            finding
                .iter()
                .all(|finding| !finding.path.is_empty() && !finding.rule.is_empty()),
            "every finding names its file and rule"
        );
    }

    #[test]
    fn the_real_august_register_plans_backlog_and_open_sections_parse() {
        let repository = repository();
        let august = register::parse(
            repository
                .get("docs/concept/log/2026-08.md")
                .expect("the August register exists"),
        );

        assert!(
            august.entry.len() > 300,
            "only {} August entries were parsed",
            august.entry.len()
        );
        assert!(august.date().len() > 5);
        assert!(
            august
                .entry
                .iter()
                .filter(|entry| entry.tag == register::UNKNOWN_TAG)
                .count()
                * 10
                < august.entry.len(),
            "too many August entries lost their tag"
        );
        assert!(august.entry.iter().all(|entry| !entry.anchor.is_empty()));

        let plan = plan::plans(&repository);
        assert!(plan.iter().any(|plan| plan.is_live()));
        assert!(plan.iter().any(|plan| !plan.task.is_empty()));
        assert!(!plan::horizon(&repository).is_empty());
        assert!(!plan::open_section(&repository).is_empty());
    }

    #[tokio::test]
    async fn the_real_model_and_agent_joins_resolve_against_compiled_sources() {
        let repository = repository();
        let vocabulary = model::vocabulary(&repository);
        let table = model::storage_table(&repository);

        assert!(vocabulary.iter().any(|term| term.id == "world"));
        assert!(
            table.iter().any(
                |table| table.name == "user" && table.created_as.as_deref() == Some("app_user")
            )
        );
        assert!(table.iter().all(|table| table.name != "_sqlx_migrations"));
        for model in model::models(&repository) {
            assert!(!model.title.is_empty());
            assert!(model::contract(repository.root(), &model.id).is_some());
        }

        let pool =
            PgPool::connect_lazy("postgresql:///unused").expect("a lazy pool never connects");
        let surface = agent::surface(
            &repository,
            crate::server::mcp_tool_catalog(World::new(pool)),
        );

        assert_eq!(surface.section.len(), 16);
        assert_eq!(surface.tool.len(), 15);
        for tool in &surface.tool {
            assert!(
                tool.capability_exists,
                "{} has no capability document",
                tool.name
            );
            assert!(tool.text_exists, "{} has no published tool text", tool.name);
            assert!(
                tool.route().is_some(),
                "{} has no HTTP operation",
                tool.name
            );
        }
    }
}
