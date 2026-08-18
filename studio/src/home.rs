//! Governed repository roots and the home mapping that classifies every record.

use std::{
    fs,
    path::{Path, PathBuf},
};

/// Repository roots whose Markdown Studio projects. `target/` is never scanned.
const GOVERNED_DIRECTORY: [&str; 8] = [
    "game/docs",
    "game/mcp/agent",
    "game/migration",
    "dev/docs",
    "dev/backlog",
    "dev/plans",
    "dev/skills",
    "dev/lab",
];

/// Governed records that are not inside a governed directory.
const GOVERNED_FILE: [&str; 3] = ["AGENTS.md", "CLAUDE.md", "dev/CONTEXT.md"];

/// Per-record read bound; a larger governed file is an explicit error.
pub const MAX_RECORD_BYTES: usize = 512 * 1024;

/// Total governed-record bound; more records than this is an explicit error.
pub const MAX_RECORD_COUNT: usize = 1_000;

/// Documentation side a home belongs to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Runtime,
    Development,
    Bridge,
}

impl Side {
    pub fn label(self) -> &'static str {
        match self {
            Self::Runtime => "runtime",
            Self::Development => "development",
            Self::Bridge => "bridge",
        }
    }
}

/// How a home is excluded from the documentation sweeps.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frozen {
    /// Live: every sweep applies.
    No,
    /// Frozen history: link, anchor and metadata sweeps are skipped.
    Yes,
    /// Frozen citations: link and anchor sweeps are skipped, metadata still applies.
    Link,
    /// Frozen exactly when the plan's own `status` is `complete` or `superseded`.
    PlanStatus,
    /// Frozen exactly when the plan owning this fragment is frozen.
    OwningPlan,
}

/// Whether a home's records must carry a role header.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoleHeaderRule {
    Required,
    Optional,
    /// Required while the record is live; frozen records keep whatever they had.
    RequiredWhenLive,
}

/// The front-matter vocabulary a home validates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vocabulary {
    /// No vocabulary: any well-formed front matter is tolerated and only displayed.
    None,
    Model,
    Concept,
    Research,
    Evidence,
    LabTrack,
    LabExperiment,
    Plan,
    /// Skill front matter: `name` and `description` must be present, values are free.
    SkillPresence,
    /// Never validated, not even for grammar (templates carry placeholders).
    Unchecked,
}

/// The index README that must link every record of a home (conventions section 9).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Index {
    /// The home has no index obligation.
    None,
    /// One fixed index README.
    Fixed(&'static str),
    /// The `README.md` of the directory above the record's own directory.
    ParentReadme,
}

