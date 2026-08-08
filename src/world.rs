use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool};
use thiserror::Error;
use uuid::Uuid;

const MAX_ENTITY_NAME_LENGTH: usize = 120;
const MAX_ENTITY_DESCRIPTION_LENGTH: usize = 4_000;
const DEFAULT_ENTITY_PAGE_SIZE: u16 = 25;
const MAX_ENTITY_PAGE_SIZE: u16 = 100;
const WORLD_NAME: &str = "Aicadia";

#[derive(Clone)]
pub struct World {
    pool: PgPool,
}

impl World {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn get_world(&self) -> WorldView {
        WorldView {
            name: WORLD_NAME.to_owned(),
        }
    }

    pub async fn create_user(&self) -> Result<User, WorldError> {
        sqlx::query_as::<_, User>(
            r#"
            INSERT INTO "user" (id)
            VALUES ($1)
            RETURNING id, created_at
            "#,
        )
        .bind(Uuid::new_v4())
        .fetch_one(&self.pool)
        .await
        .map_err(|error| storage_error("create_user", error))
    }

    pub async fn get_user(&self, user_id: UserId) -> Result<User, WorldError> {
        sqlx::query_as::<_, User>(
            r#"
            SELECT id, created_at
            FROM "user"
            WHERE id = $1
            "#,
        )
        .bind(user_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|error| storage_error("get_user", error))?
        .ok_or(WorldError::UserNotFound)
    }

    pub async fn list_entity(&self, request: ListEntity) -> Result<EntityPage, WorldError> {
        if request.limit == 0 || request.limit > MAX_ENTITY_PAGE_SIZE {
            return Err(WorldError::InvalidEntityLimit);
        }

        let cursor_time = request.cursor.as_ref().map(|cursor| cursor.introduced_at);
        let cursor_id = request.cursor.as_ref().map(|cursor| cursor.entity_id.0);
        let fetch_limit = i64::from(request.limit) + 1;

        let mut row = sqlx::query_as::<_, EntityListRow>(
            r#"
            SELECT id, name, introduced_at
            FROM entity
            WHERE $1::timestamptz IS NULL
               OR (introduced_at, id) < ($1::timestamptz, $2::uuid)
            ORDER BY introduced_at DESC, id DESC
            LIMIT $3
            "#,
        )
        .bind(cursor_time)
        .bind(cursor_id)
        .bind(fetch_limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|error| storage_error("list_entity", error))?;

        let has_more = row.len() > usize::from(request.limit);
        if has_more {
            row.pop();
        }

        let next = has_more.then(|| {
            let last = row
                .last()
                .expect("a page with another row always has a returned row");
            EntityCursor {
                introduced_at: last.introduced_at,
                entity_id: last.id,
            }
        });
        let entity = row
            .into_iter()
            .map(|row| EntitySummary {
                id: row.id,
                name: row.name,
            })
            .collect();

        Ok(EntityPage { entity, next })
    }

    pub async fn get_entity(&self, entity_id: EntityId) -> Result<Entity, WorldError> {
        sqlx::query_as::<_, Entity>(
            r#"
            SELECT id, name, description, introduced_by_user_id, introduced_at
            FROM entity
            WHERE id = $1
            "#,
        )
        .bind(entity_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|error| storage_error("get_entity", error))?
        .ok_or(WorldError::EntityNotFound)
    }

    pub async fn create_entity(
        &self,
        introduced_by_user_id: UserId,
        input: CreateEntity,
    ) -> Result<Entity, WorldError> {
        let input = input.normalize()?;

        sqlx::query_as::<_, Entity>(
            r#"
            INSERT INTO entity (id, name, description, introduced_by_user_id)
            SELECT $1, $2, $3, id
            FROM "user"
            WHERE id = $4
            RETURNING id, name, description, introduced_by_user_id, introduced_at
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(input.name)
        .bind(input.description)
        .bind(introduced_by_user_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|error| storage_error("create_entity", error))?
        .ok_or(WorldError::UserNotFound)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, sqlx::Type)]
#[sqlx(transparent)]
pub struct UserId(pub Uuid);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, sqlx::Type)]
#[sqlx(transparent)]
pub struct EntityId(pub Uuid);

#[derive(Clone, Debug, PartialEq, Eq, FromRow)]
pub struct User {
    pub id: UserId,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Entity {
    pub id: EntityId,
    pub name: String,
    pub description: String,
    pub introduced_by_user_id: UserId,
    pub introduced_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntitySummary {
    pub id: EntityId,
    pub name: String,
}

#[derive(FromRow)]
struct EntityListRow {
    id: EntityId,
    name: String,
    introduced_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorldView {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateEntity {
    pub name: String,
    pub description: String,
}

impl CreateEntity {
    fn normalize(self) -> Result<Self, WorldError> {
        let name = self.name.trim().to_owned();
        let description = self.description.trim().to_owned();

        if name.is_empty() {
            return Err(WorldError::InvalidEntity {
                field: EntityField::Name,
                reason: InvalidReason::Empty,
            });
        }

        if name.contains('\0') {
            return Err(WorldError::InvalidEntity {
                field: EntityField::Name,
                reason: InvalidReason::ContainsNul,
            });
        }

        if name.chars().count() > MAX_ENTITY_NAME_LENGTH {
            return Err(WorldError::InvalidEntity {
                field: EntityField::Name,
                reason: InvalidReason::TooLong,
            });
        }

        if description.is_empty() {
            return Err(WorldError::InvalidEntity {
                field: EntityField::Description,
                reason: InvalidReason::Empty,
            });
        }

        if description.contains('\0') {
            return Err(WorldError::InvalidEntity {
                field: EntityField::Description,
                reason: InvalidReason::ContainsNul,
            });
        }

        if description.chars().count() > MAX_ENTITY_DESCRIPTION_LENGTH {
            return Err(WorldError::InvalidEntity {
                field: EntityField::Description,
                reason: InvalidReason::TooLong,
            });
        }

        Ok(Self { name, description })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListEntity {
    pub cursor: Option<EntityCursor>,
    pub limit: u16,
}

impl Default for ListEntity {
    fn default() -> Self {
        Self {
            cursor: None,
            limit: DEFAULT_ENTITY_PAGE_SIZE,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EntityCursor {
    pub introduced_at: DateTime<Utc>,
    pub entity_id: EntityId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityPage {
    pub entity: Vec<EntitySummary>,
    pub next: Option<EntityCursor>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntityField {
    Name,
    Description,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InvalidReason {
    Empty,
    ContainsNul,
    TooLong,
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum WorldError {
    #[error("entity input is invalid")]
    InvalidEntity {
        field: EntityField,
        reason: InvalidReason,
    },
    #[error("entity list limit must be between 1 and 100")]
    InvalidEntityLimit,
    #[error("user was not found")]
    UserNotFound,
    #[error("entity was not found")]
    EntityNotFound,
    #[error("world storage is unavailable")]
    Unavailable,
}

fn storage_error(operation: &'static str, error: sqlx::Error) -> WorldError {
    let category = match error {
        sqlx::Error::PoolTimedOut | sqlx::Error::PoolClosed | sqlx::Error::WorkerCrashed => "pool",
        sqlx::Error::Io(_) | sqlx::Error::Tls(_) => "connection",
        sqlx::Error::Database(_) => "database",
        _ => "other",
    };
    eprintln!(
        "{}",
        serde_json::json!({
            "owner": "world",
            "operation": operation,
            "status": "unavailable",
            "category": category,
            "recovery": "retry_later"
        })
    );
    WorldError::Unavailable
}
