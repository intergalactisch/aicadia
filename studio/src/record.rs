//! Per-record parsing and the repository aggregate every Studio page and the lint read.

use std::{
    collections::{BTreeMap, HashMap},
    path::{Path, PathBuf},
};

use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd, html};
use serde::Serialize;

use super::{
    home::{self, Frozen, Home, Index, RoleHeaderRule, ScanError, Vocabulary},
    model,
};

/// A parsed front-matter value: one scalar or one inline list.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Scalar(String),
    List(Vec<String>),
}

impl Value {
    pub fn scalar(&self) -> Option<&str> {
        match self {
            Self::Scalar(scalar) => Some(scalar),
            Self::List(_) => None,
        }
    }

    pub fn list(&self) -> Option<&[String]> {
        match self {
            Self::List(list) => Some(list),
            Self::Scalar(_) => None,
        }
    }
}

/// One front-matter field with the file line it was read from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    pub key: String,
    pub value: Value,
    pub line: usize,
}

/// The strict front-matter subset: ordered `key: value` pairs, no nesting.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FrontMatter {
    field: Vec<Field>,
}

impl FrontMatter {
    pub fn is_empty(&self) -> bool {
        self.field.is_empty()
    }

    pub fn field(&self) -> &[Field] {
        &self.field
    }

    /// The file line the key was read from.
    pub fn line(&self, key: &str) -> Option<usize> {
        self.field
            .iter()
            .find_map(|field| (field.key == key).then_some(field.line))
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.field
            .iter()
            .find_map(|field| (field.key == key).then_some(&field.value))
    }

    pub fn scalar(&self, key: &str) -> Option<&str> {
        self.get(key).and_then(Value::scalar)
    }

    pub fn list(&self, key: &str) -> Option<&[String]> {
        self.get(key).and_then(Value::list)
    }
}

/// A front-matter deviation, naming the file line that broke the grammar.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FrontMatterError {
    pub line: usize,
    pub message: String,
}

/// Split the strict front matter from the record body.
///
/// Returns the front-matter result when a block is present, the body and the
/// 1-based file line the body starts on.
pub fn split_front_matter(
    content: &str,
) -> (Option<Result<FrontMatter, FrontMatterError>>, &str, usize) {
    let Some(rest) = content.strip_prefix("---\n") else {
        return (None, content, 1);
    };
    let mut consumed = "---\n".len();
    let mut line_number = 1;
    let mut front_matter = FrontMatter::default();

    for line in rest.split_inclusive('\n') {
        line_number += 1;
        consumed += line.len();
        let text = line.strip_suffix('\n').unwrap_or(line);
        if text == "---" {
            let body = &content[consumed..];
            return (Some(Ok(front_matter)), body, line_number + 1);
        }
        match parse_front_matter_line(text, line_number) {
            Ok((key, value)) => {
                if front_matter.get(&key).is_some() {
                    return (
                        Some(Err(FrontMatterError {
                            line: line_number,
                            message: format!("duplicate front-matter key `{key}`"),
                        })),
                        &content[consumed..],
                        line_number + 1,
                    );
                }
                front_matter.field.push(Field {
                    key,
                    value,
                    line: line_number,
                });
            }
            Err(error) => {
                return (Some(Err(error)), &content[consumed..], line_number + 1);
            }
        }
    }

    (
        Some(Err(FrontMatterError {
            line: line_number,
            message: "front matter is never closed by a `---` line".to_owned(),
        })),
        "",
        line_number,
    )
}

fn parse_front_matter_line(text: &str, line: usize) -> Result<(String, Value), FrontMatterError> {
    let fail = |message: String| FrontMatterError { line, message };
    if text.trim().is_empty() {
        return Err(fail("blank line inside front matter".to_owned()));
    }
    if text.starts_with(' ') || text.starts_with('\t') {
        return Err(fail(
            "indented front-matter line: no nesting is allowed".to_owned(),
        ));
    }
    if text.starts_with('#') {
        return Err(fail("comment inside front matter".to_owned()));
    }
    if text.starts_with("- ") {
        return Err(fail("block list inside front matter".to_owned()));
    }
    let Some((key, rest)) = text.split_once(':') else {
        return Err(fail("expected `key: value`".to_owned()));
    };
    if !is_front_matter_key(key) {
        return Err(fail(format!(
            "front-matter key `{key}` does not match `^[a-z][a-z0-9_]*$`"
        )));
    }
    let Some(raw) = rest.strip_prefix(' ') else {
        return Err(fail(format!("front-matter key `{key}` has no value")));
    };
    let value = raw.trim();
    if value.is_empty() {
        return Err(fail(format!("front-matter key `{key}` has no value")));
    }
    if let Some(inner) = value.strip_prefix('[') {
        let Some(inner) = inner.strip_suffix(']') else {
            return Err(fail(format!("front-matter list `{key}` is not closed")));
        };
        if inner.trim().is_empty() {
            return Ok((key.to_owned(), Value::List(Vec::new())));
        }
        let mut item = Vec::new();
        for raw_item in inner.split(',') {
            let trimmed = raw_item.trim();
            if trimmed.is_empty() {
                return Err(fail(format!("front-matter list `{key}` has an empty item")));
            }
            if trimmed.starts_with('[') || trimmed.starts_with('"') {
                return Err(fail(format!(
                    "front-matter list `{key}` item `{trimmed}` must be an unquoted scalar"
                )));
            }
            item.push(trimmed.to_owned());
        }
        return Ok((key.to_owned(), Value::List(item)));
    }
    if let Some(inner) = value.strip_prefix('"') {
        let Some(inner) = inner.strip_suffix('"') else {
            return Err(fail(format!("front-matter string `{key}` is not closed")));
        };
        if inner.contains('"') || inner.contains('\\') {
            return Err(fail(format!(
                "front-matter string `{key}` may not contain a quote or an escape"
            )));
        }
        return Ok((key.to_owned(), Value::Scalar(inner.to_owned())));
    }
    if value.contains('"') {
        return Err(fail(format!(
            "front-matter scalar `{key}` must be fully quoted or fully unquoted"
        )));
    }
    Ok((key.to_owned(), Value::Scalar(value.to_owned())))
}

fn is_front_matter_key(key: &str) -> bool {
    let mut character = key.chars();
    character
        .next()
        .is_some_and(|first| first.is_ascii_lowercase())
        && character.all(|rest| rest.is_ascii_lowercase() || rest.is_ascii_digit() || rest == '_')
}

