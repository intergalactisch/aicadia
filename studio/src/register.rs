//! The append-only decision register parsed from `dev/docs/concept/log/YYYY-MM.md`.

use std::collections::HashMap;

use super::record::{Link, Record, heading_id, links};

/// The tag given to a list item whose leading `tag:` phrase cannot be read.
pub const UNKNOWN_TAG: &str = "unknown";

/// The longest leading phrase still accepted as a tag.
const MAX_TAG_BYTES: usize = 120;

/// One recorded choice under one date and topic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub date: String,
    pub topic: String,
    pub topic_id: String,
    pub sub_topic: Option<String>,
    pub sub_topic_id: Option<String>,
    /// `YYYY-MM-DD/<topic-id>` or `YYYY-MM-DD/<topic-id>/<sub-id>`.
    pub anchor: String,
    pub tag: String,
    pub qualifier: Option<String>,
    pub text: String,
    /// Position of this entry within its date and topic, starting at 1.
    pub ordinal: usize,
    pub line: usize,
    pub link: Vec<Link>,
}

/// One monthly register file.
#[derive(Clone, Debug)]
pub struct Register {
    pub path: String,
    /// `YYYY-MM` taken from the file name.
    pub period: String,
    pub title: String,
    pub entry: Vec<Entry>,
}

impl Register {
    pub fn date(&self) -> Vec<&str> {
        let mut date = Vec::new();
        for entry in &self.entry {
            if date.last() != Some(&entry.date.as_str()) {
                date.push(entry.date.as_str());
            }
        }
        date
    }

    pub fn tag(&self) -> Vec<(&str, usize)> {
        let mut count: HashMap<&str, usize> = HashMap::new();
        for entry in &self.entry {
            *count.entry(entry.tag.as_str()).or_insert(0) += 1;
        }
        let mut tag = count.into_iter().collect::<Vec<_>>();
        tag.sort_by(|left, right| right.1.cmp(&left.1).then(left.0.cmp(right.0)));
        tag
    }
}

/// Parse one `decision-register` record into its dated entries.
pub fn parse(record: &Record) -> Register {
    let period = super::home::file_name(&record.path)
        .trim_end_matches(".md")
        .to_owned();
    let mut entry = Vec::new();
    let mut state = Section::default();
    let mut item: Vec<RawItem> = Vec::new();
    let mut current: Option<RawItem> = None;

    let line = record.body.lines().collect::<Vec<_>>();
    for (offset, text) in line.iter().enumerate() {
        let number = record.body_line + offset;
        let heading = heading_level(text);
        if heading.is_some() || (text.trim().is_empty() && current.is_some()) {
            flush_item(&mut current, &mut item);
        }
        match heading {
            Some((1, title)) if is_date(title) => {
                drain(&mut item, &state, &record.path, &mut entry);
                state.start_date(title);
                continue;
            }
            Some((2, title)) => {
                drain(&mut item, &state, &record.path, &mut entry);
                state.start_topic(title);
                continue;
            }
            Some((3, title)) => {
                drain(&mut item, &state, &record.path, &mut entry);
                state.start_sub_topic(title);
                continue;
            }
            Some(_) => continue,
            None => {}
        }
        if let Some(rest) = text.strip_prefix("- ") {
            flush_item(&mut current, &mut item);
            current = Some(RawItem {
                line: number,
                text: rest.trim().to_owned(),
            });
        } else if let Some(open) = current.as_mut()
            && text.starts_with("  ")
        {
            open.text.push(' ');
            open.text.push_str(text.trim());
        } else if !text.trim().is_empty() {
            flush_item(&mut current, &mut item);
        }
    }
    flush_item(&mut current, &mut item);
    drain(&mut item, &state, &record.path, &mut entry);

    Register {
        path: record.path.clone(),
        period,
        title: record.title.clone(),
        entry,
    }
}

#[derive(Clone, Debug)]
struct RawItem {
    line: usize,
    text: String,
}

fn flush_item(current: &mut Option<RawItem>, item: &mut Vec<RawItem>) {
    if let Some(open) = current.take() {
        item.push(open);
    }
}

#[derive(Default)]
struct Section {
    date: Option<String>,
    topic: Option<String>,
    topic_id: Option<String>,
    sub_topic: Option<String>,
    sub_topic_id: Option<String>,
    seen_topic_id: HashMap<String, usize>,
    ordinal: usize,
}

impl Section {
    fn start_date(&mut self, date: &str) {
        self.date = Some(date.to_owned());
        self.topic = None;
        self.topic_id = None;
        self.sub_topic = None;
        self.sub_topic_id = None;
        self.seen_topic_id.clear();
        self.ordinal = 0;
    }

