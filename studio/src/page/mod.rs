//! Server-rendered Studio pages.
//!
//! One request builds one [`Context`] (the repository projection, the compiled
//! Agent surface and the database pulse) and one [`Page`] (title, seals, plate,
//! content markup, related panels and the copyable reference). Everything around
//! the page — bar, section tree, breadcrumb, colophon and toast — belongs to
//! [`shell`], so a section module never re-implements the layout.

mod development;
mod doc;
mod game;
mod jump;
mod live;
mod overview;
mod shell;
mod tree;

pub(super) use development::{
    decision as development_decision, development, direction as development_direction,
    evidence as development_evidence, lab as development_lab, open as development_open,
    research as development_research, rules as development_rules, work as development_work,
};
pub(super) use doc::document;
pub(super) use game::{
    agent_page as game_agent, capability_page as game_capability, deferred_page as game_deferred,
    index as game, model_page as game_model, storage_page as game_storage, tool_page as game_tool,
    vocabulary_page as game_vocabulary,
};
pub(super) use jump::jump;
pub(super) use live::{
    activity_detail as live_activity_detail, activity_list as live_activity,
    character_detail as live_character_detail, character_list as live_character,
    entity_detail as live_entity_detail, entity_list as live_entity, index as live,
    investigation_detail as live_investigation_detail, investigation_list as live_investigation,
    migration_page as live_migration, place_detail as live_place_detail, place_list as live_place,
    property_history as live_property_history, property_key_detail as live_property_key_detail,
    property_key_list as live_property_key, resolve_page as live_resolve,
    storage_index as live_storage, storage_snapshot as live_storage_snapshot,
    storage_table as live_storage_table, trait_detail as live_trait_detail,
    trait_index as live_trait, user_detail as live_user_detail, user_list as live_user,
};
pub(super) use overview::{brief, overview};

use std::time::Duration;

use axum::{
    http::{HeaderMap, StatusCode, Uri, header},
    response::{Html, IntoResponse, Response},
};
use maud::{Markup, html};

use super::{StudioState, agent::AgentSurface, record::Repository};

/// How long the shell waits for the connection check before reporting unavailable.
const PULSE_TIMEOUT: Duration = Duration::from_millis(500);

/// The default origin of a copyable reference when the request carries no host.
const DEFAULT_ORIGIN: &str = "http://127.0.0.1:3000";

/// One of the four primary Studio sections.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Section {
    Overview,
    Game,
    Development,
    Live,
}

impl Section {
    pub(super) const ALL: [Self; 4] = [Self::Overview, Self::Game, Self::Development, Self::Live];

    pub(super) fn label(self) -> &'static str {
        match self {
            Self::Overview => "Overview",
            Self::Game => "Game",
            Self::Development => "Development",
            Self::Live => "Live",
        }
    }

    pub(super) fn href(self) -> &'static str {
        match self {
            Self::Overview => "/",
            Self::Game => "/game",
            Self::Development => "/development",
            Self::Live => "/live",
        }
    }

    /// The small line above the tree name.
    pub(super) fn side(self) -> &'static str {
        match self {
            Self::Overview => "Studio",
            Self::Game => "Runtime side",
            Self::Development => "Development side",
            Self::Live => "Connected World",
        }
    }

    /// The bold line of the tree title.
    pub(super) fn tree_name(self) -> &'static str {
        match self {
            Self::Overview => "State of Aicadia",
            other => other.label(),
        }
    }
}

/// The seal palette: status vocabulary only, never decoration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Tone {
    Plain,
    Moss,
    Amber,
    Brick,
    Slate,
}