/// How a repository path is matched to a home.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rule {
    /// Exactly this repository path.
    Exact(&'static str),
    /// `<prefix>/<segment>{depth}/README.md`.
    Readme { prefix: &'static str, depth: usize },
    /// `<prefix>/<segment>{depth}/<name>`.
    Named {
        prefix: &'static str,
        depth: usize,
        name: &'static str,
    },
    /// `<prefix>/<segment>{depth}/<other>.md`, never a `README.md`.
    Leaf { prefix: &'static str, depth: usize },
    /// Any Markdown at any depth below `<prefix>`.
    Subtree { prefix: &'static str },
    /// `dev/docs/concept/log/<YYYY-MM>.md`.
    MonthlyLog,
    /// `migration/<nnnn>_<name>.sql`.
    Migration,
}

impl Rule {
    fn matches(self, path: &str) -> bool {
        match self {
            Self::Exact(exact) => path == exact,
            Self::Readme { prefix, depth } => {
                matches_named(path, prefix, depth, "README.md") && path.ends_with(".md")
            }
            Self::Named {
                prefix,
                depth,
                name,
            } => matches_named(path, prefix, depth, name),
            Self::Leaf { prefix, depth } => {
                let Some(tail) = under(path, prefix) else {
                    return false;
                };
                tail.matches('/').count() == depth
                    && path.ends_with(".md")
                    && file_name(path) != "README.md"
            }
            Self::Subtree { prefix } => under(path, prefix).is_some() && path.ends_with(".md"),
            Self::MonthlyLog => {
                matches_depth(path, "dev/docs/concept/log", 0) && is_month_file(file_name(path))
            }
            Self::Migration => {
                matches_depth(path, "game/migration", 0) && is_migration_file(file_name(path))
            }
        }
    }
}

fn under<'a>(path: &'a str, prefix: &str) -> Option<&'a str> {
    path.strip_prefix(prefix)?.strip_prefix('/')
}

fn matches_depth(path: &str, prefix: &str, depth: usize) -> bool {
    under(path, prefix).is_some_and(|tail| tail.matches('/').count() == depth)
}

fn matches_named(path: &str, prefix: &str, depth: usize, name: &str) -> bool {
    matches_depth(path, prefix, depth) && file_name(path) == name
}

pub fn file_name(path: &str) -> &str {
    path.rsplit('/').next().unwrap_or(path)
}

/// The directory part of a repository path, empty at the repository root.
pub fn directory(path: &str) -> &str {
    match path.rfind('/') {
        Some(position) => &path[..position],
        None => "",
    }
}

fn is_month_file(name: &str) -> bool {
    let Some(stem) = name.strip_suffix(".md") else {
        return false;
    };
    let byte = stem.as_bytes();
    byte.len() == 7
        && byte[..4].iter().all(u8::is_ascii_digit)
        && byte[4] == b'-'
        && byte[5..].iter().all(u8::is_ascii_digit)
}

fn is_migration_file(name: &str) -> bool {
    let Some(stem) = name.strip_suffix(".sql") else {
        return false;
    };
    let byte = stem.as_bytes();
    byte.len() > 5 && byte[..4].iter().all(u8::is_ascii_digit) && byte[4] == b'_'
}

/// One governed home: its identity, side, match rule and record obligations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Home {
    pub id: &'static str,
    pub side: Side,
    pub rule: Rule,
    pub frozen: Frozen,
    pub role_header: RoleHeaderRule,
    pub vocabulary: Vocabulary,
    pub index: Index,
}

