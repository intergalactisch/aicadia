//! Model folders joined to the canonical vocabulary and the realized migration tables.

use std::collections::BTreeMap;

use super::{
    home,
    record::{
        Backlink, FrontMatter, Kind, LintFinding, RULE_STORAGE_TABLE, Repository,
        split_front_matter,
    },
};

/// One canonical term of `CONTEXT.md`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Term {
    pub name: String,
    /// Kebab-cased name; a model with this id owns the term.
    pub id: String,
    pub line: usize,
    pub definition: String,
    /// The `_Avoid_:` line of the term block, when it has one.
    pub avoid: Option<String>,
}

/// Every `**Term**:` block of the canonical vocabulary, in file order.
pub fn vocabulary(repository: &Repository) -> Vec<Term> {
    let Some(record) = repository.get("CONTEXT.md") else {
        return Vec::new();
    };
    let mut term: Vec<Term> = Vec::new();
    let mut collecting: Option<(String, usize, Vec<String>)> = None;
    for (offset, line) in record.body.lines().enumerate() {
        let number = record.body_line + offset;
        if let Some(name) = term_name(line) {
            if let Some(open) = collecting.take() {
                term.push(finish_term(open));
            }
            collecting = Some((name, number, Vec::new()));
            continue;
        }
        match collecting.as_mut() {
            Some(_) if line.trim().is_empty() => {
                if let Some(open) = collecting.take() {
                    term.push(finish_term(open));
                }
            }
            Some((_, _, body)) => body.push(line.to_owned()),
            None => {}
        }
    }
    if let Some(open) = collecting {
        term.push(finish_term(open));
    }
    term
}

fn term_name(line: &str) -> Option<String> {
    let rest = line.strip_prefix("**")?;
    let (name, tail) = rest.split_once("**:")?;
    (tail.trim().is_empty() && !name.is_empty() && !name.contains("**")).then(|| name.to_owned())
}

fn finish_term((name, line, body): (String, usize, Vec<String>)) -> Term {
    let avoid = body.iter().find_map(|text| {
        text.strip_prefix("_Avoid_:")
            .map(|value| value.trim().to_owned())
    });
    let definition = body
        .iter()
        .filter(|text| !text.starts_with("_Avoid_:"))
        .cloned()
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_owned();
    Term {
        id: super::record::heading_id(&name),
        name,
        line,
        definition,
        avoid,
    }
}

/// One `public` table as the migrations finally name it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StorageTable {
    pub name: String,
    /// The migration that created the table.
    pub created_by: String,
    /// The migration that last renamed it, when it was renamed.
    pub renamed_by: Option<String>,
    /// The name the table was created under, when it differs from `name`.
    pub created_as: Option<String>,
}

/// Every table the repository migrations create, after applying their renames.
pub fn storage_table(repository: &Repository) -> Vec<StorageTable> {
    let mut table: Vec<StorageTable> = Vec::new();
    for record in repository.record() {
        if record.kind != Kind::Sql {
            continue;
        }
        let migration = home::file_name(&record.path).to_owned();
        for statement in record.body.split(';') {
            if let Some(name) = created_table(statement) {
                table.push(StorageTable {
                    name,
                    created_by: migration.clone(),
                    renamed_by: None,
                    created_as: None,
                });
            } else if let Some((from, to)) = renamed_table(statement)
                && let Some(existing) = table.iter_mut().find(|table| table.name == from)
            {
                existing.created_as = Some(existing.created_as.clone().unwrap_or(from));
                existing.name = to;
                existing.renamed_by = Some(migration.clone());
            }
        }
    }
    table.retain(|table| table.name != "_sqlx_migrations");
    table.sort_by(|left, right| left.name.cmp(&right.name));
    table
}

fn created_table(statement: &str) -> Option<String> {
    let mut word = keyword(statement);
    if !word.next()?.eq_ignore_ascii_case("create") || !word.next()?.eq_ignore_ascii_case("table") {
        return None;
    }
    let mut name = word.next()?;
    if name.eq_ignore_ascii_case("if") {
        word.next()?;
        word.next()?;
        name = word.next()?;
    }
    Some(unquote(name))
}