/// The parsed authority role header of a record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoleHeader {
    pub line: usize,
    pub role: String,
    pub side: String,
    pub authority: String,
    pub excludes: Option<String>,
}

const ROLE_LABEL: &str = "**Role / side:**";
const AUTHORITY_LABEL: &str = "**Authority:**";
const EXCLUDES_LABEL: &str = "**Excludes:**";

/// Parse the first `> **Role / side:**` blockquote of a record body.
pub fn parse_role_header(body: &str, body_line: usize) -> Option<RoleHeader> {
    let line = body.lines().collect::<Vec<_>>();
    let start = line
        .iter()
        .position(|text| quoted(text).is_some_and(|text| text.starts_with(ROLE_LABEL)))?;

    let mut field: Vec<(usize, String)> = Vec::new();
    for text in &line[start..] {
        let Some(quoted) = quoted(text) else { break };
        if quoted.starts_with(ROLE_LABEL)
            || quoted.starts_with(AUTHORITY_LABEL)
            || quoted.starts_with(EXCLUDES_LABEL)
        {
            field.push((field.len(), quoted.to_owned()));
        } else if let Some((_, last)) = field.last_mut() {
            last.push(' ');
            last.push_str(quoted);
        } else {
            break;
        }
    }

    let value = |label: &str| {
        field.iter().find_map(|(_, text)| {
            text.strip_prefix(label)
                .map(|value| value.trim().to_owned())
        })
    };
    let role_and_side = value(ROLE_LABEL)?;
    let (role, side) = split_role_and_side(&role_and_side);
    Some(RoleHeader {
        line: body_line + start,
        role,
        side,
        authority: value(AUTHORITY_LABEL).unwrap_or_default(),
        excludes: value(EXCLUDES_LABEL),
    })
}

fn quoted(text: &str) -> Option<&str> {
    let rest = text.strip_prefix('>')?;
    Some(rest.strip_prefix(' ').unwrap_or(rest).trim_end())
}

fn split_role_and_side(value: &str) -> (String, String) {
    let trimmed = value.trim().trim_end_matches('.');
    match trimmed.rfind(" / ") {
        Some(position) => {
            let side = trimmed[position + 3..].trim();
            let side = side.strip_suffix(" side").unwrap_or(side);
            (trimmed[..position].trim().to_owned(), side.to_owned())
        }
        None => (trimmed.to_owned(), String::new()),
    }
}

/// One ATX heading with the stable id used as its anchor.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Heading {
    pub id: String,
    pub title: String,
    pub level: u8,
}

/// Every ATX heading of a Markdown text, in document order.
pub fn headings(markdown: &str) -> Vec<Heading> {
    let mut seen = HashMap::<String, usize>::new();
    let mut heading = Vec::new();
    let mut fenced = false;
    for line in markdown.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            fenced = !fenced;
            continue;
        }
        if fenced {
            continue;
        }
        let level = trimmed
            .chars()
            .take_while(|character| *character == '#')
            .count();
        if !(1..=6).contains(&level) || !trimmed[level..].starts_with(' ') {
            continue;
        }
        let title = trimmed[level + 1..].trim_matches('`').trim().to_owned();
        let base = heading_id(&title);
        let occurrence = seen.entry(base.clone()).or_insert(0);
        // GitHub suffixes the second and later occurrences `-1`, `-2`, …
        let id = if *occurrence == 0 {
            base
        } else {
            format!("{base}-{}", *occurrence)
        };
        *occurrence += 1;
        heading.push(Heading {
            id,
            title,
            level: u8::try_from(level).expect("Markdown heading level fits u8"),
        });
    }
    heading
}

/// The stable anchor id of a heading title, using GitHub's algorithm.
///
/// Lower-case the text, drop every character that is not a letter, digit, space,
/// hyphen or underscore, then replace each remaining space with `-`. Punctuation
/// therefore vanishes without collapsing its neighbours, so `T3 — Prove one kernel`
/// becomes `t3--prove-one-kernel`. A title holding no such character keeps the
/// `section` fallback so Studio never addresses an empty anchor.
pub fn heading_id(title: &str) -> String {
    let id = title
        .chars()
        .flat_map(char::to_lowercase)
        .filter(|character| {
            character.is_alphanumeric()
                || *character == ' '
                || *character == '-'
                || *character == '_'
        })
        .map(|character| if character == ' ' { '-' } else { character })
        .collect::<String>();
    if id.is_empty() {
        "section".to_owned()
    } else {
        id
    }
}

/// Markdown rendered to HTML with raw HTML escaped to text.
pub fn render_markdown(markdown: &str) -> String {
    let parser = Parser::new_ext(markdown, Options::all()).map(|event| match event {
        Event::Html(raw) | Event::InlineHtml(raw) => Event::Text(raw),
        other => other,
    });
    let mut output = String::new();
    html::push_html(&mut output, parser);
    output
}

/// Where one Markdown link points after resolution.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LinkTarget {
    /// A scheme link such as `https:` or `mailto:`; never checked.
    External,
    /// An anchor inside the same record.
    SameFile { anchor: String },
    /// Another repository path with an optional anchor.
    Repository {
        path: String,
        anchor: Option<String>,
    },
}

/// One Markdown link with its source line and resolved target.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Link {
    pub line: usize,
    pub text: String,
    pub target: String,
    pub resolved: LinkTarget,
}

/// Every Markdown link of a body, resolved against the record's own directory.
pub fn links(body: &str, body_line: usize, record_path: &str) -> Vec<Link> {
    let mut link = Vec::new();
    let mut open: Option<(usize, String, String)> = None;
    for (event, range) in Parser::new_ext(body, Options::all()).into_offset_iter() {
        match event {
            Event::Start(Tag::Link { dest_url, .. }) => {
                let line = body_line + body[..range.start].matches('\n').count();
                open = Some((line, dest_url.into_string(), String::new()));
            }
            Event::Text(text) | Event::Code(text) => {
                if let Some((_, _, collected)) = open.as_mut() {
                    collected.push_str(&text);
                }
            }
            Event::End(TagEnd::Link) => {
                if let Some((line, target, text)) = open.take() {
                    let resolved = resolve_link(record_path, &target);
                    link.push(Link {
                        line,
                        text,
                        target,
                        resolved,
                    });
                }
            }
            _ => {}
        }
    }
    link
}

