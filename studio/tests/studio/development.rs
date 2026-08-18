//! Contract tests for the source-backed Development section.

use aicadia::World;
use aicadia_studio::{self as studio, plan, record::Repository, register};
use reqwest::{Client, StatusCode, redirect::Policy};
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

    async fn html(&self, path: &str) -> String {
        let response = self
            .client
            .get(format!("{}{path}", self.base_url))
            .send()
            .await
            .unwrap_or_else(|error| panic!("Studio request for {path} should send: {error}"));
        assert_eq!(response.status(), StatusCode::OK, "{path} should render");
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

fn repository() -> Repository {
    Repository::load(aicadia_studio::workspace_root())
        .expect("the governed repository should parse")
}

fn attribute(html: &str, name: &str) -> usize {
    let prefix = format!("{name}=\"");
    let start = html
        .find(&prefix)
        .unwrap_or_else(|| panic!("HTML should carry {name}"))
        + prefix.len();
    let end = html[start..]
        .find('"')
        .map(|offset| start + offset)
        .expect("attribute should close");
    html[start..end]
        .parse()
        .unwrap_or_else(|error| panic!("{name} should be numeric: {error}"))
}

#[sqlx::test(migrations = "../game/migration")]
async fn every_development_destination_is_a_complete_source_backed_page(pool: PgPool) {
    let server = StudioServer::start(pool).await;

    for (path, title) in [
        ("/development", "Development"),
        ("/development/direction", "Direction"),
        ("/development/decision", "Decision register"),
        ("/development/open", "Open questions"),
        ("/development/research", "Research"),
        ("/development/work", "Current edge and work"),
        ("/development/lab", "Lab"),
        ("/development/evidence", "Evidence"),
        ("/development/rules", "Build rules"),
    ] {
        let html = server.html(path).await;
        assert!(
            html.contains(&format!("<h1>{title}</h1>")),
            "{path} should render its T6 page"
        );
        assert!(html.contains("projection of governed development records"));
        assert!(
            !html.contains("Page pending"),
            "{path} must not fall through to a stub"
        );
    }
}

#[sqlx::test(migrations = "../game/migration")]
async fn register_open_plan_and_lab_counts_equal_the_shared_projection(pool: PgPool) {
    let repository = repository();
    let entry_count = repository
        .in_home("decision-register")
        .into_iter()
        .map(register::parse)
        .map(|register| register.entry.len())
        .sum::<usize>();
    let open_count = plan::open_section(&repository).len();
    let live_task_count = plan::plans(&repository)
        .iter()
        .filter(|plan| plan.is_live())
        .map(|plan| plan.task.len())
        .sum::<usize>();
    let experiment_count = repository.in_home("lab-experiment").len();

    let server = StudioServer::start(pool).await;
    let decisions = server.html("/development/decision").await;
    let open = server.html("/development/open").await;
    let work = server.html("/development/work").await;
    let lab = server.html("/development/lab").await;

    assert_eq!(attribute(&decisions, "data-entry-count"), entry_count);
    assert_eq!(attribute(&open, "data-open-section-count"), open_count);
    assert_eq!(attribute(&work, "data-plan-task-count"), live_task_count);
    assert_eq!(attribute(&lab, "data-experiment-count"), experiment_count);
}

#[sqlx::test(migrations = "../game/migration")]
async fn decision_facets_filter_loaded_entries_and_keep_stable_deep_links(pool: PgPool) {
    let repository = repository();
    let registers = repository
        .in_home("decision-register")
        .into_iter()
        .map(register::parse)
        .collect::<Vec<_>>();
    let accepted = registers
        .iter()
        .flat_map(|register| &register.entry)
        .filter(|entry| entry.tag == "accepted")
        .count();
    let first = registers
        .iter()
        .flat_map(|register| &register.entry)
        .next()
        .expect("the decision register has an entry");
    let expected_id = format!(
        "decision-{}-{}-{}",
        first.date, first.topic_id, first.ordinal
    );

    let server = StudioServer::start(pool).await;
    let filtered = server.html("/development/decision?tag=accepted").await;
    let all = server.html("/development/decision").await;

    assert_eq!(attribute(&filtered, "data-filtered-entry-count"), accepted);
    assert!(filtered.contains("aria-pressed=\"true\""));
    assert!(all.contains(&format!("id=\"{expected_id}\"")));
    assert!(all.contains(&format!("href=\"#{expected_id}\"")));
}

#[sqlx::test(migrations = "../game/migration")]
async fn direction_research_evidence_and_rules_counts_equal_their_sources(pool: PgPool) {
    let repository = repository();
    let direction_count = repository.in_home("concept-record").len();
    let research_count = repository.in_home("research-report").len();
    let evidence_count =
        repository.in_home("evidence-slice").len() + repository.in_home("runner-contract").len();
    let agents = repository.get("AGENTS.md").expect("AGENTS.md exists");
    let heuristic_count = agents
        .heading
        .iter()
        .skip_while(|heading| heading.title != "Build Heuristics")
        .skip(1)
        .take_while(|heading| heading.level != 2)
        .filter(|heading| heading.level == 3)
        .count();

    let server = StudioServer::start(pool).await;
    let direction = server.html("/development/direction").await;
    let research = server.html("/development/research").await;
    let evidence = server.html("/development/evidence").await;
    let rules = server.html("/development/rules").await;

    assert_eq!(
        attribute(&direction, "data-direction-record-count"),
        direction_count
    );
    assert_eq!(attribute(&research, "data-record-count"), research_count);
    assert_eq!(attribute(&evidence, "data-record-count"), evidence_count);
    assert_eq!(attribute(&rules, "data-heuristic-count"), heuristic_count);
}
