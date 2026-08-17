//! `/jump?q=` — the server-side resolver. Not a search engine: it resolves one
//! known name, path, model, capability or term to its page, lists the candidates
//! when several match, and says so honestly when nothing does.

use axum::{
    extract::{Query, State},
    http::{HeaderMap, Uri},
    response::{IntoResponse, Redirect, Response},
};
use maud::html;
use serde::Deserialize;

use super::{Context, Crumb, Page, Plate, PlateRow, Reference, Seal, Section, Tone, doc_href};
use crate::studio::{StudioState, model};

#[derive(Debug, Deserialize)]
pub(in crate::studio) struct JumpQuery {
    #[serde(default)]
    q: String,
}

/// One resolvable Studio resource.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Target {
    label: String,
    href: String,
    kind: &'static str,
    /// The text a query is matched against, already lower-cased.
    key: Vec<String>,
}

pub(in crate::studio) async fn jump(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
    Query(query): Query<JumpQuery>,
) -> Response {
    let context = match Context::build(&state, &header, &uri).await {
        Ok(context) => context,
        Err(response) => return response,
    };
    let question = query.q.trim().to_owned();
    let target = catalog(&context);

    match resolve(&target, &question) {
        Resolution::Exact(href) => Redirect::to(&href).into_response(),
        Resolution::Several(candidate) => {
            context.render(result_page(&context, &question, &candidate))
        }
        Resolution::None => context.render(empty_page(&context, &question)),
    }
}

/// Everything `/jump` can resolve, in resolution order.
fn catalog(context: &Context) -> Vec<Target> {
    let repository = context.repository();
    let mut target = Vec::new();

    for model in model::models(repository) {
        target.push(Target {
            key: vec![model.id.to_lowercase(), model.title.to_lowercase()],
            label: model.title.clone(),
            href: doc_href(&model.path),
            kind: "model",
        });
    }
    for tool in &context.surface().tool {
        target.push(Target {
            key: vec![tool.name.to_lowercase()],
            label: tool.name.clone(),
            href: doc_href(&tool.capability_path),
            kind: "capability",
        });
    }
    for term in model::vocabulary(repository) {
        target.push(Target {
            key: vec![term.name.to_lowercase(), term.id.to_lowercase()],
            label: term.name.clone(),
            href: "/game/vocabulary".to_owned(),
            kind: "term",
        });
    }
    for record in repository.record() {
        target.push(Target {
            key: vec![record.path.to_lowercase(), record.title.to_lowercase()],
            label: record.title.clone(),
            href: doc_href(&record.path),
            kind: super::home_label(record.home_id()),
        });
    }
    target
}

enum Resolution {
    Exact(String),
    Several(Vec<Target>),
    None,
}

/// Exact key, then unique prefix, then unique substring; otherwise the candidates.
fn resolve(target: &[Target], question: &str) -> Resolution {
    let question = question.trim().to_lowercase();
    if question.is_empty() {
        return Resolution::None;
    }

    if let Some(exact) = target.iter().find(|target| target.key.contains(&question)) {
        return Resolution::Exact(exact.href.clone());
    }

    for narrower in [
        |key: &str, question: &str| key.starts_with(question),
        |key: &str, question: &str| key.contains(question),
    ] {
        let matched = unique(target, &question, narrower);
        match matched.len() {
            0 => continue,
            1 => return Resolution::Exact(matched[0].href.clone()),
            _ => return Resolution::Several(matched),
        }
    }
    Resolution::None
}

/// Every target matching the question, deduplicated by route.
fn unique(target: &[Target], question: &str, matches: fn(&str, &str) -> bool) -> Vec<Target> {
    let mut found: Vec<Target> = Vec::new();
    for candidate in target {
        if !candidate
            .key
            .iter()
            .any(|key| matches(key.as_str(), question))
        {
            continue;
        }
        if found.iter().any(|found| found.href == candidate.href) {
            continue;
        }
        found.push(candidate.clone());
    }
    found
}