/// Resolve one link target against the directory of the record that holds it.
pub fn resolve_link(record_path: &str, target: &str) -> LinkTarget {
    if has_scheme(target) {
        return LinkTarget::External;
    }
    let (path, anchor) = match target.split_once('#') {
        Some((path, anchor)) => (path, Some(anchor.to_owned())),
        None => (target, None),
    };
    if path.is_empty() {
        return LinkTarget::SameFile {
            anchor: anchor.unwrap_or_default(),
        };
    }
    LinkTarget::Repository {
        path: normalize(home::directory(record_path), path),
        anchor,
    }
}

/// Resolve a front-matter repository reference such as `informs`.
///
/// A front-matter reference is a repository path (conventions section 3), so it is
/// root-relative, unlike a Markdown link which resolves against its own directory.
/// Existence and anchor checking are otherwise identical.
pub fn resolve_reference(value: &str) -> LinkTarget {
    resolve_link("", value)
}

fn has_scheme(target: &str) -> bool {
    let Some(position) = target.find(':') else {
        return false;
    };
    let scheme = &target[..position];
    !scheme.is_empty()
        && scheme.starts_with(|first: char| first.is_ascii_alphabetic())
        && scheme
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || "+-.".contains(character))
}

fn normalize(directory: &str, relative: &str) -> String {
    let mut segment: Vec<&str> = if relative.starts_with('/') {
        Vec::new()
    } else {
        directory
            .split('/')
            .filter(|part| !part.is_empty())
            .collect()
    };
    for part in relative.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                if segment.pop().is_none() {
                    segment.push("..");
                }
            }
            other => segment.push(other),
        }
    }
    segment.join("/")
}

/// Whether a record is Markdown prose or a governed SQL migration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Markdown,
    Sql,
}

/// One parsed governed record.
#[derive(Clone, Debug)]
pub struct Record {
    pub path: String,
    pub home: Option<&'static Home>,
    pub kind: Kind,
    pub title: String,
    pub body: String,
    pub body_line: usize,
    pub front_matter: FrontMatter,
    pub front_matter_error: Option<FrontMatterError>,
    pub role_header: Option<RoleHeader>,
    pub heading: Vec<Heading>,
    pub link: Vec<Link>,
    /// Excluded from the metadata sweeps (front matter, role header, index links).
    pub metadata_frozen: bool,
    /// Excluded from the link and anchor sweeps.
    pub link_frozen: bool,
}

impl Record {
    /// The record body rendered to HTML with raw HTML escaped.
    pub fn html(&self) -> String {
        match self.kind {
            Kind::Markdown => render_markdown(&self.body),
            Kind::Sql => render_markdown(&format!("```sql\n{}\n```\n", self.body.trim_end())),
        }
    }

    /// The home id, or `unmatched` when no home claims this record.
    pub fn home_id(&self) -> &'static str {
        self.home.map_or("unmatched", |home| home.id)
    }

    pub fn heading_id_exists(&self, anchor: &str) -> bool {
        self.heading.iter().any(|heading| heading.id == anchor)
    }
}

fn parse_record(path: String, home: Option<&'static Home>, content: &str) -> Record {
    let kind = if path.ends_with(".sql") {
        Kind::Sql
    } else {
        Kind::Markdown
    };
    if kind == Kind::Sql {
        return Record {
            title: home::file_name(&path).to_owned(),
            body: content.to_owned(),
            body_line: 1,
            front_matter: FrontMatter::default(),
            front_matter_error: None,
            role_header: None,
            heading: Vec::new(),
            link: Vec::new(),
            metadata_frozen: false,
            link_frozen: false,
            path,
            home,
            kind,
        };
    }

    let unchecked = home.is_some_and(|home| home.vocabulary == Vocabulary::Unchecked);
    let (parsed, body, body_line) = if unchecked {
        skip_unchecked_front_matter(content)
    } else {
        split_front_matter(content)
    };
    let (front_matter, front_matter_error) = match parsed {
        Some(Ok(front_matter)) => (front_matter, None),
        Some(Err(error)) => (FrontMatter::default(), Some(error)),
        None => (FrontMatter::default(), None),
    };
    let heading = headings(body);
    let title = heading
        .iter()
        .find(|heading| heading.level == 1)
        .map(|heading| heading.title.clone())
        .unwrap_or_else(|| home::file_name(&path).trim_end_matches(".md").to_owned());
    let (metadata_frozen, link_frozen) = freeze(home, &front_matter);

    Record {
        title,
        role_header: parse_role_header(body, body_line),
        link: links(body, body_line, &path),
        body: body.to_owned(),
        body_line,
        front_matter,
        front_matter_error,
        heading,
        metadata_frozen,
        link_frozen,
        path,
        home,
        kind,
    }
}

/// Skill assets carry placeholder front matter, so their block is skipped, never parsed.
fn skip_unchecked_front_matter(
    content: &str,
) -> (Option<Result<FrontMatter, FrontMatterError>>, &str, usize) {
    let Some(rest) = content.strip_prefix("---\n") else {
        return (None, content, 1);
    };
    let mut consumed = "---\n".len();
    let mut line_number = 1;
    for line in rest.split_inclusive('\n') {
        line_number += 1;
        consumed += line.len();
        if line.strip_suffix('\n').unwrap_or(line) == "---" {
            return (None, &content[consumed..], line_number + 1);
        }
    }
    (None, content, 1)
}

fn freeze(home: Option<&'static Home>, front_matter: &FrontMatter) -> (bool, bool) {
    match home.map(|home| home.frozen) {
        Some(Frozen::Yes) => (true, true),
        Some(Frozen::Link) => (false, true),
        Some(Frozen::PlanStatus) => {
            let frozen = matches!(
                front_matter.scalar("status"),
                Some("complete" | "superseded" | "dropped")
            );
            (frozen, frozen)
        }
        // Resolved from the owning plan once every record is loaded.
        Some(Frozen::OwningPlan) => (false, false),
        Some(Frozen::No) | None => (false, false),
    }
}

/// One record that links to another record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Backlink {
    pub path: String,
    pub line: usize,
    pub anchor: Option<String>,
}

/// One documentation-convention violation, naming the offending file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LintFinding {
    pub path: String,
    pub line: Option<usize>,
    pub rule: &'static str,
    pub message: String,
}

pub const RULE_UNMATCHED: &str = "unmatched-file";
pub const RULE_ROLE_HEADER: &str = "role-header";
pub const RULE_FRONT_MATTER: &str = "front-matter";
pub const RULE_LINK: &str = "link";
pub const RULE_ANCHOR: &str = "anchor";
pub const RULE_STORAGE_TABLE: &str = "storage-table";
pub const RULE_INDEX_LINK: &str = "index-link";