fn renamed_table(statement: &str) -> Option<(String, String)> {
    let mut word = keyword(statement);
    if !word.next()?.eq_ignore_ascii_case("alter") || !word.next()?.eq_ignore_ascii_case("table") {
        return None;
    }
    let from = unquote(word.next()?);
    if !word.next()?.eq_ignore_ascii_case("rename") || !word.next()?.eq_ignore_ascii_case("to") {
        return None;
    }
    Some((from, unquote(word.next()?)))
}

/// The statement's words, with comment lines removed and `(` treated as a break.
fn keyword(statement: &str) -> impl Iterator<Item = &str> {
    statement
        .lines()
        .filter(|line| !line.trim_start().starts_with("--"))
        .flat_map(|line| line.split_whitespace())
        .flat_map(|word| word.split('('))
        .filter(|word| !word.is_empty())
}

fn unquote(name: &str) -> String {
    name.trim_matches('"').to_owned()
}

/// One model folder joined to its term, tables, concern pages and backlinks.
#[derive(Clone, Debug)]
pub struct Model {
    /// Folder name, which is the stable model id.
    pub id: String,
    pub title: String,
    pub path: String,
    pub kind: Option<String>,
    pub storage_table: Vec<String>,
    /// Further `model-concern` pages in the same folder, ordered by file name.
    pub concern: Vec<String>,
    pub term: Option<Term>,
    pub backlink: Vec<Backlink>,
}

/// Every model folder under `docs/game/model/`, in id order.
///
/// An absent `docs/game/model/` directory yields an empty list rather than an error.
pub fn models(repository: &Repository) -> Vec<Model> {
    let term = vocabulary(repository);
    let mut model = repository
        .in_home("model")
        .into_iter()
        .map(|record| {
            let directory = home::directory(&record.path);
            let id = home::file_name(directory).to_owned();
            let concern = repository
                .in_home("model-concern")
                .into_iter()
                .filter(|concern| home::directory(&concern.path) == directory)
                .map(|concern| concern.path.clone())
                .collect();
            Model {
                term: term.iter().find(|term| term.id == id).cloned(),
                kind: record.front_matter.scalar("kind").map(str::to_owned),
                storage_table: record
                    .front_matter
                    .list("storage_table")
                    .map(<[String]>::to_vec)
                    .unwrap_or_default(),
                backlink: repository.backlink(&record.path).to_vec(),
                title: record.title.clone(),
                path: record.path.clone(),
                concern,
                id,
            }
        })
        .collect::<Vec<_>>();
    model.sort_by(|left, right| left.id.cmp(&right.id));
    model
}

/// One model folder's own front matter and contract body, front matter removed.
///
/// Returns `None` while `docs/game/model/<id>/README.md` does not exist.
pub fn contract(repository_root: &std::path::Path, id: &str) -> Option<(FrontMatter, String)> {
    let path = format!("docs/game/model/{id}/README.md");
    let content = home::read(repository_root, &path).ok()?;
    let (parsed, body, _) = split_front_matter(&content);
    let front_matter = match parsed {
        Some(Ok(front_matter)) => front_matter,
        _ => FrontMatter::default(),
    };
    Some((front_matter, body.to_owned()))
}

