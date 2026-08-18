//! Overview hierarchy and shared builder-brief contract.

use std::process::Command;

use aicadia::World;
use aicadia_studio::{self as studio, record::Repository};
use reqwest::{Client, StatusCode, redirect::Policy};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::{net::TcpListener, task::JoinHandle};

struct StudioServer {
    base_url: String,
    client: Client,
    task: JoinHandle<()>,
}

impl StudioServer {
    async fn start(pool: PgPool) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("loopback listener should bind");
        let address = listener
            .local_addr()
            .expect("loopback listener should have an address");
        let router = studio::app(World::new(pool.clone()), pool);
        let task = tokio::spawn(async move {
            axum::serve(listener, router)
                .await
                .expect("Studio test server should run");
        });
        Self {
            base_url: format!("http://{address}"),
            client: Client::builder()
                .redirect(Policy::none())
                .build()
                .expect("test client should build"),
            task,
        }
    }

    async fn get(&self, path: &str) -> reqwest::Response {
        self.client
            .get(format!("{}{path}", self.base_url))
            .send()
            .await
            .unwrap_or_else(|error| panic!("Studio request for {path} should send: {error}"))
    }
}

impl Drop for StudioServer {
    fn drop(&mut self) {
        self.task.abort();
    }
}

#[sqlx::test(migrations = "../game/migration")]
async fn overview_prioritizes_current_work_and_clickable_integrity(pool: PgPool) {
    let server = StudioServer::start(pool).await;
    let response = server.get("/").await;
    assert_eq!(response.status(), StatusCode::OK);
    let html = response.text().await.expect("Overview should be HTML");

    assert!(html.contains("<h1>State of Aicadia</h1>"));
    assert!(html.contains("class=\"section overview-priority\""));
    assert!(html.contains("Copy builder brief"));
    assert!(html.contains("Open Markdown"));
    assert!(html.contains("Current work"));
    assert!(html.contains("Needs attention"));
    assert!(html.contains("System shape"));
    assert!(html.contains("secondary-disclosure"));
    assert!(!html.contains("complete state dashboard"));

    let repository = Repository::load(aicadia_studio::workspace_root())
        .expect("the governed repository should project");
    let lint = repository.lint();
    if lint.is_empty() {
        assert!(html.contains("No documentation lint findings."));
    } else {
        for finding in lint {
            assert!(
                html.contains(&format!("href=\"/doc/{}\"", finding.path)),
                "lint finding {} should link to its source",
                finding.path
            );
        }
    }
}

#[tokio::test]
async fn brief_route_and_cli_are_byte_identical_without_a_database() {
    let pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_millis(25))
        .connect_lazy("postgresql://127.0.0.1:1/aicadia-brief-test")
        .expect("an unreachable lazy pool should construct");
    let server = StudioServer::start(pool).await;
    let response = server.get("/brief").await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok()),
        Some("text/markdown; charset=utf-8")
    );
    let route = response.text().await.expect("brief route should be text");

    let output = Command::new(env!("CARGO_BIN_EXE_aicadia-brief"))
        .env_remove("DATABASE_URL")
        .output()
        .expect("brief binary should run");
    assert!(
        output.status.success(),
        "brief binary failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let cli = String::from_utf8(output.stdout).expect("brief output should be UTF-8");

    assert_eq!(route, cli);
    assert!(cli.contains("Generated orientation only"));
    assert!(cli.contains("Live World\n\n- Unavailable"));
}
