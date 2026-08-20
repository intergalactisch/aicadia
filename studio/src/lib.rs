//! Read-only development projection for the local Aicadia Studio.

pub mod agent;
pub mod brief;
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

pub use aicadia::World;

const STUDIO_CSS: &str = include_str!("../web/studio.css");
const STUDIO_JS: &str = include_str!("../web/studio.js");

/// The repository root that owns the three Cargo workspace members.
pub fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("Studio is a direct workspace member")
        .to_path_buf()
}

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
        repository_root: Arc::new(workspace_root()),
    };

    Router::new()
        .route("/", get(page::overview))
        .route("/brief", get(page::brief))
        .route("/game", get(page::game))
        .route("/game/model/{id}", get(page::game_model))
        .route("/game/capability/{name}", get(page::game_capability))
        .route("/game/tool/{name}", get(page::game_tool))
        .route("/game/agent", get(page::game_agent))
        .route("/game/vocabulary", get(page::game_vocabulary))
        .route("/game/storage", get(page::game_storage))
        .route("/game/deferred", get(page::game_deferred))
        .route("/dev", get(page::development))
        .route("/dev/areas", get(page::development_areas))
        .route("/dev/areas/{area}", get(page::development_area))
        .route(
            "/dev/areas/{area}/scenarios",
            get(page::development_area_scenarios),
        )
        .route("/dev/direction", get(page::development_direction))
        .route("/dev/decision", get(page::development_decision))
        .route("/dev/open", get(page::development_open))
        .route("/dev/research", get(page::development_research))
        .route("/dev/work", get(page::development_work))
        .route("/dev/lab", get(page::development_lab))
        .route("/dev/evidence", get(page::development_evidence))
        .route("/dev/rules", get(page::development_rules))
        .route("/live", get(page::live))
        .route("/live/place", get(page::live_place))
        .route("/live/place/{id}", get(page::live_place_detail))
        .route("/live/character", get(page::live_character))
        .route("/live/character/{id}", get(page::live_character_detail))
        .route("/live/entity", get(page::live_entity))
        .route("/live/entity/{id}", get(page::live_entity_detail))
        .route(
            "/live/entity/{entity_id}/property/{property_key_id}",
            get(page::live_property_history),
        )
        .route("/live/activity", get(page::live_activity))
        .route("/live/activity/{id}", get(page::live_activity_detail))
        .route("/live/user", get(page::live_user))
        .route("/live/user/{id}", get(page::live_user_detail))
        .route("/live/property-key", get(page::live_property_key))
        .route(
            "/live/property-key/{key}",
            get(page::live_property_key_detail),
        )
        .route("/live/trait", get(page::live_trait))
        .route("/live/trait/{id}", get(page::live_trait_detail))
        .route("/live/investigation", get(page::live_investigation))
        .route(
            "/live/investigation/{id}",
            get(page::live_investigation_detail),
        )
        .route("/live/resolve", get(page::live_resolve))
        .route("/live/storage", get(page::live_storage))
        .route(
            "/live/storage/snapshot.json",
            get(page::live_storage_snapshot),
        )
        .route("/live/storage/{table}", get(page::live_storage_table))
        .route("/live/migration", get(page::live_migration))
        .route("/doc/{*path}", get(page::document))
        .route("/jump", get(page::jump))
        .route("/assets/studio.css", get(stylesheet))
        .route("/assets/studio.js", get(script))
        .with_state(state)
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
    UnpageableTable,
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
            Self::UnpageableTable => (StatusCode::CONFLICT, "table rows require a primary key"),
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
        for forbidden in [
            "fetch(",
            "XMLHttpRequest",
            concat!("/studio", "/api/"),
            "/api/",
        ] {
            assert!(
                !STUDIO_JS.contains(forbidden),
                "the enhancement script must not carry {forbidden}"
            );
        }
    }

    fn repository() -> record::Repository {
        record::Repository::load(workspace_root())
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
        assert!(repository.get("dev/docs/README.md").is_some());
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
                .get("dev/docs/concept/log/2026-08.md")
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
            aicadia::server::mcp_tool_catalog(World::new(pool)),
        );

        assert_eq!(surface.section.len(), 15);
        assert_eq!(surface.tool.len(), 19);
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