/// The complete home table; the first matching row wins, so rows are most specific first.
pub static HOME: &[Home] = &[
    Home {
        id: "constitution",
        side: Side::Development,
        rule: Rule::Exact("dev/docs/README.md"),
        frozen: Frozen::No,
        role_header: RoleHeaderRule::Required,
        vocabulary: Vocabulary::None,
        index: Index::None,
    },
    Home {
        id: "build-constitution",
        side: Side::Development,
        rule: Rule::Exact("AGENTS.md"),
        frozen: Frozen::No,
        role_header: RoleHeaderRule::Required,
        vocabulary: Vocabulary::None,
        index: Index::None,
    },
    Home {
        id: "entry-pointer",
        side: Side::Development,
        rule: Rule::Exact("CLAUDE.md"),
        frozen: Frozen::No,
        role_header: RoleHeaderRule::Required,
        vocabulary: Vocabulary::None,
        index: Index::None,
    },
    Home {
        id: "vocabulary",
        side: Side::Development,
        rule: Rule::Exact("dev/CONTEXT.md"),
        frozen: Frozen::No,
        role_header: RoleHeaderRule::Required,
        vocabulary: Vocabulary::None,
        index: Index::None,
    },
    Home {
        id: "game-index",
        side: Side::Runtime,
        rule: Rule::Exact("game/docs/README.md"),
        frozen: Frozen::No,
        role_header: RoleHeaderRule::Required,
        vocabulary: Vocabulary::None,
        index: Index::None,
    },
    Home {
        id: "model",
        side: Side::Runtime,
        rule: Rule::Readme {
            prefix: "game/docs/model",
            depth: 1,
        },
        frozen: Frozen::No,
        role_header: RoleHeaderRule::Required,
        vocabulary: Vocabulary::Model,
        index: Index::None,
    },
    Home {
        id: "model-concern",
        side: Side::Runtime,
        rule: Rule::Leaf {
            prefix: "game/docs/model",
            depth: 1,
        },
        frozen: Frozen::No,
        role_header: RoleHeaderRule::Required,
        vocabulary: Vocabulary::None,
        index: Index::None,
    },
    Home {
        id: "capability",
        side: Side::Runtime,
        rule: Rule::Leaf {
            prefix: "game/docs/capability",
            depth: 0,
        },
        frozen: Frozen::No,
        role_header: RoleHeaderRule::Required,
        vocabulary: Vocabulary::None,
        index: Index::None,
    },
    Home {
        id: "game-contract",
        side: Side::Runtime,
        rule: Rule::Leaf {
            prefix: "game/docs",
            depth: 0,
        },
        frozen: Frozen::No,
        role_header: RoleHeaderRule::Required,
        vocabulary: Vocabulary::None,
        index: Index::None,
    },
    Home {
        id: "concept-index",
        side: Side::Development,
        rule: Rule::Exact("dev/docs/concept/README.md"),
        frozen: Frozen::No,
        role_header: RoleHeaderRule::Required,
        vocabulary: Vocabulary::None,
        index: Index::None,
    },
    Home {
        id: "concept-log-index",
        side: Side::Development,
        rule: Rule::Exact("dev/docs/concept/log/README.md"),
        frozen: Frozen::No,
        role_header: RoleHeaderRule::Required,
        vocabulary: Vocabulary::None,
        index: Index::None,
    },
    Home {
        id: "decision-register",
        side: Side::Development,
        rule: Rule::MonthlyLog,
        frozen: Frozen::Link,
        role_header: RoleHeaderRule::Required,
        vocabulary: Vocabulary::None,
        index: Index::None,
    },
    Home {
        id: "concept-archive",
        side: Side::Development,
        rule: Rule::Subtree {
            prefix: "dev/docs/concept/archive",
        },
        frozen: Frozen::Yes,
        role_header: RoleHeaderRule::Optional,
        vocabulary: Vocabulary::None,
        index: Index::None,
    },
    Home {
        id: "concept-record",
        side: Side::Development,
        rule: Rule::Leaf {
            prefix: "dev/docs/concept",
            depth: 0,
        },
        frozen: Frozen::No,
        role_header: RoleHeaderRule::Required,
        vocabulary: Vocabulary::Concept,
        index: Index::Fixed("dev/docs/concept/README.md"),
    },
    Home {
        id: "research-index",
        side: Side::Development,
        rule: Rule::Exact("dev/docs/research/README.md"),
        frozen: Frozen::No,
        role_header: RoleHeaderRule::Required,
        vocabulary: Vocabulary::None,
        index: Index::None,
    },
    Home {
        id: "research-archive",
        side: Side::Development,
        rule: Rule::Subtree {
            prefix: "dev/docs/research/archive",
        },
        frozen: Frozen::Yes,
        role_header: RoleHeaderRule::Optional,
        vocabulary: Vocabulary::None,
        index: Index::None,
    },
    Home {
        id: "research-report",
        side: Side::Development,
        rule: Rule::Leaf {
            prefix: "dev/docs/research",
            depth: 0,
        },
        frozen: Frozen::No,
        role_header: RoleHeaderRule::Required,
        vocabulary: Vocabulary::Research,
        index: Index::Fixed("dev/docs/research/README.md"),
    },
    Home {
        id: "evidence-index",
        side: Side::Bridge,
        rule: Rule::Exact("dev/docs/evidence/README.md"),
        frozen: Frozen::No,
        role_header: RoleHeaderRule::Required,
        vocabulary: Vocabulary::None,
        index: Index::None,
    },
    Home {
        id: "runner-contract",
        side: Side::Bridge,
        rule: Rule::Leaf {
            prefix: "dev/docs/evidence/runner",
            depth: 0,
        },
        frozen: Frozen::No,
        role_header: RoleHeaderRule::Required,
        vocabulary: Vocabulary::None,
        index: Index::None,
    },
    Home {
        id: "evidence-slice",
        side: Side::Bridge,
        rule: Rule::Leaf {
            prefix: "dev/docs/evidence",
            depth: 0,
        },
        frozen: Frozen::No,
        role_header: RoleHeaderRule::Required,
        vocabulary: Vocabulary::Evidence,
        index: Index::Fixed("dev/docs/evidence/README.md"),
    },
    Home {
        id: "backlog-index",
        side: Side::Development,
        rule: Rule::Exact("dev/backlog/README.md"),
        frozen: Frozen::No,
        role_header: RoleHeaderRule::Required,
        vocabulary: Vocabulary::None,
        index: Index::None,
    },
    Home {
        id: "capability-map",
        side: Side::Development,
        rule: Rule::Exact("dev/backlog/capability-map.md"),
        frozen: Frozen::No,
        role_header: RoleHeaderRule::Required,
        vocabulary: Vocabulary::None,
        index: Index::None,
    },
    Home {
        id: "backlog-item",
        side: Side::Development,
        rule: Rule::Leaf {
            prefix: "dev/backlog/items",
            depth: 0,
        },
        frozen: Frozen::No,
        role_header: RoleHeaderRule::Required,
        vocabulary: Vocabulary::None,
        index: Index::None,
    },
    Home {
        id: "plan",
        side: Side::Development,
        rule: Rule::Named {
            prefix: "dev/plans",
            depth: 1,
            name: "plan.md",
        },
        frozen: Frozen::PlanStatus,
        role_header: RoleHeaderRule::RequiredWhenLive,
        vocabulary: Vocabulary::Plan,
        index: Index::None,
    },
    Home {
        id: "plan-fragment",
        side: Side::Development,
        rule: Rule::Subtree {
            prefix: "dev/plans",
        },
        frozen: Frozen::OwningPlan,
        role_header: RoleHeaderRule::Optional,
        vocabulary: Vocabulary::None,
        index: Index::None,
    },
    Home {
        id: "skill",
        side: Side::Development,
        rule: Rule::Named {
            prefix: "dev/skills",
            depth: 1,
            name: "SKILL.md",
        },
        frozen: Frozen::No,
        role_header: RoleHeaderRule::Required,
        vocabulary: Vocabulary::SkillPresence,
        index: Index::None,
    },
    Home {
        id: "skill-asset",
        side: Side::Development,
        rule: Rule::Subtree {
            prefix: "dev/skills",
        },
        frozen: Frozen::No,
        role_header: RoleHeaderRule::Optional,
        vocabulary: Vocabulary::Unchecked,
        index: Index::None,
    },
    Home {
        id: "lab-index",
        side: Side::Development,
        rule: Rule::Exact("dev/lab/README.md"),
        frozen: Frozen::No,
        role_header: RoleHeaderRule::Required,
        vocabulary: Vocabulary::None,
        index: Index::None,
    },
    Home {
        id: "lab-experiment",
        side: Side::Development,
        rule: Rule::Readme {
            prefix: "dev/lab",
            depth: 2,
        },
        frozen: Frozen::No,
        role_header: RoleHeaderRule::Required,
        vocabulary: Vocabulary::LabExperiment,
        index: Index::ParentReadme,
    },
    Home {
        id: "lab-track",
        side: Side::Development,
        rule: Rule::Readme {
            prefix: "dev/lab",
            depth: 1,
        },
        frozen: Frozen::No,
        role_header: RoleHeaderRule::Required,
        vocabulary: Vocabulary::LabTrack,
        index: Index::Fixed("dev/lab/README.md"),
    },
    Home {
        id: "lab-record",
        side: Side::Development,
        rule: Rule::Subtree { prefix: "dev/lab" },
        frozen: Frozen::No,
        role_header: RoleHeaderRule::Required,
        vocabulary: Vocabulary::None,
        index: Index::None,
    },
    Home {
        id: "agent-contract-index",
        side: Side::Runtime,
        rule: Rule::Exact("game/mcp/agent/README.md"),
        frozen: Frozen::No,
        role_header: RoleHeaderRule::Required,
        vocabulary: Vocabulary::None,
        index: Index::None,
    },
    Home {
        id: "agent-instruction",
        side: Side::Runtime,
        rule: Rule::Leaf {
            prefix: "game/mcp/agent/instruction",
            depth: 0,
        },
        frozen: Frozen::No,
        role_header: RoleHeaderRule::Optional,
        vocabulary: Vocabulary::None,
        index: Index::None,
    },
    Home {
        id: "agent-tool-text",
        side: Side::Runtime,
        rule: Rule::Leaf {
            prefix: "game/mcp/agent/tool",
            depth: 0,
        },
        frozen: Frozen::No,
        role_header: RoleHeaderRule::Optional,
        vocabulary: Vocabulary::None,
        index: Index::None,
    },
    Home {
        id: "migration",
        side: Side::Runtime,
        rule: Rule::Migration,
        frozen: Frozen::No,
        role_header: RoleHeaderRule::Optional,
        vocabulary: Vocabulary::None,
        index: Index::None,
    },
];

