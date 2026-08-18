//! User: the durable participant and request-provenance subject.

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use super::entity::EntityRef;
use super::page::{Bound, Page};
use crate::StudioError;

/// Why a User page does not list the Entities that User introduced.
pub const INTRODUCED_ENTITY_NOTE: &str = "Entities introduced by a User are not listed: \
entity.introduced_by_user_id carries no index, and current game behavior never reads it that \
way. Entity detail names its introducing User.";

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct UserListItem {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub character_entity_id: Option<Uuid>,
    pub character_name: Option<String>,
}

/// Every User, keyset by the `user.id` primary key.
///
/// `user` carries no index other than its primary key — `created_at` is provenance,
/// not an ordering current behavior reads — so the primary key is the ordering.
/// The Character join uses the unique `character_owner_user_id_key`.
pub async fn list_user(
    pool: &PgPool,
    after: Option<Uuid>,
    bound: Bound,
) -> Result<Page<UserListItem, Uuid>, StudioError> {
    let item = sqlx::query_as::<_, UserListItem>(
        r#"
        SELECT "user".id,
               "user".created_at,
               character.entity_id AS character_entity_id,
               character_entity.name AS character_name
        FROM "user"
        LEFT JOIN character ON character.owner_user_id = "user".id
        LEFT JOIN entity AS character_entity ON character_entity.id = character.entity_id
        WHERE ($1::uuid IS NULL OR "user".id > $1::uuid)
        ORDER BY "user".id
        LIMIT $2
        "#,
    )
    .bind(after)
    .bind(bound.fetch())
    .fetch_all(pool)
    .await?;
    Ok(Page::build(item, bound, |last| last.id))
}

#[derive(Debug, Serialize)]
pub struct UserDetail {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub character: Option<EntityRef>,
    pub character_current_place: Option<EntityRef>,
    /// States, on the page itself, why introduced Entities are absent.
    pub introduced_entity_note: &'static str,
}

#[derive(sqlx::FromRow)]
struct UserRow {
    id: Uuid,
    created_at: DateTime<Utc>,
    character_entity_id: Option<Uuid>,
    character_name: Option<String>,
    current_place_entity_id: Option<Uuid>,
    current_place_name: Option<String>,
}

/// One User with the single Character it may own.
///
/// The Character lookup uses the unique `character_owner_user_id_key`; the
/// Entity and Place lookups are primary keys.
///
/// This read deliberately does **not** list the Entities the User introduced.
/// `entity.introduced_by_user_id` has no index, so that list would be a full
/// `entity` scan, and no current game behavior earns such an index. The
/// introducing User is shown from the other side, on Entity detail, and
/// [`INTRODUCED_ENTITY_NOTE`] travels with this value so the page can say so.
///
/// This User's investigation attempts are a separate indexed read:
/// [`super::investigation::list_user_attempt`] with this `id`.
pub async fn get_user(pool: &PgPool, user_id: Uuid) -> Result<UserDetail, StudioError> {
    let row = sqlx::query_as::<_, UserRow>(
        r#"
        SELECT "user".id,
               "user".created_at,
               character.entity_id AS character_entity_id,
               character_entity.name AS character_name,
               character.current_place_entity_id,
               current_place.name AS current_place_name
        FROM "user"
        LEFT JOIN character ON character.owner_user_id = "user".id
        LEFT JOIN entity AS character_entity ON character_entity.id = character.entity_id
        LEFT JOIN entity AS current_place ON current_place.id = character.current_place_entity_id
        WHERE "user".id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?
    .ok_or(StudioError::NotFound)?;
    Ok(UserDetail {
        id: row.id,
        created_at: row.created_at,
        character: match (row.character_entity_id, row.character_name) {
            (Some(id), Some(name)) => Some(EntityRef { id, name }),
            _ => None,
        },
        character_current_place: match (row.current_place_entity_id, row.current_place_name) {
            (Some(id), Some(name)) => Some(EntityRef { id, name }),
            _ => None,
        },
        introduced_entity_note: INTRODUCED_ENTITY_NOTE,
    })
}