/// The whole governed repository, parsed once.
#[derive(Debug)]
pub struct Repository {
    root: PathBuf,
    record: Vec<Record>,
    position: BTreeMap<String, usize>,
    backlink: BTreeMap<String, Vec<Backlink>>,
}

impl Repository {
    /// Scan, read and parse every governed record under `repository_root`.
    pub fn load(repository_root: impl AsRef<Path>) -> Result<Self, ScanError> {
        let root = repository_root.as_ref().to_path_buf();
        let mut record = Vec::new();
        for scanned in home::scan(&root)? {
            let content = home::read(&root, &scanned.path)?;
            record.push(parse_record(scanned.path, scanned.home, &content));
        }

        let plan_frozen: BTreeMap<String, bool> = record
            .iter()
            .filter(|record| record.home_id() == "plan")
            .map(|record| {
                (
                    home::directory(&record.path).to_owned(),
                    record.metadata_frozen,
                )
            })
            .collect();
        for record in &mut record {
            if record.home.map(|home| home.frozen) == Some(Frozen::OwningPlan) {
                let owning = plan_frozen
                    .iter()
                    .find(|(directory, _)| record.path.starts_with(&format!("{directory}/")))
                    .map(|(_, frozen)| *frozen)
                    .unwrap_or(false);
                record.metadata_frozen = owning;
                record.link_frozen = owning;
            }
        }

        let position = record
            .iter()
            .enumerate()
            .map(|(index, record)| (record.path.clone(), index))
            .collect::<BTreeMap<_, _>>();

        let mut backlink: BTreeMap<String, Vec<Backlink>> = BTreeMap::new();
        for source in &record {
            if source.metadata_frozen {
                continue;
            }
            for link in &source.link {
                let LinkTarget::Repository { path, anchor } = &link.resolved else {
                    continue;
                };
                let Some(index) = position.get(path) else {
                    continue;
                };
                if record[*index].metadata_frozen || record[*index].path == source.path {
                    continue;
                }
                backlink.entry(path.clone()).or_default().push(Backlink {
                    path: source.path.clone(),
                    line: link.line,
                    anchor: anchor.clone(),
                });
            }
        }

        Ok(Self {
            root,
            record,
            position,
            backlink,
        })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn record(&self) -> &[Record] {
        &self.record
    }

    pub fn get(&self, path: &str) -> Option<&Record> {
        self.position.get(path).map(|index| &self.record[*index])
    }

    /// Every record of one home, in path order.
    pub fn in_home(&self, home_id: &str) -> Vec<&Record> {
        self.record
            .iter()
            .filter(|record| record.home_id() == home_id)
            .collect()
    }

    /// Records that link to `path`, frozen sources excluded.
    pub fn backlink(&self, path: &str) -> &[Backlink] {
        self.backlink.get(path).map_or(&[], Vec::as_slice)
    }

    /// Every documentation-convention violation in the live sweep scope.
    pub fn lint(&self) -> Vec<LintFinding> {
        let mut finding = Vec::new();
        for record in &self.record {
            self.lint_record(record, &mut finding);
        }
        finding.extend(model::lint_storage_table(self));
        self.lint_index_link(&mut finding);
        finding.sort_by(|left, right| {
            (left.path.as_str(), left.line, left.rule).cmp(&(
                right.path.as_str(),
                right.line,
                right.rule,
            ))
        });
        finding
    }

    fn lint_record(&self, record: &Record, finding: &mut Vec<LintFinding>) {
        let Some(home) = record.home else {
            finding.push(LintFinding {
                path: record.path.clone(),
                line: None,
                rule: RULE_UNMATCHED,
                message: "Markdown under a governed root matches no home".to_owned(),
            });
            return;
        };

        if !record.metadata_frozen {
            let required = match home.role_header {
                RoleHeaderRule::Required | RoleHeaderRule::RequiredWhenLive => true,
                RoleHeaderRule::Optional => false,
            };
            if required && record.role_header.is_none() {
                finding.push(LintFinding {
                    path: record.path.clone(),
                    line: Some(record.body_line),
                    rule: RULE_ROLE_HEADER,
                    message: format!("home `{}` requires a `> **Role / side:**` header", home.id),
                });
            }
            self.lint_front_matter(record, home, finding);
        }

        if record.link_frozen {
            return;
        }
        self.lint_front_matter_reference(record, home, finding);
        for link in &record.link {
            let LinkTarget::Repository { path, anchor } = &link.resolved else {
                if let LinkTarget::SameFile { anchor } = &link.resolved
                    && !anchor.is_empty()
                    && !record.heading_id_exists(anchor)
                {
                    finding.push(LintFinding {
                        path: record.path.clone(),
                        line: Some(link.line),
                        rule: RULE_ANCHOR,
                        message: format!("`#{anchor}` is not a heading of this record"),
                    });
                }
                continue;
            };
            let Some(target) = self.get(path) else {
                if self.root.join(path).exists() {
                    continue;
                }
                finding.push(LintFinding {
                    path: record.path.clone(),
                    line: Some(link.line),
                    rule: RULE_LINK,
                    message: format!("`{}` resolves to missing `{path}`", link.target),
                });
                continue;
            };
            if let Some(anchor) = anchor
                && !anchor.is_empty()
                && !target.heading_id_exists(anchor)
            {
                finding.push(LintFinding {
                    path: record.path.clone(),
                    line: Some(link.line),
                    rule: RULE_ANCHOR,
                    message: format!("`#{anchor}` is not a heading of `{path}`"),
                });
            }
        }
    }

    fn lint_front_matter(
        &self,
        record: &Record,
        home: &'static Home,
        finding: &mut Vec<LintFinding>,
    ) {
        if let Some(error) = &record.front_matter_error {
            finding.push(LintFinding {
                path: record.path.clone(),
                line: Some(error.line),
                rule: RULE_FRONT_MATTER,
                message: error.message.clone(),
            });
            return;
        }
        for problem in validate_vocabulary(home.vocabulary, &record.front_matter) {
            finding.push(LintFinding {
                path: record.path.clone(),
                line: Some(1),
                rule: RULE_FRONT_MATTER,
                message: problem,
            });
        }
    }

    /// Front-matter values that name a repository record resolve exactly like a link.
    fn lint_front_matter_reference(
        &self,
        record: &Record,
        home: &'static Home,
        finding: &mut Vec<LintFinding>,
    ) {
        for key in reference_key(home.vocabulary) {
            let Some(value) = record.front_matter.scalar(key) else {
                continue;
            };
            let line = record.front_matter.line(key);
            let (target, anchor) = match resolve_reference(value) {
                LinkTarget::External => continue,
                LinkTarget::SameFile { anchor } => (record, Some(anchor)),
                LinkTarget::Repository { path, anchor } => {
                    let Some(target) = self.get(&path) else {
                        finding.push(LintFinding {
                            path: record.path.clone(),
                            line,
                            rule: RULE_LINK,
                            message: format!("front-matter `{key}` names missing `{path}`"),
                        });
                        continue;
                    };
                    (target, anchor)
                }
            };
            if let Some(anchor) = anchor
                && !anchor.is_empty()
                && !target.heading_id_exists(&anchor)
            {
                finding.push(LintFinding {
                    path: record.path.clone(),
                    line,
                    rule: RULE_ANCHOR,
                    message: format!(
                        "front-matter `{key}` `#{anchor}` is not a heading of `{}`",
                        target.path
                    ),
                });
            }
        }
    }

    fn lint_index_link(&self, finding: &mut Vec<LintFinding>) {
        for record in &self.record {
            let Some(home) = record.home else { continue };
            if record.metadata_frozen {
                continue;
            }
            let index_path = match home.index {
                Index::None => continue,
                Index::Fixed(path) => path.to_owned(),
                Index::ParentReadme => {
                    let directory = home::directory(&record.path);
                    format!("{}/README.md", home::directory(directory))
                }
            };
            let Some(index) = self.get(&index_path) else {
                finding.push(LintFinding {
                    path: record.path.clone(),
                    line: None,
                    rule: RULE_INDEX_LINK,
                    message: format!("index `{index_path}` of home `{}` is missing", home.id),
                });
                continue;
            };
            let linked = index.link.iter().any(|link| {
                matches!(&link.resolved, LinkTarget::Repository { path, .. } if *path == record.path)
            });
            if !linked {
                finding.push(LintFinding {
                    path: record.path.clone(),
                    line: None,
                    rule: RULE_INDEX_LINK,
                    message: format!("`{index_path}` does not link this `{}` record", home.id),
                });
            }
        }
    }
}

/// Required keys, allowed keys and allowed values per home vocabulary.
fn vocabulary_rule(vocabulary: Vocabulary) -> Vec<(&'static str, bool, &'static [&'static str])> {
    const FREE: &[&str] = &[];
    match vocabulary {
        Vocabulary::None | Vocabulary::Unchecked => Vec::new(),
        Vocabulary::Model => vec![
            ("storage_table", true, FREE),
            (
                "kind",
                false,
                &[
                    "subject",
                    "role",
                    "seam",
                    "history",
                    "state",
                    "provenance",
                    "participant",
                ],
            ),
        ],
        Vocabulary::Concept => vec![("status", true, &["active", "live", "retained", "throwaway"])],
        Vocabulary::Research => vec![
            (
                "status",
                true,
                &["pending", "load-bearing", "historical", "superseded"],
            ),
            ("era", true, FREE),
        ],
        Vocabulary::Evidence => vec![("status", true, FREE)],
        Vocabulary::LabTrack => vec![(
            "status",
            true,
            &["active", "kept", "superseded", "discarded"],
        )],
        Vocabulary::LabExperiment => vec![
            ("question", true, FREE),
            (
                "verdict",
                true,
                &["supported", "refuted", "inconclusive", "pending"],
            ),
            (
                "status",
                true,
                &["active", "kept", "superseded", "discarded"],
            ),
            ("real_seam", true, FREE),
            ("simulated_seam", true, FREE),
            ("informs", true, FREE),
        ],
        Vocabulary::Plan => vec![
            (
                "status",
                true,
                &["draft", "active", "complete", "superseded", "dropped"],
            ),
            ("created_at", true, FREE),
            ("updated_at", true, FREE),
            ("accepted_at", true, FREE),
            ("completed_at", true, FREE),
            ("backlog_item", false, FREE),
            ("dropped_at", false, FREE),
            ("dropped_reason", false, FREE),
        ],
        Vocabulary::SkillPresence => vec![("name", true, FREE), ("description", true, FREE)],
    }
}

/// Keys whose scalar value is a repository reference and is resolved like a link.
fn reference_key(vocabulary: Vocabulary) -> &'static [&'static str] {
    match vocabulary {
        Vocabulary::LabExperiment => &["informs"],
        Vocabulary::Plan => &["backlog_item"],
        _ => &[],
    }
}

/// Keys whose value must be an inline list rather than a scalar.
fn list_key(vocabulary: Vocabulary) -> &'static [&'static str] {
    match vocabulary {
        Vocabulary::Model => &["storage_table"],
        Vocabulary::LabExperiment => &["real_seam", "simulated_seam"],
        _ => &[],
    }
}