/// The home owning a repository path, or `None` when no home matches it.
pub fn classify(path: &str) -> Option<&'static Home> {
    HOME.iter().find(|home| home.rule.matches(path))
}

/// The home with this id, used by pages that address one home directly.
pub fn home(id: &str) -> Option<&'static Home> {
    HOME.iter().find(|home| home.id == id)
}

/// One governed file on disk with the home that owns it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordPath {
    /// Repository-relative, `/`-separated.
    pub path: String,
    /// `None` for a Markdown file under a governed root that no home matches.
    pub home: Option<&'static Home>,
}

/// A governed read that could not be completed within its explicit bounds.
#[derive(Debug)]
pub enum ScanError {
    Read { path: String, error: std::io::Error },
    TooLarge { path: String, bytes: usize },
    NotUtf8 { path: String },
    TooManyRecords { count: usize, bound: usize },
}

impl std::fmt::Display for ScanError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read { path, error } => write!(formatter, "{path} could not be read: {error}"),
            Self::TooLarge { path, bytes } => write!(
                formatter,
                "{path} is {bytes} bytes and exceeds the {MAX_RECORD_BYTES} byte record bound"
            ),
            Self::NotUtf8 { path } => write!(formatter, "{path} is not UTF-8"),
            Self::TooManyRecords { count, bound } => write!(
                formatter,
                "the governed roots hold {count} records and exceed the {bound} record bound"
            ),
        }
    }
}