    fn start_topic(&mut self, topic: &str) {
        let base = heading_id(topic);
        let occurrence = self.seen_topic_id.entry(base.clone()).or_insert(0);
        // Topic ids are scoped per date and repeat GitHub's `-1`, `-2`, … suffixing.
        self.topic_id = Some(if *occurrence == 0 {
            base
        } else {
            format!("{base}-{}", *occurrence)
        });
        *occurrence += 1;
        self.topic = Some(topic.to_owned());
        self.sub_topic = None;
        self.sub_topic_id = None;
        self.ordinal = 0;
    }

    fn start_sub_topic(&mut self, sub_topic: &str) {
        self.sub_topic = Some(sub_topic.to_owned());
        self.sub_topic_id = Some(heading_id(sub_topic));
    }
}

fn drain(item: &mut Vec<RawItem>, state: &Section, path: &str, entry: &mut Vec<Entry>) {
    if item.is_empty() {
        return;
    }
    let raw = std::mem::take(item);
    let (Some(date), Some(topic), Some(topic_id)) = (
        state.date.clone(),
        state.topic.clone(),
        state.topic_id.clone(),
    ) else {
        return;
    };
    let anchor = match &state.sub_topic_id {
        Some(sub_id) => format!("{date}/{topic_id}/{sub_id}"),
        None => format!("{date}/{topic_id}"),
    };

    let mut group: Vec<Vec<RawItem>> = Vec::new();
    for item in raw {
        let bold = bold_field(&item.text).is_some();
        match group.last_mut() {
            Some(last) if bold && bold_field(&last[0].text).is_some() => last.push(item),
            _ => group.push(vec![item]),
        }
    }

    for group in group {
        let ordinal = entry
            .iter()
            .filter(|existing| existing.date == date && existing.topic_id == topic_id)
            .count()
            + 1;
        let line = group[0].line;
        let (tag, qualifier, text) = if bold_field(&group[0].text).is_some() {
            bold_entry(&group)
        } else {
            bullet_entry(&group[0].text)
        };
        entry.push(Entry {
            link: links(&text, line, path),
            date: date.clone(),
            topic: topic.clone(),
            topic_id: topic_id.clone(),
            sub_topic: state.sub_topic.clone(),
            sub_topic_id: state.sub_topic_id.clone(),
            anchor: anchor.clone(),
            tag,
            qualifier,
            text,
            ordinal,
            line,
        });
    }
}

fn heading_level(text: &str) -> Option<(usize, &str)> {
    let level = text
        .chars()
        .take_while(|character| *character == '#')
        .count();
    if !(1..=6).contains(&level) {
        return None;
    }
    let rest = text[level..].strip_prefix(' ')?;
    Some((level, rest.trim()))
}

fn is_date(title: &str) -> bool {
    let byte = title.as_bytes();
    byte.len() == 10
        && byte[..4].iter().all(u8::is_ascii_digit)
        && byte[4] == b'-'
        && byte[5..7].iter().all(u8::is_ascii_digit)
        && byte[7] == b'-'
        && byte[8..].iter().all(u8::is_ascii_digit)
}

/// The bold key of a `- **Status:** …` item.
fn bold_field(text: &str) -> Option<(&str, &str)> {
    let rest = text.strip_prefix("**")?;
    let (key, value) = rest.split_once(":**")?;
    if key.is_empty() || key.contains("**") {
        return None;
    }
    Some((key, value.trim()))
}

fn bold_entry(group: &[RawItem]) -> (String, Option<String>, String) {
    let tag = group
        .iter()
        .find_map(|item| {
            let (key, value) = bold_field(&item.text)?;
            (key.eq_ignore_ascii_case("status")).then(|| first_word(value))
        })
        .unwrap_or_else(|| UNKNOWN_TAG.to_owned());
    let text = group
        .iter()
        .map(|item| format!("- {}", item.text))
        .collect::<Vec<_>>()
        .join("\n");
    (tag, None, text)
}

fn first_word(value: &str) -> String {
    value
        .split_whitespace()
        .next()
        .unwrap_or(UNKNOWN_TAG)
        .trim_matches(|character: char| !character.is_alphanumeric() && character != '-')
        .to_lowercase()
}

fn bullet_entry(text: &str) -> (String, Option<String>, String) {
    let Some(prefix_end) = tag_prefix_end(text) else {
        return (UNKNOWN_TAG.to_owned(), None, text.to_owned());
    };
    let prefix = text[..prefix_end].trim();
    let rest = text[prefix_end + 1..].trim().to_owned();
    match prefix.split_once(" / ") {
        Some((tag, qualifier)) => (
            tag.trim().to_lowercase(),
            Some(qualifier.trim().to_owned()),
            rest,
        ),
        None => (prefix.to_lowercase(), None, rest),
    }
}

