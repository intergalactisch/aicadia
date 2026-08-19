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

fn bullet_count(record: &aicadia_studio::record::Record, heading: &str) -> usize {
    plan::section(&record.body, heading, 3)
        .or_else(|| plan::section(&record.body, heading, 2))
        .unwrap_or_else(|| panic!("{} should contain {heading}", record.path))
        .lines()
        .filter(|line| line.trim_start().starts_with("- "))
        .count()
}

#[sqlx::test(migrations = "../game/migration")]
async fn every_development_destination_is_a_complete_source_backed_page(pool: PgPool) {
    let server = StudioServer::start(pool).await;

    for (path, title) in [
        ("/dev", "Development"),
        ("/dev/areas", "Development Areas"),
        ("/dev/areas/multiplayer", "Multiplayer"),
        ("/dev/areas/multiplayer/scenarios", "Multiplayer scenarios"),
        ("/dev/areas/place", "Place"),
        ("/dev/areas/movement", "Movement"),
        ("/dev/areas/discovery", "Discovery"),
        ("/dev/areas/agent-play", "Agent Play"),
        ("/dev/areas/world-change", "World Change"),
        ("/dev/direction", "Direction"),
        ("/dev/decision", "Decision register"),
        ("/dev/open", "Open landscape"),
        ("/dev/research", "Research"),
        ("/dev/work", "Current edge and work"),
        ("/dev/lab", "Lab"),
        ("/dev/evidence", "Evidence"),
        ("/dev/rules", "Build rules"),
    ] {
        let html = server.html(path).await;
        assert!(
            html.contains(&format!("<h1>{title}</h1>")),
            "{path} should render its T6 page"
        );
        assert!(html.contains("Reference ·"));
        assert!(
            !html.contains("Page pending"),
            "{path} must not fall through to a stub"
        );
    }
}

#[sqlx::test(migrations = "../game/migration")]
async fn areas_project_their_sources_and_the_superseded_route_family_is_absent(pool: PgPool) {
    let repository = repository();
    let area = repository.in_home("development-area");
    let area_count = area.len();
    let area_record_count = repository
        .in_home("area-record")
        .into_iter()
        .filter(|record| record.path.starts_with("dev/areas/multiplayer/"))
        .count();
    let scenario_count = repository
        .get("dev/areas/multiplayer/scenarios.md")
        .expect("the Multiplayer scenario catalogue exists")
        .heading
        .iter()
        .filter(|heading| {
            heading.level == 2
                && heading.title.len() > 4
                && heading.title.starts_with('S')
                && heading.title.as_bytes()[1..3]
                    .iter()
                    .all(u8::is_ascii_digit)
        })
        .count();

    let server = StudioServer::start(pool).await;
    let areas = server.html("/dev/areas").await;
    let multiplayer = server.html("/dev/areas/multiplayer").await;
    let scenarios = server.html("/dev/areas/multiplayer/scenarios").await;
    let work = server.html("/dev/work").await;

    assert_eq!(attribute(&areas, "data-area-count"), area_count);
    assert_eq!(area_count, 6);
    for record in area {
        let id = record
            .path
            .strip_prefix("dev/areas/")
            .and_then(|path| path.strip_suffix("/README.md"))
            .expect("Area path should follow its convention");
        let detail = server.html(&format!("/dev/areas/{id}")).await;
        assert!(detail.contains("<h2>Boundary</h2>"), "{id}");
        assert!(detail.contains("<h3>This is</h3>"), "{id}");
        assert!(detail.contains("<h3>This is not</h3>"), "{id}");
        assert!(detail.contains("Decisions and open landscape"), "{id}");
        assert!(detail.contains("Components and concepts"), "{id}");
        assert!(detail.contains("Technical model"), "{id}");
        assert_eq!(
            attribute(&detail, "data-chosen-count"),
            bullet_count(record, "Chosen"),
            "{id} chosen parity"
        );
        assert_eq!(
            attribute(&detail, "data-rejected-count"),
            bullet_count(record, "Rejected"),
            "{id} rejected parity"
        );
        assert_eq!(
            attribute(&detail, "data-not-yet-chosen-count"),
            bullet_count(record, "Not yet chosen"),
            "{id} unresolved parity"
        );
        assert_eq!(
            attribute(&detail, "data-research-needed-count"),
            bullet_count(record, "Research needed"),
            "{id} research parity"
        );
        assert!(!detail.contains("Live plan boards"), "{id}");
    }
    assert_eq!(
        attribute(&multiplayer, "data-area-record-count"),
        area_record_count
    );
    assert_eq!(attribute(&scenarios, "data-scenario-count"), scenario_count);
    assert_eq!(scenario_count, 14);
    for number in 1..=14 {
        assert_eq!(
            scenarios.matches(&format!(">S{number:02}</span>")).count(),
            1,
            "scenario S{number:02} should render once"
        );
    }
    assert!(scenarios.contains("Development pressure, not current game contract"));
    assert!(!multiplayer.contains("Live plan boards"));
    assert!(!scenarios.contains("Live plan boards"));
    assert!(work.contains("Live plan boards"));

    for path in [
        "/development",
        "/development/direction",
        "/development/decision",
        "/development/open",
        "/development/research",
        "/development/work",
        "/development/lab",
        "/development/evidence",
        "/development/rules",
    ] {
        let response = server
            .client
            .get(format!("{}{path}", server.base_url))
            .send()
            .await
            .unwrap_or_else(|error| panic!("old route {path} should send: {error}"));
        assert_eq!(response.status(), StatusCode::NOT_FOUND, "{path}");
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
    let decisions = server.html("/dev/decision").await;
    let open = server.html("/dev/open").await;
    let work = server.html("/dev/work").await;
    let lab = server.html("/dev/lab").await;

    assert_eq!(attribute(&decisions, "data-entry-count"), entry_count);
    assert_eq!(attribute(&open, "data-open-section-count"), open_count);
    assert!(open.contains("Not yet chosen"));
    assert!(open.contains("Research needed"));
    assert!(open.contains("/doc/dev/areas/place/README.md#not-yet-chosen"));
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
    let filtered = server.html("/dev/decision?tag=accepted").await;
    let all = server.html("/dev/decision").await;

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
    let direction = server.html("/dev/direction").await;
    let research = server.html("/dev/research").await;
    let evidence = server.html("/dev/evidence").await;
    let rules = server.html("/dev/rules").await;

    assert_eq!(
        attribute(&direction, "data-direction-record-count"),
        direction_count
    );
    assert_eq!(attribute(&research, "data-record-count"), research_count);
    assert_eq!(attribute(&evidence, "data-record-count"), evidence_count);
    assert_eq!(attribute(&rules, "data-heuristic-count"), heuristic_count);
}