impl Tone {
    fn class(self) -> &'static str {
        match self {
            Self::Plain => "seal seal-plain",
            Self::Moss => "seal seal-moss",
            Self::Amber => "seal seal-amber",
            Self::Brick => "seal seal-brick",
            Self::Slate => "seal seal-slate",
        }
    }

    /// The tone one projected status, verdict or state word carries.
    pub(super) fn of(value: &str) -> Self {
        match value.trim().to_ascii_lowercase().as_str() {
            "accepted" | "active" | "complete" | "completed" | "connected" | "current" | "done"
            | "kept" | "live" | "load-bearing" | "ok" | "supported" => Self::Moss,
            "blocked" | "dropped" | "error" | "refuted" | "rejected" | "unavailable" => Self::Brick,
            "draft" | "in_progress" | "open" | "pending" | "proposed" | "queued" | "ready"
            | "warning" => Self::Amber,
            "archive" | "discarded" | "frozen" | "historical" | "inconclusive" | "researched"
            | "retained" | "superseded" | "throwaway" => Self::Slate,
            _ => Self::Plain,
        }
    }
}

/// One status seal above the page title.
#[derive(Clone, Debug)]
pub(super) struct Seal {
    pub(super) text: String,
    pub(super) tone: Tone,
}

impl Seal {
    pub(super) fn plain(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            tone: Tone::Plain,
        }
    }

    /// A seal whose tone follows its own projected value.
    pub(super) fn status(text: impl Into<String>) -> Self {
        let text = text.into();
        let tone = Tone::of(&text);
        Self { text, tone }
    }

    pub(super) fn toned(text: impl Into<String>, tone: Tone) -> Self {
        Self {
            text: text.into(),
            tone,
        }
    }
}

/// One value in the authority plate.
#[derive(Clone, Debug)]
pub(super) enum PlateValue {
    Text(String),
    Fact(String),
}

/// One `key · value` pair of the authority plate.
#[derive(Clone, Debug)]
pub(super) struct PlateRow {
    pub(super) key: String,
    pub(super) value: PlateValue,
}

impl PlateRow {
    pub(super) fn text(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: PlateValue::Text(value.into()),
        }
    }

    pub(super) fn fact(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: PlateValue::Fact(value.into()),
        }
    }
}

/// A plate action: either a copy button or a plain link.
#[derive(Clone, Debug)]
pub(super) enum Action {
    Copy { label: String, text: String },
    Link { label: String, href: String },
}

impl Action {
    pub(super) fn copy(label: impl Into<String>, text: impl Into<String>) -> Self {
        Self::Copy {
            label: label.into(),
            text: text.into(),
        }
    }

    pub(super) fn link(label: impl Into<String>, href: impl Into<String>) -> Self {
        Self::Link {
            label: label.into(),
            href: href.into(),
        }
    }
}

/// The authority plate under the page title.
#[derive(Clone, Debug, Default)]
pub(super) struct Plate {
    pub(super) row: Vec<PlateRow>,
    pub(super) action: Vec<Action>,
    pub(super) authority: Option<String>,
}

impl Plate {
    pub(super) fn new(row: Vec<PlateRow>) -> Self {
        Self {
            row,
            ..Self::default()
        }
    }

    pub(super) fn with_action(mut self, action: Vec<Action>) -> Self {
        self.action = action;
        self
    }

    pub(super) fn with_authority(mut self, authority: impl Into<String>) -> Self {
        self.authority = Some(authority.into());
        self
    }
}

/// One row of a related panel.
#[derive(Clone, Debug)]
pub(super) struct PanelItem {
    pub(super) label: String,
    pub(super) href: Option<String>,
    pub(super) note: Option<String>,
}

impl PanelItem {
    pub(super) fn link(label: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: Some(href.into()),
            note: None,
        }
    }

    pub(super) fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }
}

/// One outline entry of the record reader.
#[derive(Clone, Debug)]
pub(super) struct OutlineItem {
    pub(super) id: String,
    pub(super) title: String,
    pub(super) level: u8,
}

/// What a related panel holds.
#[derive(Clone, Debug)]
pub(super) enum PanelBody {
    List(Vec<PanelItem>),
    Outline(Vec<OutlineItem>),
}