/// The byte offset of the `:` that closes a leading tag phrase, when there is one.
fn tag_prefix_end(text: &str) -> Option<usize> {
    let position = text.find(':')?;
    if position == 0 || position > MAX_TAG_BYTES {
        return None;
    }
    let prefix = &text[..position];
    let mut code = false;
    for character in prefix.chars() {
        if character == '`' {
            code = !code;
        } else if !code && ".;!?()[]".contains(character) {
            return None;
        }
    }
    (!code).then_some(position)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::Repository;

    fn register(body: &str) -> Register {
        let root = crate::test_directory("register-fixture");
        let path = root.join("dev/docs/concept/log");
        std::fs::create_dir_all(&path).expect("log directory");
        std::fs::write(path.join("2026-08.md"), body).expect("log file");
        let repository = Repository::load(&root).expect("fixture loads");
        parse(
            repository
                .get("dev/docs/concept/log/2026-08.md")
                .expect("register record"),
        )
    }

    #[test]
    fn both_entry_shapes_and_sub_topics_are_parsed() {
        let register = register(
            "# August 2026\n\n# 2026-08-07\n\n## Game direction\n\n- decided: one shared World.\n- decided method / Terry: the MVP is the filter\n  across two lines.\n\n### Open decisions\n\n- open: which result proves the loop?\n\n# 2026-08-17\n\n## Studio accepted\n\n- **Status:** accepted; implementation active.\n- **Choice:** accept the [plan](../../../dev/plans/x/plan.md).\n- decided: a later plain entry.\n",
        );

        assert_eq!(register.period, "2026-08");
        let shape = register
            .entry
            .iter()
            .map(|entry| {
                (
                    entry.date.as_str(),
                    entry.topic_id.as_str(),
                    entry.sub_topic_id.as_deref(),
                    entry.tag.as_str(),
                    entry.qualifier.as_deref(),
                    entry.ordinal,
                )
            })
            .collect::<Vec<_>>();

        assert_eq!(
            shape,
            vec![
                ("2026-08-07", "game-direction", None, "decided", None, 1),
                (
                    "2026-08-07",
                    "game-direction",
                    None,
                    "decided method",
                    Some("Terry"),
                    2
                ),
                (
                    "2026-08-07",
                    "game-direction",
                    Some("open-decisions"),
                    "open",
                    None,
                    3
                ),
                ("2026-08-17", "studio-accepted", None, "accepted", None, 1),
                ("2026-08-17", "studio-accepted", None, "decided", None, 2),
            ]
        );
        assert_eq!(
            register.entry[1].text,
            "the MVP is the filter across two lines."
        );
        assert_eq!(
            register.entry[2].anchor,
            "2026-08-07/game-direction/open-decisions"
        );
        assert!(register.entry[3].text.contains("**Choice:**"));
        assert_eq!(register.entry[3].link.len(), 1);
        assert_eq!(register.date(), vec!["2026-08-07", "2026-08-17"]);
    }

    #[test]
    fn an_unreadable_list_item_becomes_an_unknown_tag_entry_instead_of_being_dropped() {
        let register = register(
            "# August 2026\n\n# 2026-08-07\n\n## Topic\n\n- created draft plan `x/plan.md` with no tag at all.\n- corrected the wording. Then a much later sentence: with a colon.\n",
        );

        assert_eq!(register.entry.len(), 2);
        assert!(
            register
                .entry
                .iter()
                .all(|entry| entry.tag == UNKNOWN_TAG && !entry.text.is_empty())
        );
    }

    #[test]
    fn a_tag_may_hold_backticks_but_never_sentence_punctuation() {
        assert_eq!(
            bullet_entry("researched / `5jaar`: momentum findings").0,
            "researched"
        );
        assert_eq!(
            bullet_entry("researched / `5jaar`: momentum findings").1,
            Some("`5jaar`".to_owned())
        );
        assert_eq!(
            bullet_entry("replaced change with `submit_action.change_entity_property`: done").0,
            "replaced change with `submit_action.change_entity_property`"
        );
        assert_eq!(bullet_entry("A sentence. Another: text").0, UNKNOWN_TAG);
    }

    #[test]
    fn repeated_topic_titles_under_one_date_get_distinct_anchors() {
        let register = register(
            "# August 2026\n\n# 2026-08-07\n\n## Topic\n\n- decided: first.\n\n## Topic\n\n- decided: second.\n",
        );

        assert_eq!(
            register
                .entry
                .iter()
                .map(|entry| entry.anchor.as_str())
                .collect::<Vec<_>>(),
            vec!["2026-08-07/topic", "2026-08-07/topic-1"]
        );
    }
}