fn validate_vocabulary(vocabulary: Vocabulary, front_matter: &FrontMatter) -> Vec<String> {
    let rule = vocabulary_rule(vocabulary);
    if rule.is_empty() {
        return Vec::new();
    }
    let mut problem = Vec::new();
    for (key, required, allowed) in &rule {
        let Some(value) = front_matter.get(key) else {
            if *required {
                problem.push(format!("required front-matter key `{key}` is missing"));
            }
            continue;
        };
        let wants_list = list_key(vocabulary).contains(key);
        match (wants_list, value) {
            (true, Value::Scalar(_)) => {
                problem.push(format!("front-matter key `{key}` must be an inline list"));
                continue;
            }
            (false, Value::List(_)) => {
                problem.push(format!("front-matter key `{key}` must be a scalar"));
                continue;
            }
            _ => {}
        }
        if let (false, Value::Scalar(scalar)) = (allowed.is_empty(), value)
            && !allowed.contains(&scalar.as_str())
        {
            problem.push(format!(
                "front-matter key `{key}` value `{scalar}` is not one of {}",
                allowed.join(", ")
            ));
        }
    }
    for field in front_matter.field() {
        if !rule.iter().any(|(allowed, _, _)| *allowed == field.key) {
            problem.push(format!(
                "unknown front-matter key `{}` for this home",
                field.key
            ));
        }
    }
    problem
}

#[cfg(test)]
mod tests {
    use super::*;

    fn front_matter(content: &str) -> Result<FrontMatter, FrontMatterError> {
        split_front_matter(content)
            .0
            .expect("the fixture opens a front-matter block")
    }

