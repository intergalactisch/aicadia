//! Server-rendered Studio pages over the real repository and a disposable World.
//!
//! Every assertion here is about navigation and projection: a governed record is
//! reachable, its own text is on the page, the tree links resolve and the
//! resolver lands on the record it names. Page content itself is proved by the
//! projection tests in `studio/src/`.

use aicadia::World;
use aicadia_studio as studio;
use reqwest::{Client, StatusCode, redirect::Policy};
use sqlx::PgPool;
use std::collections::HashMap;
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

    async fn html(&self, path: &str) -> String {
        let response = self.get(path).await;
        assert_eq!(
            response.status(),
            StatusCode::OK,
            "{path} should render a page"
        );
        assert_eq!(
            response
                .headers()
                .get(reqwest::header::CONTENT_TYPE)
                .and_then(|value| value.to_str().ok()),
            Some("text/html; charset=utf-8"),
            "{path} should be HTML"
        );
        response
            .text()
            .await
            .unwrap_or_else(|error| panic!("{path} should be text: {error}"))
    }
}

impl Drop for StudioServer {
    fn drop(&mut self) {
        self.task.abort();
    }
}

/// Every `href="…"` inside the section tree of one rendered page, in order.
fn tree_href(html: &str) -> Vec<String> {
    let start = html
        .find("<nav class=\"tree\"")
        .expect("every page carries a section tree");
    let end = html[start..]
        .find("</nav>")
        .map(|offset| start + offset)
        .expect("the section tree is closed");
    let mut href = Vec::new();
    let mut rest = &html[start..end];
    while let Some(position) = rest.find("href=\"") {
        rest = &rest[position + 6..];
        let Some(quote) = rest.find('"') else { break };
        href.push(rest[..quote].to_owned());
        rest = &rest[quote..];
    }
    href
}

#[sqlx::test(migrations = "../game/migration")]
async fn the_overview_names_the_state_of_aicadia_and_every_section(pool: PgPool) {
    let server = StudioServer::start(pool).await;

    let html = server.html("/").await;

    assert!(html.contains("State of Aicadia"));
    assert!(html.contains("<title>State of Aicadia · Aicadia Studio</title>"));
    for section in ["/game", "/development", "/live"] {
        assert!(
            html.contains(&format!("href=\"{section}\"")),
            "the Overview should link to {section}"
        );
    }
    assert!(html.contains("aria-current=\"page\""));
    assert!(html.contains("id=\"content\""));
    assert!(html.contains("class=\"skip\""));
    assert!(html.contains("class=\"mark\" href=\"/\" aria-label=\"Homepage\""));
    assert!(html.contains("/assets/studio.css"));
}

#[sqlx::test(migrations = "../game/migration")]
async fn every_section_landing_renders_its_own_tree(pool: PgPool) {
    let server = StudioServer::start(pool).await;

    for (path, expected) in [
        ("/game", "Models"),
        ("/development", "Decisions"),
        ("/live", "Subjects"),
    ] {
        let html = server.html(path).await;
        assert!(
            html.contains(expected),
            "{path} should carry the {expected} group"
        );
        assert!(
            !tree_href(&html).is_empty(),
            "{path} should render a section tree"
        );
        assert!(html.contains("class=\"plate\""));
    }
}

#[sqlx::test(migrations = "../game/migration")]
async fn a_model_record_carries_its_authority_backlinks_and_heading_ids(pool: PgPool) {
    let server = StudioServer::start(pool).await;

    let html = server.html("/doc/game/docs/model/entity/README.md").await;

    assert!(html.contains("<h1>Entity</h1>"));
    assert!(html.contains("game/docs/model/entity/README.md"));
    assert!(html.contains("Authority:"));
    assert!(html.contains("Referenced by"));
    assert_eq!(
        html.matches("Authority:").count(),
        1,
        "authority belongs in one compact location"
    );
    assert!(
        !html.contains("Role header") && !html.contains("Front matter"),
        "source metadata should not be repeated in permanent side panels"
    );
    assert!(
        html.contains("href=\"/doc/"),
        "internal Markdown links should point at Studio routes"
    );
    assert!(html.contains("[Aicadia Studio · Entity](http://"));
}

#[sqlx::test(migrations = "../game/migration")]
async fn the_build_constitution_and_one_migration_render(pool: PgPool) {
    let server = StudioServer::start(pool).await;

    let constitution = server.html("/doc/AGENTS.md").await;
    assert!(constitution.contains("<h1>Aicadia</h1>"));
    assert!(constitution.contains("<h2 id=\"build-heuristics\">"));
    assert!(constitution.contains("class=\"anchor\""));
    assert!(constitution.contains("On this page"));

    let migration = server.html("/doc/game/migration/0001_world.sql").await;
    assert!(migration.contains("0001_world.sql"));
    assert!(migration.contains("<pre>"));
    assert!(migration.contains("CREATE TABLE"));
}

#[sqlx::test(migrations = "../game/migration")]
async fn an_unknown_record_path_is_an_honest_not_found_page(pool: PgPool) {
    let server = StudioServer::start(pool).await;

    let response = server.get("/doc/nope.md").await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let html = response.text().await.expect("the 404 page should be text");
    assert!(html.contains("No governed record"));
    assert!(html.contains("nope.md"));
    assert!(html.contains("class=\"bar\""), "the 404 stays in the shell");
}