/// Model claims against realized migration tables, in both directions.
pub fn lint_storage_table(repository: &Repository) -> Vec<LintFinding> {
    let model = models(repository);
    let table = storage_table(repository);
    let mut finding = Vec::new();

    if model.is_empty() {
        if !table.is_empty() {
            finding.push(LintFinding {
                path: "docs/game/model".to_owned(),
                line: None,
                rule: RULE_STORAGE_TABLE,
                message: format!(
                    "no model folder exists, so none of the {} migration tables is claimed",
                    table.len()
                ),
            });
        }
        return finding;
    }

    let mut claim: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for model in &model {
        for claimed in &model.storage_table {
            claim.entry(claimed).or_default().push(&model.id);
            if !table.iter().any(|table| table.name == *claimed) {
                finding.push(LintFinding {
                    path: model.path.clone(),
                    line: Some(1),
                    rule: RULE_STORAGE_TABLE,
                    message: format!("claimed table `{claimed}` is created by no migration"),
                });
            }
        }
    }
    for (claimed, owner) in &claim {
        if owner.len() > 1 {
            finding.push(LintFinding {
                path: format!("docs/game/model/{}/README.md", owner[0]),
                line: Some(1),
                rule: RULE_STORAGE_TABLE,
                message: format!("table `{claimed}` is claimed by {} models", owner.len()),
            });
        }
    }
    for table in &table {
        if !claim.contains_key(table.name.as_str()) {
            finding.push(LintFinding {
                path: format!("migration/{}", table.created_by),
                line: None,
                rule: RULE_STORAGE_TABLE,
                message: format!("table `{}` is claimed by no model", table.name),
            });
        }
    }
    finding
}