/// One panel of the fixed Related column.
#[derive(Clone, Debug)]
pub(super) struct Panel {
    pub(super) title: String,
    pub(super) body: PanelBody,
}

impl Panel {
    pub(super) fn list(title: impl Into<String>, item: Vec<PanelItem>) -> Self {
        Self {
            title: title.into(),
            body: PanelBody::List(item),
        }
    }

    pub(super) fn outline(title: impl Into<String>, item: Vec<OutlineItem>) -> Self {
        Self {
            title: title.into(),
            body: PanelBody::Outline(item),
        }
    }
}

/// One breadcrumb step; the last step has no link.
#[derive(Clone, Debug)]
pub(super) struct Crumb {
    pub(super) label: String,
    pub(super) href: Option<String>,
}

impl Crumb {
    pub(super) fn link(label: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: Some(href.into()),
        }
    }

    pub(super) fn here(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: None,
        }
    }
}

/// The copyable reference line at the foot of every page.
#[derive(Clone, Debug)]
pub(super) struct Reference {
    pub(super) title: String,
    pub(super) url: String,
    pub(super) context: String,
}

impl Reference {
    /// `[Aicadia Studio · <title>](<url>) — <owning path or durable id>`.
    pub(super) fn line(&self) -> String {
        format!(
            "[Aicadia Studio · {}]({}) — {}",
            self.title, self.url, self.context
        )
    }
}

/// One rendered Studio page, independent of the shell around it.
#[derive(Clone, Debug)]
pub(super) struct Page {
    pub(super) section: Section,
    pub(super) document_title: String,
    pub(super) title: String,
    pub(super) lede: Option<String>,
    pub(super) seal: Vec<Seal>,
    pub(super) crumb: Vec<Crumb>,
    pub(super) plate: Option<Plate>,
    pub(super) content: Markup,
    pub(super) related: Vec<Panel>,
    pub(super) reference: Option<Reference>,
}

impl Page {
    pub(super) fn new(section: Section, title: impl Into<String>) -> Self {
        let title = title.into();
        Self {
            document_title: document_title(&title, section),
            section,
            title,
            lede: None,
            seal: Vec::new(),
            crumb: Vec::new(),
            plate: None,
            content: Markup::default(),
            related: Vec::new(),
            reference: None,
        }
    }

    pub(super) fn with_document_title(mut self, title: impl Into<String>) -> Self {
        self.document_title = title.into();
        self
    }

    pub(super) fn with_lede(mut self, lede: impl Into<String>) -> Self {
        self.lede = Some(lede.into());
        self
    }

    pub(super) fn with_seal(mut self, seal: Vec<Seal>) -> Self {
        self.seal = seal;
        self
    }

    pub(super) fn with_crumb(mut self, crumb: Vec<Crumb>) -> Self {
        self.crumb = crumb;
        self
    }

    pub(super) fn with_plate(mut self, plate: Plate) -> Self {
        self.plate = Some(plate);
        self
    }

    pub(super) fn with_content(mut self, content: Markup) -> Self {
        self.content = content;
        self
    }

    pub(super) fn with_related(mut self, related: Vec<Panel>) -> Self {
        self.related = related;
        self
    }

    pub(super) fn with_reference(mut self, reference: Reference) -> Self {
        self.reference = Some(reference);
        self
    }
}

/// `<title>` of a page: the page name, its section and Studio.
fn document_title(title: &str, section: Section) -> String {
    if section == Section::Overview {
        format!("{title} · Aicadia Studio")
    } else {
        format!("{title} · {} · Aicadia Studio", section.label())
    }
}

/// The connection line of the bar.
#[derive(Clone, Debug)]
pub(super) struct Pulse {
    pub(super) database: Option<String>,
    pub(super) state: &'static str,
    pub(super) read_at: String,
}