fn result_page(context: &Context, question: &str, candidate: &[Target]) -> Page {
    Page::new(Section::Overview, "Several matches")
        .with_document_title(format!("{question} · Jump · Aicadia Studio"))
        .with_crumb(vec![Crumb::link("Overview", "/"), Crumb::here("Jump")])
        .with_seal(vec![Seal::plain(super::count(
            candidate.len(),
            "match",
            "matches",
        ))])
        .with_lede("More than one governed resource carries this name.")
        .with_plate(Plate::new(vec![PlateRow::fact("Query", question)]))
        .with_content(html! {
            ul class="list" {
                @for target in candidate {
                    li {
                        a href=(target.href) { (target.label) }
                        small { (target.kind) }
                    }
                }
            }
        })
        .with_reference(Reference {
            title: format!("Jump · {question}"),
            url: context.url(&format!("/jump?q={question}")),
            context: "Studio resolver, not an authority".to_owned(),
        })
}

fn empty_page(context: &Context, question: &str) -> Page {
    Page::new(Section::Overview, "Nothing resolves")
        .with_document_title("Jump · Aicadia Studio")
        .with_crumb(vec![Crumb::link("Overview", "/"), Crumb::here("Jump")])
        .with_seal(vec![Seal::toned("No match", Tone::Amber)])
        .with_lede(
            "Jump resolves a known record path, record title, model, capability or vocabulary term. It is not a search engine.",
        )
        .with_plate(Plate::new(vec![PlateRow::fact(
            "Query",
            if question.is_empty() { "—" } else { question },
        )]))
        .with_content(super::empty_state(
            "No governed record, model, capability or term carries this name.",
        ))
        .with_reference(Reference {
            title: "Jump".to_owned(),
            url: context.url("/jump"),
            context: "Studio resolver, not an authority".to_owned(),
        })
}

/// The repository is only needed to prove the resolver against real records.
#[cfg(test)]
fn repository_target(repository: &crate::studio::record::Repository) -> Vec<Target> {
    repository
        .record()
        .iter()
        .map(|record| Target {
            key: vec![record.path.to_lowercase(), record.title.to_lowercase()],
            label: record.title.clone(),
            href: doc_href(&record.path),
            kind: super::home_label(record.home_id()),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn target() -> Vec<Target> {
        vec![
            Target {
                label: "Entity".to_owned(),
                href: "/doc/docs/game/model/entity/README.md".to_owned(),
                kind: "model",
                key: vec!["entity".to_owned()],
            },
            Target {
                label: "create_entity".to_owned(),
                href: "/doc/docs/game/capability/create_entity.md".to_owned(),
                kind: "capability",
                key: vec!["create_entity".to_owned()],
            },
            Target {
                label: "Entity state rationale".to_owned(),
                href: "/doc/docs/concept/entity-state.md".to_owned(),
                kind: "concept record",
                key: vec![
                    "docs/concept/entity-state.md".to_owned(),
                    "entity state rationale".to_owned(),
                ],
            },
        ]
    }

    #[test]
    fn an_exact_name_wins_over_every_prefix() {
        assert!(matches!(
            resolve(&target(), "Entity"),
            Resolution::Exact(href) if href == "/doc/docs/game/model/entity/README.md"
        ));
    }

    #[test]
    fn a_unique_prefix_resolves_and_a_shared_one_lists() {
        assert!(matches!(
            resolve(&target(), "create_ent"),
            Resolution::Exact(_)
        ));
        assert!(matches!(
            resolve(&target(), "entity s"),
            Resolution::Exact(href) if href == "/doc/docs/concept/entity-state.md"
        ));
        assert!(matches!(
            resolve(&target(), "ent"),
            Resolution::Several(candidate) if candidate.len() == 2
        ));
    }

    #[test]
    fn an_unknown_name_resolves_to_nothing() {
        assert!(matches!(resolve(&target(), "zzz"), Resolution::None));
        assert!(matches!(resolve(&target(), "   "), Resolution::None));
    }

    #[test]
    fn every_governed_record_path_resolves_exactly() {
        let repository = crate::studio::record::Repository::load(env!("CARGO_MANIFEST_DIR"))
            .expect("the governed roots parse");
        let target = repository_target(&repository);

        for record in repository.record() {
            assert!(
                matches!(resolve(&target, &record.path), Resolution::Exact(href) if href == doc_href(&record.path)),
                "{} does not resolve to its own page",
                record.path
            );
        }
    }
}
