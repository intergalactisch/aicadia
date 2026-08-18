//! Game-section route tests over the real repository, compiled catalog and a
//! disposable migrated PostgreSQL database.

use aicadia::World;
use aicadia_studio::{self as studio, model, record::Repository};
use reqwest::{Client, StatusCode};
use sqlx::PgPool;
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
        let address = listener.local_addr().expect("listener has an address");
        let router = studio::app(World::new(pool.clone()), pool);
        let task = tokio::spawn(async move {
            axum::serve(listener, router)
                .await
                .expect("Studio should serve");
        });
        Self {
            base_url: format!("http://{address}"),
            client: Client::new(),
            task,
        }
    }

    async fn html(&self, path: &str) -> String {
        let response = self
            .client
            .get(format!("{}{path}", self.base_url))
            .send()
            .await
            .unwrap_or_else(|error| panic!("{path} should send: {error}"));
        assert_eq!(response.status(), StatusCode::OK, "{path} should render");
        response.text().await.expect("page should be text")
    }
}

impl Drop for StudioServer {
    fn drop(&mut self) {
        self.task.abort();
    }
}

fn occurrences(text: &str, needle: &str) -> usize {
    text.match_indices(needle).count()
}

fn repository() -> Repository {
    Repository::load(aicadia_studio::workspace_root()).expect("the real repository should project")
}

fn stem(path: &str) -> &str {
    path.rsplit('/')
        .next()
        .unwrap_or(path)
        .trim_end_matches(".md")
}

#[sqlx::test(migrations = "../game/migration")]
async fn game_counts_equal_the_discovered_model_capability_and_tool_counts(pool: PgPool) {
    let server = StudioServer::start(pool).await;
    let html = server.html("/game").await;

    let repository = repository();
    let model_count = model::models(&repository).len();
    let capability_count = repository.in_home("capability").len();

    assert_eq!(occurrences(&html, "data-model="), model_count);
    assert_eq!(occurrences(&html, "data-tool="), capability_count);
    assert!(html.contains(&format!("{model_count} models")));
    assert!(html.contains(&format!("{capability_count} published tools")));
}

#[sqlx::test(migrations = "../game/migration")]
async fn every_model_has_a_stable_page_with_its_claimed_database_columns(pool: PgPool) {
    let server = StudioServer::start(pool).await;
    for item in model::models(&repository()) {
        let id = &item.id;
        let html = server.html(&format!("/game/model/{id}")).await;
        assert!(html.contains("Contract"), "{id} should show its contract");
        assert!(
            html.contains("Canonical vocabulary"),
            "{id} should join vocabulary"
        );
        if item.storage_table.is_empty() {
            assert!(html.contains("claims no PostgreSQL table"));
        } else {
            assert!(
                html.contains("data-column="),
                "{id} should show live columns"
            );
        }
        assert!(html.contains("Copy reference"));
        assert!(html.contains(&format!("href=\"/game/model/{id}\" aria-current=\"page\"")));
        assert!(html.contains("<details open>"));
    }
}

#[sqlx::test(migrations = "../game/migration")]
async fn all_capabilities_and_tools_are_separate_cross_linked_pages(pool: PgPool) {
    let server = StudioServer::start(pool).await;
    let repository = repository();
    let instructions = repository.in_home("agent-instruction").len();
    let capabilities = repository.in_home("capability");
    let agent = server.html("/game/agent").await;
    assert_eq!(occurrences(&agent, "data-instruction="), instructions);
    assert_eq!(occurrences(&agent, "data-tool="), capabilities.len());
    assert!(agent.contains("Assembled instructions"));
    assert!(agent.contains("Host requirements"));
    assert!(agent.contains("Local adapter command shape"));
    assert!(agent.contains("href=\"/game/agent#tool-catalog\" aria-current=\"page\""));
    assert!(!agent.contains("<span class=\"seal seal-plain\">Published</span>"));

    for record in capabilities {
        let name = stem(&record.path);
        let capability = server.html(&format!("/game/capability/{name}")).await;
        assert!(capability.contains("Published tool description"));
        assert!(capability.contains("Input schema"));
        assert!(capability.contains("Annotations"));
        assert!(capability.contains(&format!("href=\"/game/tool/{name}\"")));
        assert!(capability.contains(&format!(
            "href=\"/game/capability/{name}\" aria-current=\"page\""
        )));
        assert!(!capability.contains("<span class=\"seal seal-plain\">Published</span>"));

        let tool = server.html(&format!("/game/tool/{name}")).await;
        assert!(tool.contains("Complete compiled entry"));
        assert!(tool.contains(&format!("href=\"/game/capability/{name}\"")));
        assert!(tool.contains(&format!("<h1>{name}</h1>")));
        assert!(!tool.contains(&format!("<h1>`{name}`</h1>")));
        assert!(tool.contains(&format!(
            "href=\"/game/capability/{name}\" aria-current=\"page\""
        )));
        assert!(!tool.contains("<span class=\"seal seal-plain\">Published</span>"));
    }
}

#[sqlx::test(migrations = "../game/migration")]
async fn vocabulary_storage_and_deferred_are_projected_from_their_authorities(pool: PgPool) {
    let server = StudioServer::start(pool).await;

    let repository = repository();
    let term_count = model::vocabulary(&repository).len();
    let models = model::models(&repository);
    let table_count = model::storage_table(&repository).len();
    let migration_count = repository.in_home("migration").len();

    let vocabulary = server.html("/game/vocabulary").await;
    assert_eq!(occurrences(&vocabulary, "data-term="), term_count);
    for item in models.iter().filter(|item| item.term.is_some()) {
        assert!(vocabulary.contains(&format!("href=\"/game/model/{}\"", item.id)));
    }
    assert!(vocabulary.contains("dev/CONTEXT.md"));

    let storage = server.html("/game/storage").await;
    assert_eq!(occurrences(&storage, "data-storage-table="), table_count);
    assert_eq!(occurrences(&storage, "data-migration="), migration_count);
    assert!(storage.contains("game/docs/storage.md"));

    let deferred = server.html("/game/deferred").await;
    assert!(deferred.contains("Negative current scope"));
    assert!(deferred.contains("Authentication"));
    assert!(deferred.contains("game/docs/deferred.md"));
}

#[sqlx::test(migrations = "../game/migration")]
async fn unknown_game_resources_are_honest_not_found_pages(pool: PgPool) {
    let server = StudioServer::start(pool).await;
    let response = server
        .client
        .get(format!("{}/game/model/nope", server.base_url))
        .send()
        .await
        .expect("request should send");
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    assert!(
        response
            .text()
            .await
            .expect("body")
            .contains("No projected model")
    );
}