    #[test]
    fn front_matter_parses_scalars_quoted_strings_and_inline_lists() {
        let (parsed, body, line) = split_front_matter(
            "---\nstatus: live\nera: \"July scene-claim\"\nstorage_table: [entity, entity_location]\nempty: []\naccepted_at: null\n---\n\n# Title\n",
        );
        let parsed = parsed.expect("block present").expect("valid front matter");

        assert_eq!(parsed.scalar("status"), Some("live"));
        assert_eq!(parsed.scalar("era"), Some("July scene-claim"));
        assert_eq!(
            parsed.list("storage_table"),
            Some(["entity".to_owned(), "entity_location".to_owned()].as_slice())
        );
        assert_eq!(parsed.list("empty"), Some([].as_slice()));
        assert_eq!(parsed.scalar("accepted_at"), Some("null"));
        assert_eq!(body, "\n# Title\n");
        assert_eq!(line, 8);
    }

    #[test]
    fn a_record_without_a_block_keeps_its_whole_body() {
        let (parsed, body, line) = split_front_matter("# Title\n\nText\n");

        assert!(parsed.is_none());
        assert_eq!(body, "# Title\n\nText\n");
        assert_eq!(line, 1);
    }

    #[test]
    fn every_front_matter_grammar_deviation_names_its_line() {
        let case = [
            ("---\nStatus: live\n---\n", 2, "does not match"),
            ("---\nstatus live\n---\n", 2, "expected `key: value`"),
            ("---\nstatus:\n---\n", 2, "has no value"),
            ("---\nstatus: live\nstatus: active\n---\n", 3, "duplicate"),
            ("---\nstatus: live\n\nera: x\n---\n", 3, "blank line"),
            ("---\nstatus: live\n  nested: x\n---\n", 3, "no nesting"),
            ("---\n# comment\n---\n", 2, "comment"),
            ("---\n- item\n---\n", 2, "block list"),
            ("---\nstorage_table: [entity\n---\n", 2, "is not closed"),
            ("---\nstorage_table: [entity, ]\n---\n", 2, "empty item"),
            (
                "---\nstorage_table: [\"entity\"]\n---\n",
                2,
                "unquoted scalar",
            ),
            ("---\nera: \"July\n---\n", 2, "is not closed"),
            ("---\nera: \"Ju\"ly\"\n---\n", 2, "quote or an escape"),
            ("---\nera: Ju\"ly\n---\n", 2, "fully quoted"),
            ("---\nstatus: live\n", 2, "never closed"),
        ];

        for (content, line, needle) in case {
            let error = front_matter(content).expect_err(&format!("{content:?} must fail"));
            assert_eq!(error.line, line, "wrong line for {content:?}");
            assert!(
                error.message.contains(needle),
                "{:?} reported {:?}, expected {needle}",
                content,
                error.message
            );
        }
    }

    #[test]
    fn headings_use_github_anchor_ids_and_ignore_fenced_code() {
        let heading = headings(
            "# One thing\n## Repeated\n## Repeated\n## Repeated\n#### Exact `field`\n### T3 — Prove one kernel\n### get_world, the read\n```rust\n# not a heading\n```\n",
        );
        let reference = heading
            .iter()
            .map(|heading| (heading.id.as_str(), heading.level))
            .collect::<Vec<_>>();

        assert_eq!(
            reference,
            vec![
                ("one-thing", 1),
                ("repeated", 2),
                ("repeated-1", 2),
                ("repeated-2", 2),
                ("exact-field", 4),
                ("t3--prove-one-kernel", 3),
                ("get_world-the-read", 3),
            ]
        );
    }

    #[test]
    fn a_heading_id_drops_punctuation_without_collapsing_its_neighbours() {
        assert_eq!(heading_id("Open questions"), "open-questions");
        assert_eq!(heading_id("T3 — Prove one kernel"), "t3--prove-one-kernel");
        assert_eq!(heading_id("`get_world`"), "get_world");
        assert_eq!(
            heading_id("Property, Trait and Activity"),
            "property-trait-and-activity"
        );
        assert_eq!(heading_id("Investigation attempt"), "investigation-attempt");
        assert_eq!(heading_id("HTTP/MCP parity"), "httpmcp-parity");
        assert_eq!(heading_id("***"), "section");
    }

    #[test]
    fn the_role_header_splits_role_side_authority_and_excludes() {
        let header = parse_role_header(
            "# Title\n\n> **Role / side:** proportional build plan / development side.\n> **Authority:** owns the execution state\n> and its evidence.\n> **Excludes:** game behavior.\n",
            1,
        )
        .expect("role header present");

        assert_eq!(header.line, 3);
        assert_eq!(header.role, "proportional build plan");
        assert_eq!(header.side, "development");
        assert_eq!(
            header.authority,
            "owns the execution state and its evidence."
        );
        assert_eq!(header.excludes.as_deref(), Some("game behavior."));
    }

    #[test]
    fn a_bridge_side_header_keeps_its_own_side_wording() {
        let header = parse_role_header(
            "> **Role / side:** Trait evidence history / evidence bridge.\n> **Authority:** x\n",
            1,
        )
        .expect("role header present");

        assert_eq!(header.role, "Trait evidence history");
        assert_eq!(header.side, "evidence bridge");
    }

    #[test]
    fn links_resolve_relative_parents_anchors_and_ignore_scheme_targets() {
        let body = "[a](../../../game/docs/domain.md#activity) [b](#local) [c](https://example.test/x) [d](items/one.md) [e](./two.md)\n";
        let link = links(body, 1, "dev/docs/concept/discovery.md");
        let resolved = link
            .iter()
            .map(|link| link.resolved.clone())
            .collect::<Vec<_>>();

        assert_eq!(
            resolved,
            vec![
                LinkTarget::Repository {
                    path: "game/docs/domain.md".to_owned(),
                    anchor: Some("activity".to_owned())
                },
                LinkTarget::SameFile {
                    anchor: "local".to_owned()
                },
                LinkTarget::External,
                LinkTarget::Repository {
                    path: "dev/docs/concept/items/one.md".to_owned(),
                    anchor: None
                },
                LinkTarget::Repository {
                    path: "dev/docs/concept/two.md".to_owned(),
                    anchor: None
                },
            ]
        );
        assert_eq!(link[0].text, "a");
        assert!(link.iter().all(|link| link.line == 1));
    }

    #[test]
    fn a_link_inside_a_code_span_is_not_a_link() {
        assert!(links("`[a](b.md)`\n", 1, "docs/x.md").is_empty());
    }

    #[test]
    fn raw_html_is_escaped_in_rendered_records() {
        assert!(render_markdown("<script>x</script>\n").contains("&lt;script&gt;"));
    }

