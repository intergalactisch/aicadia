//! The Live section: bounded, cross-linked views of the connected local World.
//!
//! The page hierarchy follows the game subjects first and exposes PostgreSQL
//! structure as a separate inspection layer. Every query delegates to the T7
//! read projection; this module owns presentation, cursor encoding and links.

use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode, Uri, header},
    response::{IntoResponse, Response},
};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::{DateTime, Utc};
use maud::{Markup, html};
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

use super::{
    Action, Context, Crumb, Page, Panel, PanelItem, Plate, PlateRow, Reference, Seal, Section,
    Tone, count, empty_state, note,
};
use crate::{
    StudioError, StudioState,
    live::{
        character, chronicle, entity, estimate, investigation, migration, page::Bound, place,
        property, resolve, row, schema, r#trait, user,
    },
};

const LIVE_PATH: &str = "/live";

macro_rules! context_or_return {
    ($state:expr, $header:expr, $uri:expr) => {{
        match Context::build($state, $header, $uri).await {
            Ok(context) => context,
            Err(response) => return response,
        }
    }};
}

macro_rules! bound_or_return {
    ($context:expr, $limit:expr, $title:expr, $path:expr) => {{
        match Bound::new($limit) {
            Ok(bound) => bound,
            Err(error) => return read_error($context, $title, $path, error),
        }
    }};
}