/// The model that claims one table, when exactly one does.
pub fn owning_model<'a>(model: &'a [Model], table: &str) -> Option<&'a Model> {
    let mut owner = model
        .iter()
        .filter(|model| model.storage_table.iter().any(|claimed| claimed == table));
    let first = owner.next()?;
    owner.next().is_none().then_some(first)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repository(file: &[(&str, &str)]) -> Repository {
        let root = crate::studio::test_directory("model-fixture");
        for (path, content) in file {
            let full = root.join(path);
            std::fs::create_dir_all(full.parent().expect("parent")).expect("directory");
            std::fs::write(full, content).expect("record");
        }
        Repository::load(&root).expect("fixture loads")
    }

    const CONTEXT: &str = "# Aicadia\n\n> **Role / side:** canonical vocabulary / development side.\n> **Authority:** terminology.\n\n## Language\n\n**User**:\nThe durable participant.\n_Avoid_: Player, account\n\n**Investigation attempt**:\nOne admitted attempt.\n";

    const MIGRATION_ONE: &str = "CREATE TABLE app_user (\n  id uuid PRIMARY KEY\n);\n\nCREATE TABLE entity (\n  id uuid PRIMARY KEY\n);\n";
    const MIGRATION_TWO: &str = "ALTER TABLE app_user RENAME TO \"user\";\n\nALTER TABLE \"user\" RENAME CONSTRAINT app_user_pkey TO user_pkey;\n";
    const MIGRATION_THREE: &str = "-- CREATE TABLE commented_out (\nCREATE TABLE IF NOT EXISTS investigation_attempt (\n  id uuid PRIMARY KEY\n);\n";

    fn model_files() -> Vec<(&'static str, &'static str)> {
        vec![
            ("CONTEXT.md", CONTEXT),
            ("migration/0001_world.sql", MIGRATION_ONE),
            ("migration/0002_rename_app_user.sql", MIGRATION_TWO),
            ("migration/0010_investigation.sql", MIGRATION_THREE),
            (
                "docs/game/model/user/README.md",
                "---\nkind: subject\nstorage_table: [user]\n---\n\n# User\n\n> **Role / side:** User model contract / runtime side.\n> **Authority:** the durable participant.\n",
            ),
            (
                "docs/game/model/user/provenance.md",
                "# Request provenance\n\n> **Role / side:** User provenance concern / runtime side.\n> **Authority:** provenance columns.\n",
            ),
            (
                "docs/game/model/entity/README.md",
                "---\nkind: subject\nstorage_table: [entity]\n---\n\n# Entity\n\n> **Role / side:** Entity model contract / runtime side.\n> **Authority:** the durable subject.\n",
            ),
            (
                "docs/game/model/investigation-attempt/README.md",
                "---\nstorage_table: [investigation_attempt]\n---\n\n# Investigation attempt\n\n> **Role / side:** Investigation attempt contract / runtime side.\n> **Authority:** admission and chance.\n",
            ),
            (
                "docs/game/domain.md",
                "# Domain\n\n> **Role / side:** domain overview / runtime side.\n> **Authority:** cross-model rules.\n\n[Entity](model/entity/README.md)\n",
            ),
        ]
    }

    #[test]
    fn migration_tables_take_their_final_renamed_name() {
        let repository = repository(&model_files());
        let table = storage_table(&repository);

        assert_eq!(
            table
                .iter()
                .map(|table| (
                    table.name.as_str(),
                    table.created_by.as_str(),
                    table.created_as.as_deref()
                ))
                .collect::<Vec<_>>(),
            vec![
                ("entity", "0001_world.sql", None),
                ("investigation_attempt", "0010_investigation.sql", None),
                ("user", "0001_world.sql", Some("app_user")),
            ]
        );
        assert!(table.iter().all(|table| table.name != "commented_out"));
    }

    #[test]
    fn a_model_folder_joins_its_term_concern_pages_tables_and_backlinks() {
        let repository = repository(&model_files());
        let model = models(&repository);

        assert_eq!(
            model
                .iter()
                .map(|model| model.id.as_str())
                .collect::<Vec<_>>(),
            vec!["entity", "investigation-attempt", "user"]
        );
        let user = &model[2];
        assert_eq!(user.title, "User");
        assert_eq!(user.kind.as_deref(), Some("subject"));
        assert_eq!(user.storage_table, vec!["user".to_owned()]);
        assert_eq!(user.concern, vec!["docs/game/model/user/provenance.md"]);
        assert_eq!(
            user.term.as_ref().map(|term| term.name.as_str()),
            Some("User")
        );
        assert_eq!(
            user.term.as_ref().and_then(|term| term.avoid.as_deref()),
            Some("Player, account")
        );
        assert_eq!(
            model[1].term.as_ref().map(|term| term.id.as_str()),
            Some("investigation-attempt")
        );
        assert_eq!(
            model[0]
                .backlink
                .iter()
                .map(|backlink| backlink.path.as_str())
                .collect::<Vec<_>>(),
            vec!["docs/game/domain.md"]
        );
        assert_eq!(
            owning_model(&model, "entity").map(|model| model.id.as_str()),
            Some("entity")
        );
        let (front_matter, body) = contract(repository.root(), "user").expect("model contract");
        assert_eq!(
            front_matter.list("storage_table"),
            Some(["user".to_owned()].as_slice())
        );
        assert!(body.starts_with("\n# User"));
        assert!(contract(repository.root(), "absent").is_none());
    }

    #[test]
    fn every_migration_table_must_be_claimed_by_exactly_one_model() {
        let mut file = model_files();
        file.retain(|(path, _)| *path != "docs/game/model/entity/README.md");
        file.push((
            "docs/game/model/place/README.md",
            "---\nstorage_table: [place, user]\n---\n\n# Place\n\n> **Role / side:** Place model contract / runtime side.\n> **Authority:** the spatial role.\n",
        ));
        let repository = repository(&file);
        let message = lint_storage_table(&repository)
            .into_iter()
            .map(|finding| (finding.path, finding.message))
            .collect::<Vec<_>>();

        assert!(message.contains(&(
            "docs/game/model/place/README.md".to_owned(),
            "claimed table `place` is created by no migration".to_owned()
        )));
        assert!(message.contains(&(
            "migration/0001_world.sql".to_owned(),
            "table `entity` is claimed by no model".to_owned()
        )));
        assert!(
            message
                .iter()
                .any(|(_, message)| message == "table `user` is claimed by 2 models")
        );
    }

    #[test]
    fn a_complete_model_set_lints_clean_and_an_absent_model_home_is_one_finding() {
        let complete = repository(&model_files());
        assert_eq!(lint_storage_table(&complete), Vec::new());

        let without_models = repository(&[
            ("migration/0001_world.sql", MIGRATION_ONE),
            ("CONTEXT.md", CONTEXT),
        ]);
        assert!(models(&without_models).is_empty());
        let absent_home = lint_storage_table(&without_models);
        assert_eq!(absent_home.len(), 1);
        assert_eq!(absent_home[0].path, "docs/game/model");
        assert_eq!(absent_home[0].rule, RULE_STORAGE_TABLE);
    }
}