impl std::error::Error for ScanError {}

/// Every governed record under `repository_root`, sorted by path.
///
/// Markdown under a governed root that matches no home is returned with `home: None`
/// so the lint can report it; it is never silently dropped.
pub fn scan(repository_root: &Path) -> Result<Vec<RecordPath>, ScanError> {
    let mut path = Vec::new();

    for file in GOVERNED_FILE {
        if repository_root.join(file).is_file() {
            path.push(file.to_owned());
        }
    }

    for root in GOVERNED_DIRECTORY {
        let mut pending = vec![(repository_root.join(root), root.to_owned())];
        while let Some((directory, relative)) = pending.pop() {
            let entry = match fs::read_dir(&directory) {
                Ok(entry) => entry,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
                Err(error) => {
                    return Err(ScanError::Read {
                        path: relative,
                        error,
                    });
                }
            };
            for entry in entry {
                let entry = entry.map_err(|error| ScanError::Read {
                    path: relative.clone(),
                    error,
                })?;
                let name = entry.file_name().to_string_lossy().into_owned();
                let child = format!("{relative}/{name}");
                let file_type = entry.file_type().map_err(|error| ScanError::Read {
                    path: child.clone(),
                    error,
                })?;
                if file_type.is_dir() {
                    if name != "target" {
                        pending.push((entry.path(), child));
                    }
                } else if is_governed_record(&child) {
                    path.push(child);
                }
            }
        }
    }

    if path.len() > MAX_RECORD_COUNT {
        return Err(ScanError::TooManyRecords {
            count: path.len(),
            bound: MAX_RECORD_COUNT,
        });
    }

    path.sort();
    Ok(path
        .into_iter()
        .map(|path| RecordPath {
            home: classify(&path),
            path,
        })
        .collect())
}