#[derive(Clone, Debug, Default, Deserialize)]
pub(crate) struct IdPageQuery {
    limit: Option<u16>,
    before: Option<Uuid>,
    after: Option<Uuid>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub(crate) struct TimePageQuery {
    limit: Option<u16>,
    before_at: Option<i64>,
    before: Option<Uuid>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub(crate) struct KeyPageQuery {
    limit: Option<u16>,
    after: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub(crate) struct ResolveQuery {
    id: Option<Uuid>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub(crate) struct RowQuery {
    limit: Option<u16>,
    after: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub(crate) struct PropertyHistoryQuery {
    limit: Option<u16>,
    before: Option<Uuid>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub(crate) struct ParticipationQuery {
    limit: Option<u16>,
    before: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub(crate) struct PlaceDetailQuery {
    limit: Option<u16>,
    character_after: Option<Uuid>,
    entity_after: Option<Uuid>,
    activity_before_at: Option<i64>,
    activity_before: Option<Uuid>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub(crate) struct CharacterDetailQuery {
    limit: Option<u16>,
    activity_before_at: Option<i64>,
    activity_before: Option<Uuid>,
    attempt_before: Option<Uuid>,
}

/// `/live` — the connected World, entered through domain subjects rather than tables.
pub(crate) async fn index(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
) -> Response {
    let context = match Context::build(&state, &header, &uri).await {
        Ok(context) => context,
        Err(response) => return response,
    };
    if !context.pulse().is_connected() {
        return context.render(unavailable_page(&context, "Live", LIVE_PATH));
    }

    let bound = Bound::default();
    let (estimate, latest, places) = tokio::join!(
        estimate::estimate(&state.pool),
        chronicle::list_world_chronicle(&state.pool, None, bound),
        place::list_place(&state.pool, None, bound),
    );
    let (estimate, latest, places) = match (estimate, latest, places) {
        (Ok(estimate), Ok(latest), Ok(places)) => (estimate, latest, places),
        (Err(error), _, _) | (_, Err(error), _) | (_, _, Err(error)) => {
            return read_error(&context, "Live", LIVE_PATH, error);
        }
    };
    let entry = places.item.iter().find(|item| item.is_entry);
    let entity_estimate = estimate
        .table
        .iter()
        .find(|item| item.table == "entity")
        .and_then(|item| item.row_estimate);
    let activity_estimate = estimate
        .table
        .iter()
        .find(|item| item.table == "activity")
        .and_then(|item| item.row_estimate);
    let reference = reference(&context, "Live", LIVE_PATH, "connected local World");

    let page = live_page(&context, "Live", LIVE_PATH)
        .with_seal(vec![Seal::status("Connected")])
        .with_lede(
            "The connected local World as Places, Characters, Activities and durable state. Storage remains a separate inspection layer.",
        )
        .with_plate(
            Plate::new(vec![
                PlateRow::text("Entities", estimate_label(entity_estimate)),
                PlateRow::text("Activities", estimate_label(activity_estimate)),
                PlateRow::text("Places loaded", places.item.len().to_string()),
                PlateRow::text("Ordering", latest.scope),
            ])
            .with_action(vec![Action::copy("Copy reference", reference.line())])
            .with_authority("Counts are PostgreSQL planner estimates. The World chronicle is a labeled local-development sort, never a game read."),
        )
        .with_content(html! {
            section class="section section-first" {
                h2 { "Enter the World" }
                @match entry {
                    Some(entry) => {
                        p class="section-note" { "The entry Place is the most direct route into its current Characters, Entities and chronicle." }
                        ul class="list" role="list" {
                            li {
                                a href=(place_href(entry.id)) { (&entry.name) }
                                small { "Entry Place" }
                            }
                        }
                    },
                    None => (empty_state("No entry Place is present in the loaded Place page.")),
                }
            }
            section class="section" {
                h2 { "Latest chronicle" span class="count" { (count(latest.page.item.len(), "loaded Activity", "loaded Activities")) } }
                p class="section-note" { "A local-development overview. Open a Place or Character for indexed history." }
                (chronicle_table(&latest.page.item, "world-chronicle"))
                (truncation(&latest.page.truncated, latest.page.next_cursor.map(world_cursor_href)))
            }
            section class="section" {
                h2 { "Browse by meaning" }
                ul class="list" role="list" {
                    li { a href="/live/place" { "Places" } small { "location and membership" } }
                    li { a href="/live/character" { "Characters" } small { "actor, owner and current Place" } }
                    li { a href="/live/entity" { "Entities" } small { "identity, roles and current state" } }
                    li { a href="/live/activity" { "World chronicle" } small { "accepted change history" } }
                    li { a href="/live/property-key" { "Property keys" } small { "canonical typed keys" } }
                    li { a href="/live/investigation" { "Investigation attempts" } small { "admission and outcome provenance" } }
                }
            }
        })
        .with_related(vec![Panel::list(
            "Operator tools",
            vec![
                PanelItem::link("Resolve an id", "/live/resolve"),
                PanelItem::link("Users", "/live/user"),
                PanelItem::link("Schema and rows", "/live/storage"),
                PanelItem::link("Migrations", "/live/migration"),
            ],
        )])
        .with_reference(reference);
    context.render(page)
}

/// `/live/place` — bounded Place list.
pub(crate) async fn place_list(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
    Query(query): Query<IdPageQuery>,
) -> Response {
    let context = context_or_return!(&state, &header, &uri);
    let bound = bound_or_return!(&context, query.limit, "Places", "/live/place");
    let data = match place::list_place(&state.pool, query.before, bound).await {
        Ok(data) => data,
        Err(error) => return read_error(&context, "Places", "/live/place", error),
    };
    let next = data
        .next_cursor
        .map(|cursor| format!("/live/place?before={cursor}&limit={}", bound.limit()));
    let page = list_page(&context, "Places", "/live/place", "Place", data.item.len(), data.truncated)
        .with_lede("Locations that hold World entry, current membership and the revision of their latest accepted change.")
        .with_content(html! {
            (loaded_filter("place-list", "Filter loaded Places"))
            (place_table(&data.item))
            (truncation(&data.truncated, next))
        });
    context.render(page)
}

/// `/live/place/{id}` — one Place, membership and indexed chronicle.
pub(crate) async fn place_detail(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
    Path(id): Path<Uuid>,
    Query(query): Query<PlaceDetailQuery>,
) -> Response {
    let context = context_or_return!(&state, &header, &uri);
    let bound = bound_or_return!(&context, query.limit, "Place", &place_href(id));
    let (detail, characters, entities, activity) = tokio::join!(
        place::get_place(&state.pool, id),
        place::list_place_character(&state.pool, id, query.character_after, bound),
        place::list_place_entity(&state.pool, id, query.entity_after, bound),
        chronicle::list_place_chronicle(
            &state.pool,
            id,
            time_cursor(query.activity_before_at, query.activity_before)
                .map(|(occurred_at, id)| chronicle::ChronicleCursor { occurred_at, id }),
            bound,
        ),
    );
    let (detail, characters, entities, activity) = match (detail, characters, entities, activity) {
        (Ok(a), Ok(b), Ok(c), Ok(d)) => (a, b, c, d),
        (Err(error), _, _, _)
        | (_, Err(error), _, _)
        | (_, _, Err(error), _)
        | (_, _, _, Err(error)) => return read_error(&context, "Place", &place_href(id), error),
    };
    let path = place_href(id);
    let reference = reference(&context, &detail.name, &path, &format!("Place {id}"));
    let page = live_page(&context, &detail.name, &path)
        .with_crumb(vec![Crumb::link("Live", "/live"), Crumb::link("Places", "/live/place"), Crumb::here(&detail.name)])
        .with_seal(if detail.is_entry {
            vec![Seal::status("Entry Place")]
        } else {
            Vec::new()
        })
        .with_lede(&detail.description)
        .with_plate(Plate::new(vec![
            PlateRow::fact("Place id", id.to_string()),
            PlateRow::text("Introduced", time(detail.introduced_at)),
            PlateRow::text("Latest change", format!("{} · {}", detail.latest_activity_operation, time(detail.latest_activity_occurred_at))),
        ]).with_action(vec![Action::copy("Copy id", id.to_string())]).with_authority("Place identity and revision come from the connected World; each list below is independently bounded."))
        .with_content(html! {
            section class="section section-first" {
                h2 { "Characters here" span class="count" { (characters.item.len()) } }
                (character_membership_table(&characters.item))
                (truncation(&characters.truncated, characters.next_cursor.map(|cursor| format!("{path}?character_after={cursor}&limit={}", bound.limit()))))
            }
            section class="section" {
                h2 { "Entities here" span class="count" { (entities.item.len()) } }
                (place_entity_table(&entities.item))
                (truncation(&entities.truncated, entities.next_cursor.map(|cursor| format!("{path}?entity_after={cursor}&limit={}", bound.limit()))))
            }
            section class="section" {
                h2 { "Place chronicle" span class="count" { (activity.item.len()) } }
                p class="section-note" { "Indexed history for this Place, newest accepted Activity first." }
                (chronicle_table(&activity.item, "place-chronicle"))
                (truncation(&activity.truncated, activity.next_cursor.map(|cursor| format!("{path}?activity_before_at={}&activity_before={}&limit={}", cursor.occurred_at.timestamp_millis(), cursor.id, bound.limit()))))
            }
        })
        .with_related(vec![Panel::list("Place links", vec![
            PanelItem::link("Entity identity", entity_href(id)),
            PanelItem::link("Latest Activity", activity_href(detail.latest_activity_id)),
            PanelItem::link("Place table", "/live/storage/place"),
        ])])
        .with_reference(reference);
    context.render(page)
}

/// `/live/character` — bounded Character list.
pub(crate) async fn character_list(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
    Query(query): Query<IdPageQuery>,
) -> Response {
    let context = context_or_return!(&state, &header, &uri);
    let bound = bound_or_return!(&context, query.limit, "Characters", "/live/character");
    let data = match character::list_character(&state.pool, query.before, bound).await {
        Ok(data) => data,
        Err(error) => return read_error(&context, "Characters", "/live/character", error),
    };
    let next = data
        .next_cursor
        .map(|cursor| format!("/live/character?before={cursor}&limit={}", bound.limit()));
    let page = list_page(&context, "Characters", "/live/character", "Character", data.item.len(), data.truncated)
        .with_lede("User-owned actors, with the Place each Character currently occupies.")
        .with_content(html! { (loaded_filter("character-list", "Filter loaded Characters")) (character_table(&data.item)) (truncation(&data.truncated, next)) });
    context.render(page)
}

/// `/live/character/{id}` — one Character with indexed history and attempts.
pub(crate) async fn character_detail(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
    Path(id): Path<Uuid>,
    Query(query): Query<CharacterDetailQuery>,
) -> Response {
    let context = context_or_return!(&state, &header, &uri);
    let bound = bound_or_return!(&context, query.limit, "Character", &character_href(id));
    let cursor = time_cursor(query.activity_before_at, query.activity_before);
    let attempt_cursor = query
        .attempt_before
        .map(|request_id| investigation::AttemptCursor { request_id });
    let (detail, activity, attempts) = tokio::join!(
        character::get_character(&state.pool, id),
        chronicle::list_character_chronicle(
            &state.pool,
            id,
            cursor.map(|c| chronicle::ChronicleCursor {
                occurred_at: c.0,
                id: c.1
            }),
            bound
        ),
        character::list_character_attempt(&state.pool, id, attempt_cursor, bound),
    );
    let (detail, activity, attempts) = match (detail, activity, attempts) {
        (Ok(a), Ok(b), Ok(c)) => (a, b, c),
        (Err(error), _, _) | (_, Err(error), _) | (_, _, Err(error)) => {
            return read_error(&context, "Character", &character_href(id), error);
        }
    };
    let path = character_href(id);
    let reference = reference(&context, &detail.name, &path, &format!("Character {id}"));
    let page = live_page(&context, &detail.name, &path)
        .with_crumb(vec![Crumb::link("Live", "/live"), Crumb::link("Characters", "/live/character"), Crumb::here(&detail.name)])
        .with_lede(&detail.description)
        .with_plate(Plate::new(vec![
            PlateRow::fact("Character id", id.to_string()),
            PlateRow::fact("Owner User", detail.owner_user_id.to_string()),
            PlateRow::text("Introduced", time(detail.introduced_at)),
        ]).with_action(vec![Action::copy("Copy id", id.to_string())]).with_authority("Character identity, ownership and current Place are read by primary key; history and attempts are separately bounded."))
        .with_content(html! {
            section class="section section-first" { h2 { "Current Place" }
                @match &detail.current_place { Some(place) => ul class="list" role="list" { li { a href=(place_href(place.id)) { (&place.name) } small { @if detail.current_place_is_entry == Some(true) { "Entry Place" } @else { "Place" } } } }, None => (empty_state("This Character has not entered the World.")) }
            }
            section class="section" { h2 { "Character chronicle" span class="count" { (activity.item.len()) } } (loaded_operation_filter("character-chronicle")) (chronicle_table(&activity.item, "character-chronicle")) (truncation(&activity.truncated, activity.next_cursor.map(|cursor| format!("{path}?activity_before_at={}&activity_before={}&limit={}", cursor.occurred_at.timestamp_millis(), cursor.id, bound.limit())))) }
            section class="section" { h2 { "Investigation attempts" span class="count" { (attempts.item.len()) } } (attempt_table(&attempts.item)) (truncation(&attempts.truncated, attempts.next_cursor.map(|cursor| format!("{path}?attempt_before={}&limit={}", cursor.request_id, bound.limit())))) }
        })
        .with_related(vec![Panel::list("Character links", vec![
            PanelItem::link("Entity identity", entity_href(id)),
            PanelItem::link("Owner User", user_href(detail.owner_user_id)),
            PanelItem::link("Character table", "/live/storage/character"),
        ])])
        .with_reference(reference);
    context.render(page)
}

/// `/live/entity` — newest-first Entity list.
pub(crate) async fn entity_list(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
    Query(query): Query<TimePageQuery>,
) -> Response {
    let context = context_or_return!(&state, &header, &uri);
    let bound = bound_or_return!(&context, query.limit, "Entities", "/live/entity");
    let before = time_cursor(query.before_at, query.before)
        .map(|(introduced_at, id)| entity::EntityCursor { introduced_at, id });
    let data = match entity::list_entity(&state.pool, before, bound).await {
        Ok(data) => data,
        Err(error) => return read_error(&context, "Entities", "/live/entity", error),
    };
    let next = data.next_cursor.map(|cursor| {
        format!(
            "/live/entity?before_at={}&before={}&limit={}",
            cursor.introduced_at.timestamp_millis(),
            cursor.id,
            bound.limit()
        )
    });
    let page = list_page(&context, "Entities", "/live/entity", "Entity", data.item.len(), data.truncated)
        .with_lede("Durable World subjects, newest introduced first, with Character and Place roles made explicit.")
        .with_content(html! { (loaded_filter("entity-list", "Filter loaded Entities")) (entity_table(&data.item)) (truncation(&data.truncated, next)) });
    context.render(page)
}

/// `/live/entity/{id}` — identity, current state and participation.
pub(crate) async fn entity_detail(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
    Path(id): Path<Uuid>,
    Query(query): Query<ParticipationQuery>,
) -> Response {
    let context = context_or_return!(&state, &header, &uri);
    let bound = bound_or_return!(&context, query.limit, "Entity", &entity_href(id));
    let before = query.before.as_deref().and_then(decode_participation);
    let (detail, participation) = tokio::join!(
        entity::get_entity(&state.pool, id),
        entity::list_participation(&state.pool, id, before, bound)
    );
    let (detail, participation) = match (detail, participation) {
        (Ok(a), Ok(b)) => (a, b),
        (Err(error), _) | (_, Err(error)) => {
            return read_error(&context, "Entity", &entity_href(id), error);
        }
    };
    let path = entity_href(id);
    let reference = reference(
        &context,
        &detail.entity.name,
        &path,
        &format!("Entity {id}"),
    );
    let page = live_page(&context, &detail.entity.name, &path)
        .with_crumb(vec![Crumb::link("Live", "/live"), Crumb::link("Entities", "/live/entity"), Crumb::here(&detail.entity.name)])
        .with_lede(&detail.entity.description)
        .with_plate(Plate::new(vec![
            PlateRow::fact("Entity id", id.to_string()), PlateRow::text("Introduced", time(detail.entity.introduced_at)),
        ]).with_action(vec![Action::copy("Copy id", id.to_string())]).with_authority("Identity and current state are bounded World reads. Property and Trait history stays on the linked lineage pages."))
        .with_content(html! {
            section class="section section-first" { h2 { "World context" } dl class="meta" {
                dt { "Current Place" } dd { @match &detail.entity.current_place { Some(place) => a href=(place_href(place.id)) { (&place.name) }, None => span class="mute" { "Not placed" } } }
                dt { "Introduced by" } dd { a href=(user_href(detail.entity.introduced_by_user_id)) { "User " code class="fact" { (detail.entity.introduced_by_user_id) } } }
                @if let Some(owner) = detail.entity.owner_user_id { dt { "Owner" } dd { a href=(user_href(owner)) { "User " code class="fact" { (owner) } } } }
            } }
            section class="section" { h2 { "Current Properties" span class="count" { (detail.property.len()) } } (entity_property_table(id, &detail.property)) (truncated_only(detail.property_truncated)) }
            section class="section" { h2 { "Current Traits" span class="count" { (detail.r#trait.len()) } } (entity_trait_table(&detail.r#trait)) (truncated_only(detail.trait_truncated)) }
            section class="section" { h2 { "Activity participation" span class="count" { (participation.page.item.len()) } } p class="section-note" { "Ordered by Activity id, not chronology; each row shows its stored time." } (participation_table(&participation.page.item)) (truncation(&participation.page.truncated, participation.page.next_cursor.as_ref().map(|cursor| format!("{path}?before={}&limit={}", encode_participation(cursor), bound.limit())))) }
        })
        .with_related(vec![Panel::list("Entity links", entity_related(id, &detail))])
        .with_reference(reference);
    context.render(page)
}

/// `/live/entity/{entity_id}/property/{property_key_id}` — one exact Property lineage.
pub(crate) async fn property_history(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
    Path((entity_id, property_key_id)): Path<(Uuid, i64)>,
    Query(query): Query<PropertyHistoryQuery>,
) -> Response {
    let context = context_or_return!(&state, &header, &uri);
    let path = format!("/live/entity/{entity_id}/property/{property_key_id}");
    let bound = bound_or_return!(&context, query.limit, "Property history", &path);
    let data = match entity::list_property_history(
        &state.pool,
        entity_id,
        property_key_id,
        query.before,
        bound,
    )
    .await
    {
        Ok(data) => data,
        Err(error) => return read_error(&context, "Property history", &path, error),
    };
    let reference = reference(
        &context,
        &data.key,
        &path,
        &format!("Entity {entity_id} · Property key {property_key_id}"),
    );
    let page = live_page(&context, &data.key, &path)
        .with_crumb(vec![Crumb::link("Live", "/live"), Crumb::link("Entity", entity_href(entity_id)), Crumb::here(&data.key)])
        .with_lede("Every stored version of this Entity Property, with explicit predecessor links and current state.")
        .with_plate(Plate::new(vec![PlateRow::fact("Entity", entity_id.to_string()), PlateRow::fact("Property key id", property_key_id.to_string()), PlateRow::text("Ordering", data.order)]).with_authority("The primary key orders by random Activity id, not time. Stored timestamps and predecessor ids carry the exact lineage."))
        .with_content(html! { section class="section section-first" { h2 { "Versions" span class="count" { (data.page.item.len()) } } (property_version_table(&data.page.item)) (truncation(&data.page.truncated, data.page.next_cursor.map(|cursor| format!("{path}?before={cursor}&limit={}", bound.limit())))) } })
        .with_related(vec![Panel::list("Property links", vec![PanelItem::link("Entity", entity_href(entity_id)), PanelItem::link("Property key", property_key_href(&data.key))])])
        .with_reference(reference);
    context.render(page)
}

/// `/live/activity` — the bounded World chronicle.
pub(crate) async fn activity_list(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
    Query(query): Query<TimePageQuery>,
) -> Response {
    let context = context_or_return!(&state, &header, &uri);
    let bound = bound_or_return!(&context, query.limit, "World chronicle", "/live/activity");
    let before = time_cursor(query.before_at, query.before)
        .map(|(occurred_at, id)| chronicle::ChronicleCursor { occurred_at, id });
    let data = match chronicle::list_world_chronicle(&state.pool, before, bound).await {
        Ok(data) => data,
        Err(error) => return read_error(&context, "World chronicle", "/live/activity", error),
    };
    let next = data
        .page
        .next_cursor
        .map(|cursor| world_cursor_href_with_limit(cursor, bound.limit()));
    let page = list_page(&context, "World chronicle", "/live/activity", "Activity", data.page.item.len(), data.page.truncated)
        .with_seal(vec![Seal::toned(data.scope, Tone::Amber)])
        .with_lede("The newest accepted Activities across the local World. Open a Place or Character for indexed, game-shaped history.")
        .with_plate(Plate::new(vec![PlateRow::text("Loaded",count(data.page.item.len(),"Activity","Activities")),PlateRow::text("Limit","1–100")]).with_authority("This global newest-first view is the explicit D6 local-development sort, not an indexed game read. Operation filtering applies only to this loaded page."))
        .with_content(html! { (loaded_operation_filter("activity-list")) (chronicle_table(&data.page.item, "activity-list")) (truncation(&data.page.truncated, next)) });
    context.render(page)
}

/// `/live/activity/{id}` — one immutable Activity and every recorded change.
pub(crate) async fn activity_detail(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
    Path(id): Path<Uuid>,
) -> Response {
    let context = context_or_return!(&state, &header, &uri);
    let data = match chronicle::get_activity(&state.pool, id).await {
        Ok(data) => data,
        Err(error) => return read_error(&context, "Activity", &activity_href(id), error),
    };
    let path = activity_href(id);
    let reference = reference(&context, &data.operation, &path, &format!("Activity {id}"));
    let page = live_page(&context, &data.operation, &path)
        .with_crumb(vec![Crumb::link("Live", "/live"), Crumb::link("World chronicle", "/live/activity"), Crumb::here(&data.operation)])
        .with_lede(data.prose.as_deref().unwrap_or("This Activity carries no prose."))
        .with_plate(Plate::new(vec![PlateRow::fact("Activity id", id.to_string()), PlateRow::text("Occurred", time(data.occurred_at)), PlateRow::fact("Requesting User", data.requested_by_user_id.to_string())]).with_action(vec![Action::copy("Copy id", id.to_string())]).with_authority("Activity is the durable historical footprint accepted in the same transaction as its state change."))
        .with_content(html! {
            section class="section section-first" { h2 { "Context" } dl class="meta" {
                dt { "Consequence" } dd { (data.action_consequence.as_deref().unwrap_or("No action consequence")) }
                dt { "Actor" } dd { @match &data.actor_character { Some(actor) => a href=(character_href(actor.id)) { (&actor.name) }, None => span class="mute" { "System or provisioning action" } } }
                dt { "Place" } dd { @match &data.context_place { Some(place) => a href=(place_href(place.id)) { (&place.name) }, None => span class="mute" { "No Place context" } } }
                @if let Some(attempt) = data.consumed_investigation_attempt_id { dt { "Consumed attempt" } dd { a href=(attempt_href(attempt)) { code class="fact" { (attempt) } } } }
            } }
            section class="section" { h2 { "Involved Entities" span class="count" { (data.involved_entity.len()) } } (involved_table(&data.involved_entity)) (truncated_only(data.involved_entity_truncated)) }
            @if !data.property_change.is_empty() { section class="section" { h2 { "Property changes" span class="count" { (data.property_change.len()) } } (activity_property_table(&data.property_change)) (truncated_only(data.property_change_truncated)) } }
            @if !data.trait_change.is_empty() { section class="section" { h2 { "Trait changes" span class="count" { (data.trait_change.len()) } } (activity_trait_table(&data.trait_change)) (truncated_only(data.trait_change_truncated)) } }
        })
        .with_related(vec![Panel::list("Activity links", activity_related(&data))])
        .with_reference(reference);
    context.render(page)
}

/// `/live/user` — bounded User list.
pub(crate) async fn user_list(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
    Query(query): Query<IdPageQuery>,
) -> Response {
    let context = context_or_return!(&state, &header, &uri);
    let bound = bound_or_return!(&context, query.limit, "Users", "/live/user");
    let data = match user::list_user(&state.pool, query.after, bound).await {
        Ok(data) => data,
        Err(error) => return read_error(&context, "Users", "/live/user", error),
    };
    let next = data
        .next_cursor
        .map(|cursor| format!("/live/user?after={cursor}&limit={}", bound.limit()));
    let page = list_page(&context, "Users", "/live/user", "User", data.item.len(), data.truncated)
        .with_lede("Durable participants and the single Character each may own. This is operator provenance, not player-facing identity.")
        .with_content(html! { (user_table(&data.item)) (truncation(&data.truncated, next)) });
    context.render(page)
}

/// `/live/user/{id}` — one User, Character, Place and indexed attempts.
pub(crate) async fn user_detail(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
    Path(id): Path<Uuid>,
    Query(query): Query<IdPageQuery>,
) -> Response {
    let context = context_or_return!(&state, &header, &uri);
    let bound = bound_or_return!(&context, query.limit, "User", &user_href(id));
    let before = query
        .before
        .map(|request_id| investigation::AttemptCursor { request_id });
    let (detail, attempts) = tokio::join!(
        user::get_user(&state.pool, id),
        investigation::list_user_attempt(&state.pool, id, before, bound)
    );
    let (detail, attempts) = match (detail, attempts) {
        (Ok(a), Ok(b)) => (a, b),
        (Err(error), _) | (_, Err(error)) => {
            return read_error(&context, "User", &user_href(id), error);
        }
    };
    let path = user_href(id);
    let reference = reference(&context, "User", &path, &format!("User {id}"));
    let page = live_page(&context,"User",&path)
        .with_crumb(vec![Crumb::link("Live","/live"),Crumb::link("Users","/live/user"),Crumb::here("User")])
        .with_lede("The durable participant behind one Character and its investigation attempts.")
        .with_plate(Plate::new(vec![PlateRow::fact("User id",id.to_string()),PlateRow::text("Created",time(detail.created_at))]).with_action(vec![Action::copy("Copy id",id.to_string())]).with_authority("User detail intentionally does not scan Entities by introducing User; Entity detail shows that relationship from its indexed side."))
        .with_content(html! {
            section class="section section-first" { h2 { "World presence" } @match &detail.character { Some(character) => dl class="meta" { dt { "Character" } dd { a href=(character_href(character.id)) { (&character.name) } } dt { "Current Place" } dd { @match &detail.character_current_place { Some(place) => a href=(place_href(place.id)) { (&place.name) }, None => span class="mute" { "Not in the World" } } } }, None => (empty_state("This User has no Character.")) } }
            section class="section" { h2 { "Investigation attempts" span class="count" { (attempts.item.len()) } } (attempt_table(&attempts.item)) (truncation(&attempts.truncated, attempts.next_cursor.map(|cursor| format!("{path}?before={}&limit={}", cursor.request_id, bound.limit())))) }
            (note(Tone::Plain, detail.introduced_entity_note))
        })
        .with_related(vec![Panel::list("User links",vec![PanelItem::link("User table","/live/storage/user"),PanelItem::link("Investigation attempts","/live/investigation")])])
        .with_reference(reference);
    context.render(page)
}

/// `/live/property-key` — canonical typed keys.
pub(crate) async fn property_key_list(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
    Query(query): Query<KeyPageQuery>,
) -> Response {
    let context = context_or_return!(&state, &header, &uri);
    let bound = bound_or_return!(&context, query.limit, "Property keys", "/live/property-key");
    let data = match property::list_property_key(&state.pool, query.after.as_deref(), bound).await {
        Ok(data) => data,
        Err(error) => return read_error(&context, "Property keys", "/live/property-key", error),
    };
    let next = data.next_cursor.as_deref().map(|cursor| {
        format!(
            "/live/property-key?after={}&limit={}",
            encode_text(cursor),
            bound.limit()
        )
    });
    let page = list_page(&context,"Property keys","/live/property-key","Property key",data.item.len(),data.truncated)
        .with_lede("Canonical keys created at first accepted use, with one immutable value type across the World.")
        .with_content(html! { (loaded_filter("property-key-list","Filter loaded Property keys")) (property_key_table(&data.item)) (truncation(&data.truncated,next)) });
    context.render(page)
}

/// `/live/property-key/{key}` — one key and its bounded holder view.
pub(crate) async fn property_key_detail(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
    Path(key): Path<String>,
) -> Response {
    let context = context_or_return!(&state, &header, &uri);
    let path = property_key_href(&key);
    let data = match property::get_property_key(&state.pool, &key).await {
        Ok(data) => data,
        Err(error) => return read_error(&context, "Property key", &path, error),
    };
    let reference = reference(
        &context,
        &data.key,
        &path,
        &format!("Property key {}", data.id),
    );
    let page = live_page(&context,&data.key,&path)
        .with_crumb(vec![Crumb::link("Live","/live"),Crumb::link("Property keys","/live/property-key"),Crumb::here(&data.key)])
        .with_lede("The canonical key, immutable value type and first accepted use.")
        .with_plate(Plate::new(vec![PlateRow::fact("Key id",data.id.to_string()),PlateRow::text("Value type",&data.value_type),PlateRow::text("First use",format!("{} · {}",data.first_activity_operation,time(data.first_activity_occurred_at)))]).with_authority("The key and its first Activity are unique-index and primary-key reads. Reverse holder browsing is absent because that direction is not indexed."))
        .with_related(vec![Panel::list("Property links",vec![PanelItem::link("First Activity",activity_href(data.first_activity_id)),PanelItem::link("Property key table","/live/storage/property_key")])])
        .with_reference(reference);
    context.render(page)
}

/// `/live/trait` — an honest entry point to lineages discoverable through Entities.
pub(crate) async fn trait_index(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
) -> Response {
    let context = context_or_return!(&state, &header, &uri);
    let page = live_page(&context,"Traits","/live/trait")
        .with_lede("Trait lineages are entered from an Entity's current state or an Activity's recorded changes.")
        .with_plate(Plate::new(vec![PlateRow::text("Lookup","Stable Trait id")]).with_authority("T7 exposes exact Trait detail but no unindexed global Trait list. Studio does not invent one."))
        .with_content(html! { (note(Tone::Plain,"Open an Entity to follow its current Traits, or resolve an exact Trait id.")) section class="section" { h2 { "Ways in" } ul class="list" role="list" { li { a href="/live/entity" { "Browse Entities" } small { "current Traits" } } li { a href="/live/resolve" { "Resolve a Trait id" } small { "exact lookup" } } } } });
    context.render(page)
}

/// `/live/trait/{id}` — one current Trait and complete bounded lineage.
pub(crate) async fn trait_detail(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
    Path(id): Path<Uuid>,
) -> Response {
    let context = context_or_return!(&state, &header, &uri);
    let data = match r#trait::get_trait(&state.pool, id).await {
        Ok(data) => data,
        Err(error) => return read_error(&context, "Trait", &trait_href(id), error),
    };
    let path = trait_href(id);
    let reference = reference(&context, "Trait", &path, &format!("Trait {id}"));
    let page = live_page(&context,"Trait",&path)
        .with_crumb(vec![Crumb::link("Live","/live"),Crumb::link("Traits","/live/trait"),Crumb::here("Trait")])
        .with_lede(&data.current_statement)
        .with_plate(Plate::new(vec![PlateRow::fact("Trait id",id.to_string()),PlateRow::text("Entity",&data.entity.name),PlateRow::text("Ordering",data.order)]).with_action(vec![Action::copy("Copy id",id.to_string())]).with_authority("The lineage is ordered by Activity id, not chronology. Predecessor ids express the exact chain."))
        .with_content(html! { section class="section section-first" { h2 { "Versions" span class="count" { (data.version.len()) } } (trait_version_table(&data.version)) (truncated_only(data.version_truncated)) } })
        .with_related(vec![Panel::list("Trait links",vec![PanelItem::link("Owning Entity",entity_href(data.entity.id)),PanelItem::link("Current Activity",activity_href(data.current_activity_id)),PanelItem::link("Trait table","/live/storage/entity_trait")])])
        .with_reference(reference);
    context.render(page)
}

/// `/live/investigation` — bounded attempt provenance.
pub(crate) async fn investigation_list(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
    Query(query): Query<IdPageQuery>,
) -> Response {
    let context = context_or_return!(&state, &header, &uri);
    let bound = bound_or_return!(
        &context,
        query.limit,
        "Investigation attempts",
        "/live/investigation"
    );
    let data = match investigation::list_investigation(&state.pool, query.after, bound).await {
        Ok(data) => data,
        Err(error) => {
            return read_error(
                &context,
                "Investigation attempts",
                "/live/investigation",
                error,
            );
        }
    };
    let next = data
        .page
        .next_cursor
        .map(|cursor| format!("/live/investigation?after={cursor}&limit={}", bound.limit()));
    let page = list_page(&context,"Investigation attempts","/live/investigation","attempt",data.page.item.len(),data.page.truncated)
        .with_lede("Durable records of admitted investigations, their outcome and one-time consumption or voiding.")
        .with_content(html! { (loaded_filter("attempt-list","Filter loaded attempts")) (attempt_table(&data.page.item)) (truncation(&data.page.truncated,next)) });
    context.render(page)
}

/// `/live/investigation/{id}` — one attempt lifecycle.
pub(crate) async fn investigation_detail(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
    Path(id): Path<Uuid>,
) -> Response {
    let context = context_or_return!(&state, &header, &uri);
    let data = match investigation::get_investigation(&state.pool, id).await {
        Ok(data) => data,
        Err(error) => {
            return read_error(&context, "Investigation attempt", &attempt_href(id), error);
        }
    };
    let path = attempt_href(id);
    let reference = reference(
        &context,
        "Investigation attempt",
        &path,
        &format!("Investigation attempt {id}"),
    );
    let page = live_page(&context,"Investigation attempt",&path)
        .with_crumb(vec![Crumb::link("Live","/live"),Crumb::link("Investigation attempts","/live/investigation"),Crumb::here("Attempt")])
        .with_seal(vec![Seal::status(&data.outcome)])
        .with_lede("The complete stored lifecycle of one admitted investigation.")
        .with_plate(Plate::new(vec![PlateRow::fact("Attempt id",id.to_string()),PlateRow::text("Created",time(data.created_at)),PlateRow::fact("Request id",data.request_id.to_string())]).with_action(vec![Action::copy("Copy id",id.to_string())]).with_authority("The attempt and its forward lifecycle links are primary-key reads. Reverse voiding is absent because that direction is not indexed."))
        .with_content(html! {
            section class="section section-first" { h2 { "Context" } dl class="meta" { dt { "Character" } dd { a href=(character_href(data.character.id)) { (&data.character.name) } } dt { "Place" } dd { a href=(place_href(data.place.id)) { (&data.place.name) } } dt { "Requesting User" } dd { a href=(user_href(data.requested_by_user_id)) { code class="fact" { (data.requested_by_user_id) } } } } }
            section class="section" { h2 { "Settlement" } dl class="meta" { dt { "Consumed by" } dd { @match data.consumed_by_activity_id { Some(activity)=>a href=(activity_href(activity)) { "Activity " code class="fact" { (activity) } },None=>span class="mute" { "Not consumed" } } } dt { "Voided by" } dd { @match data.voided_by_attempt_id { Some(attempt)=>a href=(attempt_href(attempt)) { "Attempt " code class="fact" { (attempt) } },None=>span class="mute" { "Not voided" } } } } }
        })
        .with_related(vec![Panel::list("Attempt links",vec![PanelItem::link("Attempt table","/live/storage/investigation_attempt")])])
        .with_reference(reference);
    context.render(page)
}

/// `/live/resolve` — exact-id resolver, never search.
pub(crate) async fn resolve_page(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
    Query(query): Query<ResolveQuery>,
) -> Response {
    let context = context_or_return!(&state, &header, &uri);
    let result = match query.id {
        Some(id) => match resolve::resolve(&state.pool, id).await {
            Ok(result) => Some(result),
            Err(error) => return read_error(&context, "Resolve an id", "/live/resolve", error),
        },
        None => None,
    };
    let page = live_page(&context,"Resolve an id","/live/resolve")
        .with_lede("Paste one complete UUID to find the durable World or provenance record that owns it. Names and partial ids are never searched.")
        .with_plate(Plate::new(vec![PlateRow::text("Lookup","Exact UUID only")]).with_authority("Every resolver probe is an exact primary-key lookup."))
        .with_content(html! {
            form class="toolbar" action="/live/resolve" method="get" role="search" { input type="search" name="id" aria-label="Exact UUID" placeholder="Exact UUID" value=[query.id.map(|id|id.to_string())]; button class="btn btn-small" type="submit" { "Resolve" } }
            @match result { Some(result) if result.hit.is_empty() => (empty_state("No durable record holds this exact id.")), Some(result) => { section class="section section-first" { h2 { "Matches" span class="count" { (result.hit.len()) } } ul class="list" role="list" { @for hit in result.hit { li { a href=(resolve_hit_href(&hit)) { (resolve_hit_label(&hit)) } small { (hit.lookup) } } } } } }, None => (empty_state("Enter a complete UUID to resolve it.")) }
        });
    context.render(page)
}

/// `/live/storage` — introspected schema first, with rows one level deeper.
pub(crate) async fn storage_index(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
) -> Response {
    let context = context_or_return!(&state, &header, &uri);
    let (snapshot, estimates) = tokio::join!(
        schema::read_storage(&state.pool),
        estimate::estimate(&state.pool)
    );
    let (snapshot, estimates) = match (snapshot, estimates) {
        (Ok(a), Ok(b)) => (a, b),
        (Err(error), _) | (_, Err(error)) => {
            return read_error(&context, "Schema", "/live/storage", error);
        }
    };
    let value = serde_json::to_value(&snapshot).expect("storage snapshot serializes");
    let tables = value
        .get("table")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let page=live_page(&context,"Schema","/live/storage")
        .with_lede("Introspected application tables and their structure. Open a table for its schema and bounded rows.")
        .with_plate(Plate::new(vec![PlateRow::text("Tables",tables.len().to_string()),PlateRow::text("Row counts",estimates.scope),PlateRow::fact("Fingerprint",value_string(&value,"fingerprint"))]).with_action(vec![Action::link("Download snapshot","/live/storage/snapshot.json")]).with_authority("Schema comes from PostgreSQL catalogs; row counts are planner estimates, never exact counts."))
        .with_content(html! { (loaded_filter("storage-list","Filter loaded tables")) section class="section section-first" { h2 { "Application tables" } (storage_table_list(&tables,&estimates.table)) } });
    context.render(page)
}

/// `/live/storage/snapshot.json` — the introspected schema as a download, not an
/// alternate client application API.
pub(crate) async fn storage_snapshot(
    State(state): State<StudioState>,
) -> Result<Response, StudioError> {
    let snapshot = schema::read_storage(&state.pool).await?;
    let body = serde_json::to_string_pretty(&snapshot)
        .expect("the serializable Studio schema snapshot must encode as JSON");
    Ok((
        [
            (header::CONTENT_TYPE, "application/json; charset=utf-8"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"aicadia-schema-snapshot.json\"",
            ),
        ],
        body,
    )
        .into_response())
}

/// `/live/storage/{table}` — one table's schema and bounded rows.
pub(crate) async fn storage_table(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
    Path(table): Path<String>,
    Query(query): Query<RowQuery>,
) -> Response {
    let context = context_or_return!(&state, &header, &uri);
    let path = format!("/live/storage/{table}");
    let bound = bound_or_return!(&context, query.limit, &table, &path);
    let after = match query.after.as_deref() {
        Some(encoded) => match decode_cursor(encoded) {
            Some(cursor) => Some(cursor),
            None => return invalid_cursor(&context, &table, &path),
        },
        None => None,
    };
    let (snapshot, rows) = tokio::join!(
        schema::read_storage(&state.pool),
        row::list_row(&state.pool, &table, after.as_deref(), bound)
    );
    let (snapshot, rows) = match (snapshot, rows) {
        (Ok(a), Ok(b)) => (a, b),
        (Err(error), _) | (_, Err(error)) => return read_error(&context, &table, &path, error),
    };
    let value = serde_json::to_value(&snapshot).expect("storage snapshot serializes");
    let table_value = value
        .get("table")
        .and_then(Value::as_array)
        .and_then(|items| {
            items
                .iter()
                .find(|item| value_string(item, "name") == table)
        });
    let Some(table_value) = table_value else {
        return not_found(&context, "table", &table, &path);
    };
    let next = rows.next_cursor.as_ref().map(|cursor| {
        format!(
            "{path}?after={}&limit={}",
            encode_cursor(cursor),
            bound.limit()
        )
    });
    let reference = reference(
        &context,
        &table,
        &path,
        &format!("PostgreSQL table {table}"),
    );
    let page=live_page(&context,&table,&path)
        .with_crumb(vec![Crumb::link("Live","/live"),Crumb::link("Schema","/live/storage"),Crumb::here(&table)])
        .with_lede("Introspected structure followed by one bounded primary-key page of rows.")
        .with_plate(Plate::new(vec![PlateRow::text("Columns",value_array(table_value,"column").len().to_string()),PlateRow::text("Primary key",rows.primary_key.join(", ")),PlateRow::text("Rows loaded",rows.row.len().to_string())]).with_authority("Rows are ordered and keyset-paged by the introspected primary key. Row browsing fails closed when a table has no primary key."))
        .with_content(html! {
            section class="section section-first" { h2 { "Columns" } (schema_column_table(value_array(table_value,"column"))) }
            section class="section" { h2 { "Constraints and indexes" } (schema_rule_table(table_value)) }
            section class="section" id="rows" { h2 { "Rows" span class="count" { (rows.row.len()) } } (loaded_filter("row-list","Filter loaded rows")) (row_table(&rows)) (truncation(&rows.truncated,next)) }
        })
        .with_related(vec![Panel::list("Storage links",vec![PanelItem::link("All tables","/live/storage"),PanelItem::link("Migrations","/live/migration"),PanelItem::link("Game storage contract","/game/storage")])])
        .with_reference(reference);
    context.render(page)
}

/// `/live/migration` — applied database state joined to repository files.
pub(crate) async fn migration_page(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
) -> Response {
    let context = context_or_return!(&state, &header, &uri);
    let data = match migration::list_migration(&state.pool, state.repository_root.as_path()).await {
        Ok(data) => data,
        Err(error) => return read_error(&context, "Migrations", "/live/migration", error),
    };
    let page=live_page(&context,"Migrations","/live/migration")
        .with_lede("Applied PostgreSQL migrations joined by version to the migration files owned by this repository.")
        .with_plate(Plate::new(vec![PlateRow::text("Applied",data.applied.len().to_string()),PlateRow::text("Unapplied files",data.unapplied_file.len().to_string())]).with_authority("Applied state comes from _sqlx_migrations; file names come directly from migration/. Migration content remains owned by its repository file."))
        .with_content(html! { section class="section section-first" { h2 { "Applied migrations" } (migration_table(&data.applied)) (truncated_only(data.truncated)) } @if !data.unapplied_file.is_empty(){section class="section"{h2{"Unapplied files"}ul class="list" role="list"{@for file in &data.unapplied_file{li{code class="fact"{(file)}small{"repository only"}}}}}} });
    context.render(page)
}

fn live_page(context: &Context, title: &str, path: &str) -> Page {
    let page = Page::new(Section::Live, title);
    let page = if path == LIVE_PATH {
        page.with_document_title("Live · Aicadia Studio")
    } else {
        page
    };
    page.with_crumb(vec![Crumb::link("Live", "/live"), Crumb::here(title)])
        .with_reference(reference(context, title, path, "connected local World"))
}

fn list_page(
    context: &Context,
    title: &str,
    path: &str,
    subject: &str,
    loaded: usize,
    truncated: bool,
) -> Page {
    live_page(context,title,path)
        .with_seal(if truncated { vec![Seal::status("Truncated")] } else { Vec::new() })
        .with_plate(Plate::new(vec![PlateRow::text("Loaded",count(loaded,subject,&format!("{subject}s"))),PlateRow::text("Limit","1–100")]).with_authority("The list is keyset-paged over an existing index or primary key. A loaded-row filter never searches beyond this page."))
}

fn unavailable_page(context: &Context, title: &str, path: &str) -> Page {
    live_page(context,title,path)
        .with_seal(vec![Seal::status("Unavailable")])
        .with_lede("Live World data is unavailable. Repository and compiled Game pages remain usable.")
        .with_content(note(Tone::Brick,"Studio could not reach the configured PostgreSQL database. No cached or remembered World state is shown."))
}

fn read_error(context: &Context, title: &str, path: &str, error: StudioError) -> Response {
    match error {
        StudioError::NotFound => not_found(context, "World record", title, path),
        StudioError::InvalidLimit => context.render_status(
            live_page(context, title, path)
                .with_seal(vec![Seal::status("Invalid request")])
                .with_content(note(
                    Tone::Brick,
                    "The page limit must be between 1 and 100.",
                )),
            StatusCode::BAD_REQUEST,
        ),
        StudioError::UnpageableTable => context.render_status(
            live_page(context, title, path)
                .with_seal(vec![Seal::status("Unavailable")])
                .with_lede("Rows cannot be browsed because this table has no primary key.")
                .with_content(note(
                    Tone::Amber,
                    "Studio fails closed instead of scanning or exposing unstable physical row order.",
                )),
            StatusCode::CONFLICT,
        ),
        other => {
            eprintln!("Studio Live page read failed: {other:?}");
            context.render_status(
                unavailable_page(context, title, path),
                StatusCode::SERVICE_UNAVAILABLE,
            )
        }
    }
}

fn not_found(context: &Context, subject: &str, value: &str, path: &str) -> Response {
    context.render_status(
        live_page(context, "Not found", path)
            .with_seal(vec![Seal::status("Not found")])
            .with_lede(format!("No {subject} matches {value}."))
            .with_content(note(
                Tone::Brick,
                "The connected World did not return this exact record.",
            )),
        StatusCode::NOT_FOUND,
    )
}

fn invalid_cursor(context: &Context, title: &str, path: &str) -> Response {
    context.render_status(live_page(context,title,path).with_seal(vec![Seal::status("Invalid request")]).with_content(note(Tone::Brick,"The load-more cursor is invalid. Return to the table and start from its first page.")),StatusCode::BAD_REQUEST)
}

fn reference(context: &Context, title: &str, path: &str, detail: &str) -> Reference {
    Reference {
        title: title.to_owned(),
        url: context.url(path),
        context: detail.to_owned(),
    }
}
fn time(value: DateTime<Utc>) -> String {
    value.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}
fn estimate_label(value: Option<i64>) -> String {
    value
        .map(|value| format!("~{value}"))
        .unwrap_or_else(|| "not analyzed".to_owned())
}
fn time_cursor(at: Option<i64>, id: Option<Uuid>) -> Option<(DateTime<Utc>, Uuid)> {
    Some((DateTime::from_timestamp_millis(at?)?, id?))
}

fn loaded_filter(id: &str, label: &str) -> Markup {
    html! {div class="toolbar"{input type="search" name="filter" aria-label=(label) placeholder=(label) data-filter-rows=(id);span class="data-note" data-filter-note=(id) data-filter-all="Filtering applies only to the loaded rows."{"Filtering applies only to the loaded rows."}}}
}
fn loaded_operation_filter(id: &str) -> Markup {
    html! {div class="toolbar"{input type="search" name="operation-filter" aria-label="Filter loaded Activities by operation" placeholder="Filter loaded operations" data-filter-rows=(id);span class="data-note" data-filter-note=(id) data-filter-all="Operation filtering applies only to loaded rows and never changes continuation."{"Operation filtering applies only to loaded rows and never changes continuation."}}}
}
fn truncation(truncated: &bool, next: Option<String>) -> Markup {
    html! {@if *truncated{p class="truncated"{"This page is truncated. "}@match next{Some(href)=>a class="btn btn-small" href=(href){"Load more"},None=>span{"No stable continuation is available."}}}}
}
fn truncated_only(truncated: bool) -> Markup {
    truncation(&truncated, None)
}

fn id_markup(id: Uuid) -> Markup {
    html! {span class="id"{code{(id)}button class="btn btn-small btn-quiet" type="button" data-copy=(id){"Copy"}}}
}
fn value_markup(text_value: &Option<String>, integer_value: Option<i64>) -> Markup {
    html! {@match(text_value,integer_value){(Some(text),_)=>span{(text)},(_,Some(integer))=>code class="fact"{(integer)},_=>span class="mute"{"Absent"}}}
}
fn table_empty(columns: usize, message: &str) -> Markup {
    html! {tr{td colspan=(columns){(empty_state(message))}}}
}

fn place_table(items: &[place::PlaceListItem]) -> Markup {
    html! {div class="data-wrap"{table class="data" id="place-list"{thead{tr{th{"Place"}th{"Entry"}th{"Introduced"}}}tbody{@if items.is_empty(){(table_empty(3,"No Places are present."))}@for item in items{tr data-live="place"{td{a class="row-link" href=(place_href(item.id)){(&item.name)}p class="data-note"{(&item.description)}}td{@if item.is_entry{span class="stamp"{"Entry"}}@else{span class="mute"{"—"}}}td{(time(item.introduced_at))}}}}}}}
}
fn character_table(items: &[character::CharacterListItem]) -> Markup {
    html! {div class="data-wrap"{table class="data" id="character-list"{thead{tr{th{"Character"}th{"Current Place"}th{"Owner User"}}}tbody{@if items.is_empty(){(table_empty(3,"No Characters are present."))}@for item in items{tr{td{a class="row-link" href=(character_href(item.id)){(&item.name)}}td{@match(item.current_place_entity_id,&item.current_place_name){(Some(id),Some(name))=>a href=(place_href(id)){(name)},_=>span class="mute"{"Not entered"}}}td{a href=(user_href(item.owner_user_id)){code class="fact"{(item.owner_user_id)}}}}}}}}}
}
fn entity_table(items: &[entity::EntityListItem]) -> Markup {
    html! {div class="data-wrap"{table class="data" id="entity-list"{thead{tr{th{"Entity"}th{"Roles"}th{"Introduced"}}}tbody{@if items.is_empty(){(table_empty(3,"No Entities are present."))}@for item in items{tr{td{a class="row-link" href=(entity_href(item.id)){(&item.name)}div{(id_markup(item.id))}}td{@if item.is_character{span class="stamp"{"Character"}" "}@if item.is_place{span class="stamp"{"Place"}}@if !item.is_character&&!item.is_place{span class="mute"{"Entity"}}}td{(time(item.introduced_at))}}}}}}}
}
fn user_table(items: &[user::UserListItem]) -> Markup {
    html! {div class="data-wrap"{table class="data"{thead{tr{th{"User"}th{"Character"}th{"Created"}}}tbody{@if items.is_empty(){(table_empty(3,"No Users are present."))}@for item in items{tr{td{a href=(user_href(item.id)){(id_markup(item.id))}}td{@match(item.character_entity_id,&item.character_name){(Some(id),Some(name))=>a class="row-link" href=(character_href(id)){(name)},_=>span class="mute"{"No Character"}}}td{(time(item.created_at))}}}}}}}
}
fn property_key_table(items: &[property::PropertyKeyItem]) -> Markup {
    html! {div class="data-wrap"{table class="data" id="property-key-list"{thead{tr{th{"Property key"}th{"Type"}th{"First Activity"}}}tbody{@if items.is_empty(){(table_empty(3,"No Property keys are present."))}@for item in items{tr{td{a class="row-link" href=(property_key_href(&item.key)){(&item.key)}}td{code class="fact"{(&item.value_type)}}td{a href=(activity_href(item.first_activity_id)){(id_markup(item.first_activity_id))}}}}}}}}
}
fn attempt_table(items: &[investigation::AttemptItem]) -> Markup {
    html! {div class="data-wrap"{table class="data" id="attempt-list"{thead{tr{th{"Attempt"}th{"Character and Place"}th{"Outcome"}th{"Created"}}}tbody{@if items.is_empty(){(table_empty(4,"No investigation attempts are present."))}@for item in items{tr{td{a href=(attempt_href(item.id)){(id_markup(item.id))}}td{a class="row-link" href=(character_href(item.character_entity_id)){(&item.character_name)}span{" at "}a href=(place_href(item.place_entity_id)){(&item.place_name)}}td{span class="stamp"{(&item.outcome)}}td{(time(item.created_at))}}}}}}}
}
fn chronicle_table(items: &[chronicle::ChronicleItem], id: &str) -> Markup {
    html! {div class="data-wrap"{table class="data" id=(id){thead{tr{th{"Activity"}th{"Context"}th{"Occurred"}}}tbody{@if items.is_empty(){(table_empty(3,"No Activities are present."))}@for item in items{tr{td{a class="row-link" href=(activity_href(item.id)){(&item.operation)}@if let Some(prose)=&item.prose{p class="data-note"{(prose)@if item.prose_truncated{"…"}}}}td{@if let Some(actor)=&item.actor_character{a href=(character_href(actor.id)){(&actor.name)}}@if item.actor_character.is_some()&&item.context_place.is_some(){" · "}@if let Some(place)=&item.context_place{a href=(place_href(place.id)){(&place.name)}}}td{(time(item.occurred_at))}}}}}}}
}

fn place_entity_table(items: &[place::PlaceEntityItem]) -> Markup {
    html! {div class="data-wrap"{table class="data"{thead{tr{th{"Entity"}th{"Role"}th{"Introduced"}}}tbody{@if items.is_empty(){(table_empty(3,"No Entities are located here."))}@for item in items{tr{td{a class="row-link" href=(entity_href(item.id)){(&item.name)}}td{@if item.is_character{"Character"}@else if item.is_place{"Place"}@else{"Entity"}}td{(time(item.introduced_at))}}}}}}}
}
fn character_membership_table(items: &[place::PlaceCharacterItem]) -> Markup {
    html! {div class="data-wrap"{table class="data"{thead{tr{th{"Character"}th{"Owner User"}}}tbody{@if items.is_empty(){(table_empty(2,"No Characters are currently here."))}@for item in items{tr{td{a class="row-link" href=(character_href(item.id)){(&item.name)}}td{a href=(user_href(item.owner_user_id)){code class="fact"{(item.owner_user_id)}}}}}}}}}
}
fn entity_property_table(entity_id: Uuid, items: &[entity::EntityProperty]) -> Markup {
    html! {div class="data-wrap"{table class="data"{thead{tr{th{"Property"}th{"Current value"}th{"Versions"}}}tbody{@if items.is_empty(){(table_empty(3,"This Entity has no current Properties."))}@for item in items{tr{td{a class="row-link" href=(format!("/live/entity/{entity_id}/property/{}",item.property_key_id)){(&item.key)}}td{(value_markup(&item.text_value,item.integer_value))}td class="num"{(item.version_count)}}}}}}}
}
fn entity_trait_table(items: &[entity::EntityTraitCurrent]) -> Markup {
    html! {div class="data-wrap"{table class="data"{thead{tr{th{"Trait"}th{"Current Activity"}}}tbody{@if items.is_empty(){(table_empty(2,"This Entity has no current Traits."))}@for item in items{tr{td{a class="row-link" href=(trait_href(item.id)){(&item.statement)}}td{a href=(activity_href(item.current_activity_id)){(id_markup(item.current_activity_id))}}}}}}}}
}
fn participation_table(items: &[entity::ParticipationItem]) -> Markup {
    html! {div class="data-wrap"{table class="data"{thead{tr{th{"Activity"}th{"Role"}th{"Occurred"}}}tbody{@if items.is_empty(){(table_empty(3,"This Entity has no recorded Activity participation."))}@for item in items{tr{td{a class="row-link" href=(activity_href(item.activity_id)){(&item.operation)}}td{span class="stamp"{(&item.role)}}td{(time(item.occurred_at))}}}}}}}
}
fn property_version_table(items: &[entity::PropertyVersion]) -> Markup {
    html! {div class="data-wrap"{table class="data"{thead{tr{th{"Value"}th{"Activity"}th{"Occurred"}th{"State"}}}tbody{@for item in items{tr{td{(value_markup(&item.text_value,item.integer_value))}td{a href=(activity_href(item.activity_id)){(&item.operation)}}td{(time(item.occurred_at))}td{@if item.is_current{span class="stamp"{"Current"}}@else{span class="mute"{"Historical"}}}}}}}}}
}
fn trait_version_table(items: &[r#trait::TraitVersion]) -> Markup {
    html! {div class="data-wrap"{table class="data"{thead{tr{th{"Statement"}th{"Activity"}th{"Occurred"}th{"State"}}}tbody{@for item in items{tr{td{(&item.statement)}td{a href=(activity_href(item.activity_id)){(&item.operation)}}td{(time(item.occurred_at))}td{@if item.is_current{span class="stamp"{"Current"}}@else if item.is_root{span class="stamp"{"Root"}}@else{span class="mute"{"Historical"}}}}}}}}}
}
fn involved_table(items: &[chronicle::InvolvedEntity]) -> Markup {
    html! {div class="data-wrap"{table class="data"{thead{tr{th{"Entity"}th{"Role"}}}tbody{@if items.is_empty(){(table_empty(2,"No involved Entities were recorded."))}@for item in items{tr{td{a class="row-link" href=(entity_href(item.entity_id)){(&item.name)}}td{span class="stamp"{(&item.role)}}}}}}}}
}
fn activity_property_table(items: &[chronicle::ActivityPropertyChange]) -> Markup {
    html! {div class="data-wrap"{table class="data"{thead{tr{th{"Entity"}th{"Property"}th{"Accepted value"}}}tbody{@for item in items{tr{td{a href=(entity_href(item.entity_id)){(&item.entity_name)}}td{a class="row-link" href=(format!("/live/entity/{}/property/{}",item.entity_id,item.property_key_id)){(&item.key)}}td{(value_markup(&item.text_value,item.integer_value))}}}}}}}
}
fn activity_trait_table(items: &[chronicle::ActivityTraitChange]) -> Markup {
    html! {div class="data-wrap"{table class="data"{thead{tr{th{"Entity"}th{"Trait"}th{"Accepted statement"}}}tbody{@for item in items{tr{td{a href=(entity_href(item.entity_id)){(&item.entity_name)}}td{a class="row-link" href=(trait_href(item.trait_id)){(id_markup(item.trait_id))}}td{(&item.statement)}}}}}}}
}
fn storage_table_list(items: &[Value], estimates: &[estimate::TableEstimate]) -> Markup {
    html! {div class="data-wrap"{table class="data" id="storage-list"{thead{tr{th{"Table"}th class="num"{"Planner estimate"}th class="num"{"Columns"}}}tbody{@for item in items{ @let name=value_string(item,"name"); @let estimate=estimates.iter().find(|row|row.table==name).and_then(|row|row.row_estimate); tr{td{a class="row-link" href=(format!("/live/storage/{name}")){(&name)}}td class="num"{(estimate_label(estimate))}td class="num"{(value_array(item,"column").len())}}}}}}}
}
fn schema_column_table(items: &[Value]) -> Markup {
    html! {div class="data-wrap"{table class="data"{thead{tr{th{"Column"}th{"Type"}th{"Null"}th{"Default"}}}tbody{@for item in items{tr data-column=(value_string(item,"name")){td{code class="fact"{(value_string(item,"name"))}}td{(value_string(item,"data_type"))}td{(if value_bool(item,"nullable"){"yes"}else{"no"})}td class="fact"{(value_optional_string(item,"default_value").unwrap_or_else(||"—".to_owned()))}}}}}}}
}
fn schema_rule_table(table: &Value) -> Markup {
    let mut rules = value_array(table, "constraint").to_vec();
    rules.extend_from_slice(value_array(table, "index"));
    html! {div class="data-wrap"{table class="data"{thead{tr{th{"Name"}th{"Definition"}}}tbody{@if rules.is_empty(){(table_empty(2,"No constraints or indexes are present."))}@for item in rules{tr{td{code class="fact"{(value_string(&item,"name"))}}td class="fact"{(value_string(&item,"definition"))}}}}}}}
}
fn row_table(page: &row::RowPage) -> Markup {
    html! {div class="data-wrap"{table class="data" id="row-list"{thead{tr{@for column in &page.column{th{(column)}}}}tbody{@if page.row.is_empty(){(table_empty(page.column.len().max(1),"This table has no rows."))}@for record in &page.row{tr id=[row_anchor(page,record)] data-row="true"{@for column in &page.column{td class="fact"{(json_cell(record.get(column)))} }}}}}}}
}

fn row_anchor(page: &row::RowPage, record: &Value) -> Option<String> {
    let cursor = page
        .primary_key
        .iter()
        .map(|column| json_cell(record.get(column)))
        .collect::<Vec<_>>();
    (!cursor.is_empty()).then(|| format!("row-{}", encode_cursor(&cursor)))
}
fn migration_table(items: &[migration::AppliedMigration]) -> Markup {
    html! {div class="data-wrap"{table class="data"{thead{tr{th class="num"{"Version"}th{"Migration"}th{"Applied"}th{"State"}}}tbody{@for item in items{tr{td class="num"{(item.version)}td{(&item.description)p class="data-note"{(item.file.as_deref().unwrap_or("No repository file"))}}td{(time(item.installed_on))}td{@if item.success{span class="stamp"{"Applied"}}@else{span class="stamp"{"Failed"}}}}}}}}}
}

fn value_string(value: &Value, key: &str) -> String {
    value
        .get(key)
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned()
}
fn value_optional_string(value: &Value, key: &str) -> Option<String> {
    value.get(key).and_then(Value::as_str).map(str::to_owned)
}
fn value_bool(value: &Value, key: &str) -> bool {
    value.get(key).and_then(Value::as_bool).unwrap_or(false)
}
fn value_array<'a>(value: &'a Value, key: &str) -> &'a [Value] {
    value
        .get(key)
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or(&[])
}
fn json_cell(value: Option<&Value>) -> String {
    match value {
        None | Some(Value::Null) => "null".to_owned(),
        Some(Value::String(value)) => value.clone(),
        Some(value) => serde_json::to_string(value).unwrap_or_else(|_| "unavailable".to_owned()),
    }
}

fn entity_related(id: Uuid, data: &entity::EntityDetail) -> Vec<PanelItem> {
    let mut item = vec![PanelItem::link(
        "Entity table row",
        format!(
            "/live/storage/entity#row-{}",
            encode_cursor(&[id.to_string()])
        ),
    )];
    if data.entity.is_character {
        item.push(PanelItem::link("Character detail", character_href(id)));
    }
    if data.entity.is_place {
        item.push(PanelItem::link("Place detail", place_href(id)));
    }
    item
}
fn activity_related(data: &chronicle::ActivityDetail) -> Vec<PanelItem> {
    let mut item = vec![PanelItem::link(
        "Activity table row",
        format!(
            "/live/storage/activity#row-{}",
            encode_cursor(&[data.id.to_string()])
        ),
    )];
    if let Some(actor) = &data.actor_character {
        item.push(PanelItem::link("Actor Character", character_href(actor.id)));
    }
    if let Some(place) = &data.context_place {
        item.push(PanelItem::link("Context Place", place_href(place.id)));
    }
    item
}

fn place_href(id: Uuid) -> String {
    format!("/live/place/{id}")
}
fn character_href(id: Uuid) -> String {
    format!("/live/character/{id}")
}
fn entity_href(id: Uuid) -> String {
    format!("/live/entity/{id}")
}
fn activity_href(id: Uuid) -> String {
    format!("/live/activity/{id}")
}
fn user_href(id: Uuid) -> String {
    format!("/live/user/{id}")
}
fn trait_href(id: Uuid) -> String {
    format!("/live/trait/{id}")
}
fn attempt_href(id: Uuid) -> String {
    format!("/live/investigation/{id}")
}
fn property_key_href(key: &str) -> String {
    format!("/live/property-key/{}", encode_path_segment(key))
}
fn world_cursor_href(cursor: chronicle::ChronicleCursor) -> String {
    world_cursor_href_with_limit(cursor, 24)
}
fn world_cursor_href_with_limit(cursor: chronicle::ChronicleCursor, limit: usize) -> String {
    format!(
        "/live/activity?before_at={}&before={}&limit={limit}",
        cursor.occurred_at.timestamp_millis(),
        cursor.id
    )
}

fn encode_text(value: &str) -> String {
    encode_path_segment(value)
}
fn encode_path_segment(value: &str) -> String {
    let mut encoded = String::new();
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~') {
            encoded.push(char::from(byte));
        } else {
            encoded.push_str(&format!("%{byte:02X}"));
        }
    }
    encoded
}
fn encode_cursor(value: &[String]) -> String {
    URL_SAFE_NO_PAD.encode(serde_json::to_vec(value).expect("cursor serializes"))
}
fn decode_cursor(value: &str) -> Option<Vec<String>> {
    serde_json::from_slice(&URL_SAFE_NO_PAD.decode(value).ok()?).ok()
}
fn encode_participation(value: &entity::ParticipationCursor) -> String {
    encode_cursor(&[value.activity_id.to_string(), value.role.clone()])
}
fn decode_participation(value: &str) -> Option<entity::ParticipationCursor> {
    let value = decode_cursor(value)?;
    Some(entity::ParticipationCursor {
        activity_id: value.first()?.parse().ok()?,
        role: value.get(1)?.clone(),
    })
}

fn resolve_hit_href(hit: &resolve::ResolveHit) -> String {
    match hit.subject {
        "entity" => entity_href(hit.id),
        "activity" | "activity request" => activity_href(hit.id),
        "user" => user_href(hit.id),
        "trait" => trait_href(hit.id),
        "investigation attempt" | "investigation attempt request" => attempt_href(hit.id),
        _ => "/live/resolve".to_owned(),
    }
}
fn resolve_hit_label(hit: &resolve::ResolveHit) -> String {
    let suffix = hit.name.as_deref().or(hit.detail.as_deref()).unwrap_or("");
    if suffix.is_empty() {
        format!("{} · {}", hit.subject, hit.id)
    } else {
        format!("{} · {suffix}", hit.subject)
    }
}