    fn write(root: &Path, path: &str, content: &str) {
        let full = root.join(path);
        std::fs::create_dir_all(full.parent().expect("record has a parent")).expect("directory");
        std::fs::write(full, content).expect("record");
    }

    fn fixture(name: &str) -> PathBuf {
        let root = crate::test_directory(name);
        write(
            &root,
            "dev/docs/README.md",
            "# Constitution\n\n> **Role / side:** placement constitution / development side.\n> **Authority:** placement.\n",
        );
        write(
            &root,
            "dev/docs/concept/README.md",
            "# Concept\n\n> **Role / side:** concept index / development side.\n> **Authority:** navigation.\n\n- [Discovery](discovery.md)\n",
        );
        write(
            &root,
            "dev/docs/concept/discovery.md",
            "---\nstatus: live\n---\n\n# Discovery\n\n> **Role / side:** live concept exploration / development side.\n> **Authority:** discovery rationale.\n\n## Open decisions\n\nQ1 stays open.\n",
        );
        root
    }

    #[test]
    fn a_clean_fixture_repository_lints_without_findings() {
        let root = fixture("record-clean");
        let repository = Repository::load(&root).expect("fixture loads");

        assert_eq!(repository.record().len(), 3);
        assert_eq!(repository.lint(), Vec::new());
        assert_eq!(
            repository
                .get("dev/docs/concept/discovery.md")
                .expect("record")
                .title,
            "Discovery"
        );
        assert_eq!(
            repository.backlink("dev/docs/concept/discovery.md")[0].path,
            "dev/docs/concept/README.md"
        );
    }

    #[test]
    fn each_lint_rule_names_the_offending_file() {
        let root = fixture("record-findings");
        write(&root, "dev/docs/stray.md", "# Stray\n");
        write(
            &root,
            "dev/docs/concept/knowledge.md",
            "---\nstatus: unknown-value\n---\n\n# Knowledge\n\n[gone](missing.md)\n[bad anchor](discovery.md#nowhere)\n",
        );
        write(
            &root,
            "dev/docs/concept/spatial.md",
            "---\nstatus: live\nextra: x\n---\n\n# Spatial\n\n> **Role / side:** live concept exploration / development side.\n> **Authority:** spatial rationale.\n",
        );
        let repository = Repository::load(&root).expect("fixture loads");
        let finding = repository.lint();
        let by_rule = |rule: &str| {
            finding
                .iter()
                .filter(|finding| finding.rule == rule)
                .map(|finding| finding.path.as_str())
                .collect::<Vec<_>>()
        };

        assert_eq!(by_rule(RULE_UNMATCHED), vec!["dev/docs/stray.md"]);
        assert_eq!(
            by_rule(RULE_ROLE_HEADER),
            vec!["dev/docs/concept/knowledge.md"]
        );
        assert_eq!(by_rule(RULE_LINK), vec!["dev/docs/concept/knowledge.md"]);
        assert_eq!(by_rule(RULE_ANCHOR), vec!["dev/docs/concept/knowledge.md"]);
        assert_eq!(
            by_rule(RULE_FRONT_MATTER),
            vec![
                "dev/docs/concept/knowledge.md",
                "dev/docs/concept/spatial.md"
            ]
        );
        assert_eq!(
            by_rule(RULE_INDEX_LINK),
            vec![
                "dev/docs/concept/knowledge.md",
                "dev/docs/concept/spatial.md"
            ]
        );
        assert!(finding.iter().all(|finding| !finding.message.is_empty()));
    }

    #[test]
    fn front_matter_references_are_checked_as_root_relative_repository_paths() {
        let experiment = |informs: &str| {
            format!(
                "---\nquestion: Can the kernel hold?\nverdict: supported\nstatus: kept\nreal_seam: [Rust compiler]\nsimulated_seam: []\ninforms: {informs}\n---\n\n# Experiment\n\n> **Role / side:** retained experiment / development side.\n> **Authority:** this fixture and its verdict.\n"
            )
        };
        let root = crate::test_directory("record-informs");
        write(
            &root,
            "dev/docs/README.md",
            "# Constitution\n\n> **Role / side:** placement constitution / development side.\n> **Authority:** placement.\n",
        );
        write(
            &root,
            "dev/plans/20260816-lab/plan.md",
            "---\nstatus: draft\ncreated_at: \"2026-08-16T15:34:10+02:00\"\nupdated_at: \"2026-08-17T11:47:38+02:00\"\naccepted_at: null\ncompleted_at: null\nbacklog_item: dev/backlog/items/multiplayer.md\n---\n\n# Lab plan\n\n> **Role / side:** proportional build plan / development side.\n> **Authority:** execution state.\n\n### T3 — Prove one unified semantic change kernel\n\nText.\n",
        );
        write(
            &root,
            "dev/backlog/items/multiplayer.md",
            "# Multiplayer foundation\n\n> **Role / side:** forward-planning item / development side.\n> **Authority:** this outcome's dependencies.\n",
        );
        write(
            &root,
            "dev/plans/20260817-orphan/plan.md",
            "---\nstatus: active\ncreated_at: \"2026-08-17T16:16:15+02:00\"\nupdated_at: \"2026-08-17T18:05:00+02:00\"\naccepted_at: null\ncompleted_at: null\nbacklog_item: dev/backlog/items/gone.md\n---\n\n# Orphan plan\n\n> **Role / side:** proportional build plan / development side.\n> **Authority:** execution state.\n",
        );
        write(
            &root,
            "dev/lab/README.md",
            "# Lab\n\n> **Role / side:** workbench / development side.\n> **Authority:** boundaries.\n\n- [Multiplayer](multiplayer/README.md)\n",
        );
        write(
            &root,
            "dev/lab/multiplayer/README.md",
            "---\nstatus: active\n---\n\n# Multiplayer lab\n\n> **Role / side:** experiment track / development side.\n> **Authority:** track index.\n\n- [01](01-good/README.md)\n- [02](02-bad-anchor/README.md)\n- [03](03-missing-file/README.md)\n",
        );
        write(
            &root,
            "dev/lab/multiplayer/01-good/README.md",
            &experiment(
                "dev/plans/20260816-lab/plan.md#t3--prove-one-unified-semantic-change-kernel",
            ),
        );
        write(
            &root,
            "dev/lab/multiplayer/02-bad-anchor/README.md",
            &experiment(
                "dev/plans/20260816-lab/plan.md#t3-prove-one-unified-semantic-change-kernel",
            ),
        );
        write(
            &root,
            "dev/lab/multiplayer/03-missing-file/README.md",
            &experiment("dev/plans/20260816-gone/plan.md#t3"),
        );

        let repository = Repository::load(&root).expect("fixture loads");
        let finding = repository
            .lint()
            .into_iter()
            .map(|finding| (finding.path, finding.line, finding.rule, finding.message))
            .collect::<Vec<_>>();

        assert_eq!(
            finding,
            vec![
                (
                    "dev/lab/multiplayer/02-bad-anchor/README.md".to_owned(),
                    Some(7),
                    RULE_ANCHOR,
                    "front-matter `informs` `#t3-prove-one-unified-semantic-change-kernel` is not a heading of `dev/plans/20260816-lab/plan.md`".to_owned()
                ),
                (
                    "dev/lab/multiplayer/03-missing-file/README.md".to_owned(),
                    Some(7),
                    RULE_LINK,
                    "front-matter `informs` names missing `dev/plans/20260816-gone/plan.md`".to_owned()
                ),
                (
                    "dev/plans/20260817-orphan/plan.md".to_owned(),
                    Some(7),
                    RULE_LINK,
                    "front-matter `backlog_item` names missing `dev/backlog/items/gone.md`"
                        .to_owned()
                ),
            ],
            "a resolving reference stays silent; a missing path and a wrong anchor are reported"
        );
    }