impl Pulse {
    async fn read(pool: &sqlx::PgPool) -> Self {
        let database = pool.connect_options().get_database().map(str::to_owned);
        let reachable = tokio::time::timeout(
            PULSE_TIMEOUT,
            sqlx::query_scalar::<_, i32>("select 1").fetch_one(pool),
        )
        .await
        .is_ok_and(|result| result.is_ok());

        Self {
            database,
            state: if reachable {
                "connected"
            } else {
                "unavailable"
            },
            read_at: chrono::Local::now().format("%H:%M").to_string(),
        }
    }

    pub(super) fn is_connected(&self) -> bool {
        self.state == "connected"
    }
}

/// Everything one request needs to render any Studio page.
pub(super) struct Context {
    repository: Repository,
    surface: AgentSurface,
    origin: String,
    path: String,
    uri: String,
    pulse: Pulse,
}

impl Context {
    /// Load the projection, the compiled Agent surface and the connection pulse.
    pub(super) async fn build(
        state: &StudioState,
        header: &HeaderMap,
        uri: &Uri,
    ) -> Result<Self, Response> {
        let repository = Repository::load(state.repository_root.as_path()).map_err(|error| {
            eprintln!("Studio repository projection failed: {error}");
            shell::plain_error(
                "The governed repository could not be projected",
                &error.to_string(),
            )
        })?;
        let surface = super::agent::surface(
            &repository,
            aicadia::server::mcp_tool_catalog(state.world.clone()),
        );
        let origin = header
            .get(header::HOST)
            .and_then(|host| host.to_str().ok())
            .map(|host| format!("http://{host}"))
            .unwrap_or_else(|| DEFAULT_ORIGIN.to_owned());

        Ok(Self {
            repository,
            surface,
            origin,
            path: uri.path().to_owned(),
            uri: uri
                .path_and_query()
                .map_or_else(|| uri.path().to_owned(), ToString::to_string),
            pulse: Pulse::read(&state.pool).await,
        })
    }

    pub(super) fn repository(&self) -> &Repository {
        &self.repository
    }

    pub(super) fn surface(&self) -> &AgentSurface {
        &self.surface
    }

    pub(super) fn pulse(&self) -> &Pulse {
        &self.pulse
    }

    /// The request path, used to mark the current tree link.
    pub(super) fn path(&self) -> &str {
        &self.path
    }

    /// The request path with its query, used by Refresh.
    pub(super) fn uri(&self) -> &str {
        &self.uri
    }

    /// One absolute Studio URL for a copyable reference.
    pub(super) fn url(&self, path: &str) -> String {
        format!("{}{path}", self.origin)
    }

    pub(super) fn render(&self, page: Page) -> Response {
        self.render_status(page, StatusCode::OK)
    }

    pub(super) fn render_status(&self, page: Page, status: StatusCode) -> Response {
        (status, Html(shell::render(self, &page).into_string())).into_response()
    }
}

/// The Studio route of one governed record.
pub(super) fn doc_href(path: &str) -> String {
    format!("/doc/{path}")
}

/// The human label of one home id, used in breadcrumbs, plates and panels.
pub(super) fn home_label(home_id: &str) -> &'static str {
    match home_id {
        "constitution" => "Documentation constitution",
        "build-constitution" => "Build constitution",
        "entry-pointer" => "Agent entry pointer",
        "vocabulary" => "Canonical vocabulary",
        "game-index" => "Game contract index",
        "model" => "Model contract",
        "model-concern" => "Model concern",
        "capability" => "Capability contract",
        "game-contract" => "Game contract",
        "concept-index" => "Concept index",
        "concept-record" => "Concept record",
        "concept-log-index" => "Concept log index",
        "decision-register" => "Decision register",
        "concept-archive" => "Concept archive",
        "research-index" => "Research index",
        "research-report" => "Research report",
        "research-archive" => "Research archive",
        "methodology-index" => "Methodology index",
        "methodology-record" => "Working method",
        "evidence-index" => "Evidence index",
        "evidence-slice" => "Evidence slice",
        "runner-contract" => "Runner contract",
        "backlog-index" => "Backlog index",
        "capability-map" => "Capability map",
        "backlog-item" => "Backlog item",
        "plan" => "Plan",
        "plan-fragment" => "Plan fragment",
        "skill" => "Skill",
        "skill-asset" => "Skill asset",
        "lab-index" => "Lab index",
        "lab-track" => "Lab track",
        "lab-experiment" => "Lab experiment",
        "lab-record" => "Lab record",
        "agent-contract-index" => "Agent contract sources",
        "agent-instruction" => "Agent instruction",
        "agent-tool-text" => "Agent tool text",
        "migration" => "Migration",
        _ => "Unmatched file",
    }
}

