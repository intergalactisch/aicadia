//! Read-only development projection for the local Aicadia Studio.

mod catalog;
mod live;

use std::{path::PathBuf, sync::Arc};

use axum::{
    Json, Router,
    http::{StatusCode, header},
    response::{Html, IntoResponse, Response},
    routing::get,
};
use serde::Serialize;
use sqlx::PgPool;

use crate::World;

const STUDIO_HTML: &str = include_str!("../../web/index.html");
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

    Router::new()
        .route("/", get(index))
        .route("/assets/studio.css", get(stylesheet))
        .route("/assets/studio.js", get(script))
        .route("/studio/api/catalog", get(catalog::get_catalog))
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
        )
        .with_state(state)
}

async fn index() -> Html<&'static str> {
    Html(STUDIO_HTML)
}

async fn stylesheet() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/css; charset=utf-8")],
        STUDIO_CSS,
    )
}

async fn script() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/javascript; charset=utf-8")],
        STUDIO_JS,
    )
}

#[derive(Debug)]
enum StudioError {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn studio_root_is_the_existing_application_route() {
        let response = index().await.into_response();

        assert_eq!(response.status(), StatusCode::OK);
        assert!(STUDIO_HTML.contains("Aicadia Studio"));
        assert!(STUDIO_HTML.contains("data-app-shell"));
        assert_eq!(STUDIO_HTML.matches("data-section-link=").count(), 2);
        assert!(!STUDIO_HTML.contains("data-variant="));
        assert!(!STUDIO_HTML.contains("prototype-switcher"));
    }
}