    #[test]
    fn frozen_homes_are_excluded_from_link_and_metadata_sweeps() {
        let root = fixture("record-frozen");
        write(
            &root,
            "dev/docs/concept/archive/00-vision.md",
            "> **Archived:** old.\n\n# Vision\n\n[gone](missing.md)\n",
        );
        write(
            &root,
            "dev/plans/20260810-a/plan.md",
            "---\nstatus: complete\ncreated_at: \"2026-08-10T15:58:46+02:00\"\nlegacy_key: x\n---\n\n# Complete plan\n\n[gone](missing.md)\n",
        );
        write(
            &root,
            "dev/plans/20260810-a/move-map/t2.md",
            "# Fragment\n\n[gone](missing.md)\n",
        );
        write(
            &root,
            "dev/plans/20260816-b/plan.md",
            "---\nstatus: draft\ncreated_at: \"2026-08-16T15:34:10+02:00\"\nupdated_at: \"2026-08-17T11:47:38+02:00\"\naccepted_at: null\ncompleted_at: null\n---\n\n# Draft plan\n\n> **Role / side:** proportional build plan / development side.\n> **Authority:** execution state.\n\n[gone](missing.md)\n",
        );
        write(
            &root,
            "dev/plans/20260811-c/plan.md",
            "---\nstatus: dropped\ncreated_at: 2026-08-11T08:31:45+02:00\ndropped_at: 2026-08-11T08:56:52+02:00\ndropped_reason: The User rejected a human administrator flow.\n---\n\n# Dropped plan\n\n[gone](missing.md)\n",
        );
        let repository = Repository::load(&root).expect("fixture loads");
        let finding = repository.lint();

        assert!(
            finding
                .iter()
                .all(|finding| !finding.path.starts_with("dev/plans/20260810-a")
                    && !finding.path.starts_with("dev/plans/20260811-c")
                    && !finding.path.starts_with("dev/docs/concept/archive")),
            "frozen records must not be linted: {finding:?}"
        );
        assert!(
            repository
                .get("dev/plans/20260811-c/plan.md")
                .expect("the dropped plan is projected")
                .metadata_frozen,
            "a dropped plan is frozen history and keeps its role header optional"
        );
        assert_eq!(
            finding
                .iter()
                .filter(|finding| finding.path == "dev/plans/20260816-b/plan.md")
                .map(|finding| finding.rule)
                .collect::<Vec<_>>(),
            vec![RULE_LINK]
        );
    }

    #[test]
    fn the_plan_vocabulary_accepts_every_recorded_status_and_its_drop_provenance() {
        let front_matter = |content: &str| {
            split_front_matter(content)
                .0
                .expect("block present")
                .expect("valid front matter")
        };

        for status in ["draft", "active", "complete", "superseded", "dropped"] {
            let parsed = front_matter(&format!(
                "---\nstatus: {status}\ncreated_at: \"2026-08-11T08:31:45+02:00\"\nupdated_at: \"2026-08-11T08:56:52+02:00\"\naccepted_at: null\ncompleted_at: null\n---\n"
            ));
            assert_eq!(
                validate_vocabulary(Vocabulary::Plan, &parsed),
                Vec::<String>::new(),
                "plan status {status} must validate"
            );
        }

        let dropped = front_matter(
            "---\nstatus: dropped\ncreated_at: \"2026-08-11T08:31:45+02:00\"\nupdated_at: \"2026-08-11T08:56:52+02:00\"\naccepted_at: null\ncompleted_at: null\ndropped_at: \"2026-08-11T08:56:52+02:00\"\ndropped_reason: The User rejected a human administrator flow.\n---\n",
        );
        assert_eq!(
            validate_vocabulary(Vocabulary::Plan, &dropped),
            Vec::<String>::new(),
            "`dropped_at` and `dropped_reason` are optional plan keys"
        );

        let unknown = front_matter(
            "---\nstatus: shelved\ncreated_at: \"2026-08-11T08:31:45+02:00\"\nupdated_at: \"2026-08-11T08:56:52+02:00\"\naccepted_at: null\ncompleted_at: null\nspend_authorized_at: \"2026-08-14T13:02:10+02:00\"\n---\n",
        );
        assert_eq!(
            validate_vocabulary(Vocabulary::Plan, &unknown),
            vec![
                "front-matter key `status` value `shelved` is not one of draft, active, complete, superseded, dropped".to_owned(),
                "unknown front-matter key `spend_authorized_at` for this home".to_owned(),
            ]
        );
    }

    #[test]
    fn a_decision_register_keeps_metadata_lint_but_loses_link_lint() {
        let root = fixture("record-register");
        write(
            &root,
            "dev/docs/concept/log/2026-08.md",
            "# August 2026\n\n[gone](missing.md)\n",
        );
        let repository = Repository::load(&root).expect("fixture loads");
        let rule = repository
            .lint()
            .into_iter()
            .filter(|finding| finding.path == "dev/docs/concept/log/2026-08.md")
            .map(|finding| finding.rule)
            .collect::<Vec<_>>();

        assert_eq!(rule, vec![RULE_ROLE_HEADER]);
    }
}