#[sqlx::test(migrations = "../game/migration")]
async fn the_resolver_lands_on_a_named_resource_and_says_so_when_nothing_matches(pool: PgPool) {
    let server = StudioServer::start(pool).await;

    let entity = server.get("/jump?q=entity").await;
    assert_eq!(entity.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        entity
            .headers()
            .get(reqwest::header::LOCATION)
            .and_then(|value| value.to_str().ok()),
        Some("/game/model/entity")
    );

    let capability = server.get("/jump?q=create_entity").await;
    assert_eq!(capability.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        capability
            .headers()
            .get(reqwest::header::LOCATION)
            .and_then(|value| value.to_str().ok()),
        Some("/game/capability/create_entity")
    );

    let path = server.get("/jump?q=AGENTS.md").await;
    assert_eq!(path.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        path.headers()
            .get(reqwest::header::LOCATION)
            .and_then(|value| value.to_str().ok()),
        Some("/doc/AGENTS.md")
    );

    let nothing = server.html("/jump?q=zzz").await;
    assert!(nothing.contains("Nothing resolves"));
    assert!(nothing.contains("zzz"));
}

#[sqlx::test(migrations = "../game/migration")]
async fn the_browser_assets_are_served_with_their_own_content_type(pool: PgPool) {
    let server = StudioServer::start(pool).await;

    for (path, content_type, needle) in [
        ("/assets/studio.css", "text/css; charset=utf-8", "--compass"),
        (
            "/assets/studio.js",
            "text/javascript; charset=utf-8",
            "data-copy",
        ),
    ] {
        let response = server.get(path).await;
        assert_eq!(response.status(), StatusCode::OK, "{path} should be served");
        assert_eq!(
            response
                .headers()
                .get(reqwest::header::CONTENT_TYPE)
                .and_then(|value| value.to_str().ok()),
            Some(content_type)
        );
        assert_eq!(
            response
                .headers()
                .get(reqwest::header::CACHE_CONTROL)
                .and_then(|value| value.to_str().ok()),
            Some("no-cache")
        );
        let body = response.text().await.expect("the asset should be text");
        assert!(body.contains(needle), "{path} should carry {needle}");
    }
}

#[sqlx::test(migrations = "../game/migration")]
async fn every_link_of_every_section_tree_resolves(pool: PgPool) {
    let server = StudioServer::start(pool).await;

    let mut visited: HashMap<String, String> = HashMap::new();
    for section in ["/", "/game", "/development", "/live"] {
        let html = server.html(section).await;
        for href in tree_href(&html) {
            let (path, fragment) = href
                .split_once('#')
                .map_or((href.as_str(), None), |(path, fragment)| {
                    (path, Some(fragment))
                });
            if !visited.contains_key(path) {
                let response = server.get(path).await;
                assert_eq!(
                    response.status(),
                    StatusCode::OK,
                    "the {section} tree links to {path}, which does not render"
                );
                let body = response.text().await.expect("tree target should be text");
                visited.insert(path.to_owned(), body);
            }
            if let Some(fragment) = fragment {
                assert!(
                    visited[path].contains(&format!("id=\"{fragment}\"")),
                    "the {section} tree links to missing fragment #{fragment} on {path}"
                );
            }
        }
    }

    assert!(
        visited.len() > 100,
        "only {} tree links were crawled",
        visited.len()
    );
}

#[sqlx::test(migrations = "../game/migration")]
async fn mobile_navigation_is_usable_before_javascript_enhances_it(pool: PgPool) {
    let server = StudioServer::start(pool).await;
    let html = server.html("/game").await;
    let css = server
        .get("/assets/studio.css")
        .await
        .text()
        .await
        .expect("Studio CSS should be text");
    let javascript = server
        .get("/assets/studio.js")
        .await
        .text()
        .await
        .expect("Studio JavaScript should be text");

    assert!(html.contains("<nav class=\"tree\""));
    assert!(html.contains("class=\"tree-sections\""));
    assert!(css.contains(".tree { position: static; height: auto; width: auto;"));
    assert!(css.contains(".js .tree { position: fixed;"));
    assert!(css.contains(".js .menu-toggle { display: inline-flex; }"));
    assert!(javascript.contains("document.documentElement.classList.add(\"js\")"));
}

#[sqlx::test(migrations = "../game/migration")]
async fn a_live_collection_is_a_real_projection_instead_of_a_placeholder(pool: PgPool) {
    let server = StudioServer::start(pool).await;

    let html = server.html("/live/entity").await;

    assert!(!html.contains("Page pending"));
    assert!(html.contains("Entities"));
    assert!(html.contains("loaded rows"));
}

#[tokio::test]
async fn a_record_page_still_renders_when_the_database_is_unreachable() {
    // A lazily connected pool to a port nothing listens on: the repository
    // projection is independent of the World, so only the pulse may fail.
    let pool = PgPool::connect_lazy("postgres://127.0.0.1:1/aicadia_absent")
        .expect("a lazy pool never connects");
    let server = StudioServer::start(pool).await;

    let record = server.html("/doc/AGENTS.md").await;
    assert!(record.contains("<h1>Aicadia</h1>"));
    assert!(record.contains("data-state=\"unavailable\""));
    assert!(record.contains("unavailable"));

    let overview = server.html("/").await;
    assert!(overview.contains("State of Aicadia"));
    assert!(overview.contains("The live World is unavailable"));

    let live = server.html("/live").await;
    assert!(live.contains("<h1>Live</h1>"));
}