/// The section a home belongs to, following the documentation side it carries.
pub(super) fn section_for_home(home_id: &str) -> Section {
    match super::home::home(home_id).map(|home| home.side) {
        Some(super::home::Side::Runtime) => Section::Game,
        _ => Section::Development,
    }
}

/// `1 record` / `4 records`, so a count never reads as `1 records`.
///
/// The plural is given rather than derived: English is not a rule this code owns.
pub(super) fn count(amount: usize, singular: &str, plural: &str) -> String {
    if amount == 1 {
        format!("{amount} {singular}")
    } else {
        format!("{amount} {plural}")
    }
}

/// One `.note` block for an explicit partial, warning or error state.
pub(super) fn note(tone: Tone, message: &str) -> Markup {
    let class = match tone {
        Tone::Moss => "note note-ok",
        Tone::Amber => "note note-warn",
        Tone::Brick => "note note-error",
        _ => "note",
    };
    html! { div class=(class) { (message) } }
}

/// One `.empty-state` block for an honestly empty page.
pub(super) fn empty_state(message: &str) -> Markup {
    html! { div class="empty-state" { (message) } }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_section_has_a_distinct_route_and_label() {
        let href: Vec<&str> = Section::ALL.iter().map(|section| section.href()).collect();

        assert_eq!(href, ["/", "/game", "/development", "/live"]);
        assert_eq!(Section::Overview.tree_name(), "State of Aicadia");
        assert_eq!(Section::Game.side(), "Runtime side");
    }

    #[test]
    fn a_status_word_selects_its_seal_tone() {
        assert_eq!(Tone::of("accepted"), Tone::Moss);
        assert_eq!(Tone::of("Draft"), Tone::Amber);
        assert_eq!(Tone::of("refuted"), Tone::Brick);
        assert_eq!(Tone::of("superseded"), Tone::Slate);
        assert_eq!(Tone::of("runtime"), Tone::Plain);
    }

    #[test]
    fn a_reference_line_keeps_the_agreed_copy_format() {
        let reference = Reference {
            title: "Entity".to_owned(),
            url: "http://127.0.0.1:3000/doc/game/docs/model/entity/README.md".to_owned(),
            context: "game/docs/model/entity/README.md".to_owned(),
        };

        assert_eq!(
            reference.line(),
            "[Aicadia Studio · Entity](http://127.0.0.1:3000/doc/game/docs/model/entity/README.md) — game/docs/model/entity/README.md"
        );
    }

    #[test]
    fn a_home_maps_to_its_label_and_section() {
        assert_eq!(home_label("model"), "Model contract");
        assert_eq!(home_label("nonsense"), "Unmatched file");
        assert_eq!(section_for_home("model"), Section::Game);
        assert_eq!(section_for_home("plan"), Section::Development);
        assert_eq!(section_for_home("evidence-slice"), Section::Development);
    }

    #[test]
    fn a_count_agrees_with_its_noun() {
        assert_eq!(count(1, "record", "records"), "1 record");
        assert_eq!(count(4, "record", "records"), "4 records");
        assert_eq!(count(509, "entry", "entries"), "509 entries");
    }
}