/// Studio reads governed Markdown from the owned roots and SQL only under `game/migration/`.
fn is_governed_record(path: &str) -> bool {
    path.ends_with(".md") || (path.starts_with("game/migration/") && path.ends_with(".sql"))
}

/// Read one governed file within the per-file bound.
pub fn read(repository_root: &Path, path: &str) -> Result<String, ScanError> {
    let absolute: PathBuf = repository_root.join(path);
    let bytes = fs::read(&absolute).map_err(|error| ScanError::Read {
        path: path.to_owned(),
        error,
    })?;
    if bytes.len() > MAX_RECORD_BYTES {
        return Err(ScanError::TooLarge {
            path: path.to_owned(),
            bytes: bytes.len(),
        });
    }
    String::from_utf8(bytes).map_err(|_| ScanError::NotUtf8 {
        path: path.to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(path: &str) -> Option<&'static str> {
        classify(path).map(|home| home.id)
    }

    #[test]
    fn the_most_specific_home_wins_over_its_parent_pattern() {
        assert_eq!(id("game/docs/README.md"), Some("game-index"));
        assert_eq!(id("game/docs/domain.md"), Some("game-contract"));
        assert_eq!(id("game/docs/model/entity/README.md"), Some("model"));
        assert_eq!(id("game/docs/model/entity/value.md"), Some("model-concern"));
        assert_eq!(id("game/docs/capability/get_world.md"), Some("capability"));
        assert_eq!(id("dev/docs/concept/README.md"), Some("concept-index"));
        assert_eq!(id("dev/docs/concept/discovery.md"), Some("concept-record"));
        assert_eq!(
            id("dev/docs/concept/log/README.md"),
            Some("concept-log-index")
        );
        assert_eq!(
            id("dev/docs/concept/log/2026-08.md"),
            Some("decision-register")
        );
        assert_eq!(
            id("dev/docs/concept/archive/00-vision.md"),
            Some("concept-archive")
        );
        assert_eq!(
            id("dev/docs/research/archive/x.md"),
            Some("research-archive")
        );
        assert_eq!(
            id("dev/docs/research/world-momentum.md"),
            Some("research-report")
        );
        assert_eq!(id("dev/docs/evidence/README.md"), Some("evidence-index"));
        assert_eq!(id("dev/docs/evidence/trait.md"), Some("evidence-slice"));
        assert_eq!(
            id("dev/docs/evidence/runner/trait-playtest.md"),
            Some("runner-contract")
        );
        assert_eq!(id("dev/backlog/README.md"), Some("backlog-index"));
        assert_eq!(id("dev/backlog/capability-map.md"), Some("capability-map"));
        assert_eq!(id("dev/backlog/items/x.md"), Some("backlog-item"));
        assert_eq!(id("dev/plans/20260817-x/plan.md"), Some("plan"));
        assert_eq!(
            id("dev/plans/20260817-x/move-map/t2.md"),
            Some("plan-fragment")
        );
        assert_eq!(id("dev/skills/build-aicadia/SKILL.md"), Some("skill"));
        assert_eq!(
            id("dev/skills/build-aicadia/assets/plan-template.md"),
            Some("skill-asset")
        );
        assert_eq!(id("dev/lab/README.md"), Some("lab-index"));
        assert_eq!(id("dev/lab/multiplayer/README.md"), Some("lab-track"));
        assert_eq!(
            id("dev/lab/multiplayer/01-a/README.md"),
            Some("lab-experiment")
        );
        assert_eq!(id("dev/lab/multiplayer/scenarios.md"), Some("lab-record"));
        assert_eq!(id("dev/lab/multiplayer/01-a/note.md"), Some("lab-record"));
        assert_eq!(id("game/mcp/agent/README.md"), Some("agent-contract-index"));
        assert_eq!(
            id("game/mcp/agent/instruction/00-contract.md"),
            Some("agent-instruction")
        );
        assert_eq!(
            id("game/mcp/agent/tool/get_world.md"),
            Some("agent-tool-text")
        );
        assert_eq!(id("game/migration/0001_world.sql"), Some("migration"));
        assert_eq!(id("dev/docs/README.md"), Some("constitution"));
        assert_eq!(id("AGENTS.md"), Some("build-constitution"));
        assert_eq!(id("CLAUDE.md"), Some("entry-pointer"));
        assert_eq!(id("dev/CONTEXT.md"), Some("vocabulary"));
    }

    #[test]
    fn markdown_without_a_home_stays_unmatched() {
        assert_eq!(id("dev/docs/stray.md"), None);
        assert_eq!(id("game/docs/model/entity/nested/deep.md"), None);
        assert_eq!(id("dev/docs/concept/log/notes.md"), None);
        assert_eq!(id("game/migration/notes.sql"), None);
        assert_eq!(id("studio/web/studio.js"), None);
    }

    #[test]
    fn every_home_id_is_unique_and_reachable_by_id() {
        for home in HOME {
            assert_eq!(super::home(home.id), Some(home));
            assert_eq!(
                HOME.iter().filter(|other| other.id == home.id).count(),
                1,
                "duplicate home id {}",
                home.id
            );
        }
    }

    #[test]
    fn scanning_a_temporary_root_reports_unmatched_markdown_and_skips_target() {
        let root = crate::test_directory("home-scan");
        std::fs::create_dir_all(root.join("game/docs/model/entity")).expect("model directory");
        std::fs::create_dir_all(root.join("dev/docs/target")).expect("target directory");
        std::fs::write(root.join("AGENTS.md"), "# A\n").expect("build constitution");
        std::fs::write(root.join("dev/docs/README.md"), "# C\n").expect("constitution");
        std::fs::write(root.join("dev/docs/stray.md"), "# S\n").expect("stray record");
        std::fs::write(root.join("game/docs/model/entity/README.md"), "# E\n").expect("model");
        std::fs::write(root.join("dev/docs/target/ignored.md"), "# I\n").expect("skipped record");

        let scanned = scan(&root).expect("scan should stay within its bounds");
        let path = scanned
            .iter()
            .map(|record| record.path.as_str())
            .collect::<Vec<_>>();

        assert_eq!(
            path,
            vec![
                "AGENTS.md",
                "dev/docs/README.md",
                "dev/docs/stray.md",
                "game/docs/model/entity/README.md",
            ]
        );
        assert!(
            scanned
                .iter()
                .any(|record| record.path == "dev/docs/stray.md" && record.home.is_none())
        );
    }

    #[test]
    fn a_file_over_the_read_bound_is_an_explicit_error() {
        let root = crate::test_directory("home-bound");
        std::fs::create_dir_all(root.join("dev/docs")).expect("docs directory");
        std::fs::write(
            root.join("dev/docs/README.md"),
            "x".repeat(MAX_RECORD_BYTES + 1),
        )
        .expect("oversized record");

        assert!(matches!(
            read(&root, "dev/docs/README.md"),
            Err(ScanError::TooLarge { .. })
        ));
    }
}
