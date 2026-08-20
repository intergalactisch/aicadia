use super::*;

pub(super) async fn lock_user(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: UserId,
    operation: &'static str,
) -> Result<(), WorldError> {
    let found: Option<Uuid> =
        sqlx::query_scalar("SELECT id FROM \"user\" WHERE id = $1 FOR UPDATE")
            .bind(user_id.0)
            .fetch_optional(&mut **transaction)
            .await
            .map_err(|error| storage_error(operation, error))?;
    found.map(|_| ()).ok_or(WorldError::UserNotFound)
}

pub(super) async fn require_user(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: UserId,
    operation: &'static str,
) -> Result<(), WorldError> {
    let found: Option<Uuid> = sqlx::query_scalar("SELECT id FROM \"user\" WHERE id = $1")
        .bind(user_id.0)
        .fetch_optional(&mut **transaction)
        .await
        .map_err(|error| storage_error(operation, error))?;
    found.map(|_| ()).ok_or(WorldError::UserNotFound)
}

pub(super) async fn lock_place(
    transaction: &mut Transaction<'_, Postgres>,
    place_entity_id: EntityId,
    operation: &'static str,
) -> Result<(), WorldError> {
    let found: Option<Uuid> =
        sqlx::query_scalar("SELECT entity_id FROM place WHERE entity_id = $1 FOR UPDATE")
            .bind(place_entity_id.0)
            .fetch_optional(&mut **transaction)
            .await
            .map_err(|error| storage_error(operation, error))?;
    found.map(|_| ()).ok_or_else(invalid_stored_relation)
}

pub(super) async fn require_local_property_entity(
    transaction: &mut Transaction<'_, Postgres>,
    actor_entity_id: EntityId,
    place_entity_id: EntityId,
    property: &[PropertyWrite],
    operation: &'static str,
) -> Result<(), WorldError> {
    let mut submitted = property
        .iter()
        .map(|write| write.entity_id.0)
        .collect::<Vec<_>>();
    submitted.sort_unstable_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
    submitted.dedup();
    let eligible_count: i64 = sqlx::query_scalar(
        r#"
        SELECT count(*)
        FROM UNNEST($1::uuid[]) AS submitted(entity_id)
        JOIN (
            SELECT character.entity_id
            FROM character
            WHERE character.current_place_entity_id = $2

            UNION

            SELECT entity_location.entity_id
            FROM entity_location
            WHERE entity_location.place_entity_id = $2
              AND NOT EXISTS (
                  SELECT 1 FROM character
                  WHERE character.entity_id = entity_location.entity_id
              )
              AND NOT EXISTS (
                  SELECT 1 FROM place
                  WHERE place.entity_id = entity_location.entity_id
              )

            UNION

            SELECT $2::uuid

            UNION

            SELECT $3::uuid
        ) eligible ON eligible.entity_id = submitted.entity_id
        "#,
    )
    .bind(&submitted)
    .bind(place_entity_id.0)
    .bind(actor_entity_id.0)
    .fetch_one(&mut **transaction)
    .await
    .map_err(|error| storage_error(operation, error))?;
    if eligible_count != i64::try_from(submitted.len()).unwrap_or(i64::MAX) {
        return Err(WorldError::PropertyEntityUnavailable);
    }
    Ok(())
}

pub(super) async fn find_local_entity_ids(
    transaction: &mut Transaction<'_, Postgres>,
    actor_entity_id: EntityId,
    place_entity_id: EntityId,
    operation: &'static str,
) -> Result<Vec<EntityId>, WorldError> {
    sqlx::query_scalar::<_, EntityId>(
        r#"
        SELECT character.entity_id
        FROM character
        WHERE character.current_place_entity_id = $1

        UNION

        SELECT entity_location.entity_id
        FROM entity_location
        WHERE entity_location.place_entity_id = $1
          AND NOT EXISTS (
              SELECT 1 FROM character
              WHERE character.entity_id = entity_location.entity_id
          )
          AND NOT EXISTS (
              SELECT 1 FROM place
              WHERE place.entity_id = entity_location.entity_id
          )

        UNION

        SELECT $1::uuid

        UNION

        SELECT $2::uuid

        ORDER BY 1
        "#,
    )
    .bind(place_entity_id.0)
    .bind(actor_entity_id.0)
    .fetch_all(&mut **transaction)
    .await
    .map_err(|error| storage_error(operation, error))
}

pub(super) async fn find_place_revision(
    transaction: &mut Transaction<'_, Postgres>,
    place_entity_id: EntityId,
    operation: &'static str,
) -> Result<PlaceRevision, WorldError> {
    let row: Option<(DateTime<Utc>, ActivityId)> = sqlx::query_as(
        r#"
        SELECT activity.occurred_at, activity.id
        FROM place
        JOIN activity ON activity.id = place.latest_activity_id
        WHERE place.entity_id = $1
        "#,
    )
    .bind(place_entity_id.0)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|error| storage_error(operation, error))?;
    let (occurred_at, activity_id) = row.ok_or_else(invalid_stored_relation)?;
    Ok(PlaceRevision::from_parts(
        place_entity_id,
        occurred_at,
        activity_id,
    ))
}

pub(super) async fn advance_place_revision(
    transaction: &mut Transaction<'_, Postgres>,
    place_entity_id: EntityId,
    activity_id: ActivityId,
    operation: &'static str,
) -> Result<(), WorldError> {
    let result = sqlx::query("UPDATE place SET latest_activity_id = $1 WHERE entity_id = $2")
        .bind(activity_id.0)
        .bind(place_entity_id.0)
        .execute(&mut **transaction)
        .await
        .map_err(|error| storage_error(operation, error))?;
    if result.rows_affected() != 1 {
        return Err(invalid_stored_relation());
    }
    Ok(())
}

pub(super) async fn hydrate_entity_current_state(
    transaction: &mut Transaction<'_, Postgres>,
    entity_id: EntityId,
    place_revision: Option<PlaceRevision>,
    cursor: Option<EntityCurrentStateCursor>,
    limit: u16,
    operation: &'static str,
) -> Result<EntityCurrentStatePage, WorldError> {
    if let Some(cursor) = cursor {
        if cursor.entity_id != entity_id {
            return Err(WorldError::InvalidRequest);
        }
        if cursor.place_revision != place_revision {
            return Err(WorldError::PlaceRevisionConflict);
        }
    }
    let cursor_type = cursor.map(|cursor| match cursor.key {
        EntityCurrentStateCursorKey::Property(_) => 0_i16,
        EntityCurrentStateCursorKey::Trait(_) => 1_i16,
    });
    let cursor_property_key_id = cursor.and_then(EntityCurrentStateCursor::property_key_id);
    let cursor_trait_id = cursor
        .and_then(EntityCurrentStateCursor::trait_id)
        .map(|id| id.0);
    let fetch_limit = i64::from(limit) + 1;
    record_property_query(PropertyQueryKind::CurrentRead);
    record_trait_query(TraitQueryKind::CurrentRead);
    let mut row = sqlx::query_as::<_, EntityCurrentAssociationRow>(CURRENT_ENTITY_STATE_SQL)
        .bind(entity_id.0)
        .bind(cursor_type)
        .bind(cursor_property_key_id)
        .bind(cursor_trait_id)
        .bind(fetch_limit)
        .fetch_all(&mut **transaction)
        .await
        .map_err(|error| storage_error(operation, error))?;
    let has_more = row.len() > usize::from(limit);
    if has_more {
        row.pop();
    }
    let association = row
        .into_iter()
        .map(EntityCurrentAssociationRow::into_association)
        .collect::<Result<Vec<_>, _>>()?;
    let next = has_more.then(|| {
        let key = association
            .last()
            .expect("a current-state continuation always follows one returned association")
            .0;
        EntityCurrentStateCursor {
            entity_id,
            place_revision,
            key,
        }
    });
    Ok(EntityCurrentStatePage {
        association: association
            .into_iter()
            .map(|(_, association)| association)
            .collect(),
        next,
    })
}

pub(super) async fn insert_entity(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: UserId,
    name: String,
    description: String,
) -> Result<Entity, sqlx::Error> {
    sqlx::query_as::<_, Entity>(
        r#"
        INSERT INTO entity (id, name, description, introduced_by_user_id)
        VALUES ($1, $2, $3, $4)
        RETURNING id, name, description, introduced_by_user_id, introduced_at
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(name)
    .bind(description)
    .bind(user_id.0)
    .fetch_one(&mut **transaction)
    .await
}

pub(super) async fn find_character(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: UserId,
    for_update: bool,
    operation: &'static str,
) -> Result<Option<Character>, WorldError> {
    sqlx::query_as::<_, CharacterRow>(&character_query(for_update))
        .bind(user_id.0)
        .fetch_optional(&mut **transaction)
        .await
        .map_err(|error| storage_error(operation, error))?
        .map(CharacterRow::try_into)
        .transpose()
}

fn character_query(for_update: bool) -> String {
    format!(
        r#"
        SELECT entity.id AS entity_id, entity.name, entity.description,
               entity.introduced_by_user_id, entity.introduced_at,
               character.owner_user_id,
               character_position.current_activity_id AS position_activity_id,
               character_position_version.x_cm AS position_x_cm,
               character_position_version.y_cm AS position_y_cm,
               character_position_version.z_cm AS position_z_cm,
               character_position_version.description AS position_description,
               place_entity.id AS place_entity_id, place_entity.name AS place_name,
               place_entity.description AS place_description,
               place_entity.introduced_by_user_id AS place_introduced_by_user_id,
               place_entity.introduced_at AS place_introduced_at,
               place.is_entry AS place_is_entry,
               place_position.current_activity_id AS place_position_activity_id,
               place_position_version.x_cm AS place_position_x_cm,
               place_position_version.y_cm AS place_position_y_cm,
               place_position_version.z_cm AS place_position_z_cm,
               place_position_version.description AS place_position_description
        FROM character
        JOIN entity ON entity.id = character.entity_id
        LEFT JOIN position character_position ON character_position.entity_id = character.entity_id
        LEFT JOIN position_version character_position_version
          ON character_position_version.entity_id = character_position.entity_id
         AND character_position_version.activity_id = character_position.current_activity_id
        LEFT JOIN place ON place.entity_id = character.current_place_entity_id
        LEFT JOIN entity place_entity ON place_entity.id = place.entity_id
        LEFT JOIN position place_position ON place_position.entity_id = place.entity_id
        LEFT JOIN position_version place_position_version
          ON place_position_version.entity_id = place_position.entity_id
         AND place_position_version.activity_id = place_position.current_activity_id
        WHERE character.owner_user_id = $1
        {}
        "#,
        if for_update {
            "FOR UPDATE OF character"
        } else {
            ""
        }
    )
}

pub(super) async fn find_entry_place(
    transaction: &mut Transaction<'_, Postgres>,
    operation: &'static str,
) -> Result<Option<Place>, WorldError> {
    sqlx::query_as::<_, PlaceRow>(
        r#"
        SELECT entity.id AS entity_id, entity.name, entity.description,
               entity.introduced_by_user_id, entity.introduced_at, place.is_entry,
               position.current_activity_id AS position_activity_id,
               version.x_cm AS position_x_cm, version.y_cm AS position_y_cm,
               version.z_cm AS position_z_cm, version.description AS position_description
        FROM place
        JOIN entity ON entity.id = place.entity_id
        JOIN position ON position.entity_id = place.entity_id
        JOIN position_version version
          ON version.entity_id = position.entity_id
         AND version.activity_id = position.current_activity_id
        WHERE place.is_entry
        "#,
    )
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|error| storage_error(operation, error))?
    .map(TryInto::try_into)
    .transpose()
}

pub(super) async fn find_place_by_id(
    transaction: &mut Transaction<'_, Postgres>,
    place_entity_id: EntityId,
    operation: &'static str,
) -> Result<Option<Place>, WorldError> {
    sqlx::query_as::<_, PlaceRow>(
        r#"
        SELECT entity.id AS entity_id, entity.name, entity.description,
               entity.introduced_by_user_id, entity.introduced_at, place.is_entry,
               position.current_activity_id AS position_activity_id,
               version.x_cm AS position_x_cm, version.y_cm AS position_y_cm,
               version.z_cm AS position_z_cm, version.description AS position_description
        FROM place
        JOIN entity ON entity.id = place.entity_id
        JOIN position ON position.entity_id = place.entity_id
        JOIN position_version version
          ON version.entity_id = position.entity_id
         AND version.activity_id = position.current_activity_id
        WHERE place.entity_id = $1
        "#,
    )
    .bind(place_entity_id.0)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|error| storage_error(operation, error))?
    .map(TryInto::try_into)
    .transpose()
}

pub(super) fn validate_limit(limit: u16, error: WorldError) -> Result<(), WorldError> {
    if limit == 0 || limit > MAX_PAGE_SIZE {
        Err(error)
    } else {
        Ok(())
    }
}
#[derive(FromRow)]
pub(super) struct CurrentPlaceEntityRow {
    pub(super) id: EntityId,
    pub(super) name: String,
    pub(super) description: String,
    pub(super) introduced_at: DateTime<Utc>,
    pub(super) position_activity_id: ActivityId,
    pub(super) position_x_cm: i64,
    pub(super) position_y_cm: i64,
    pub(super) position_z_cm: i64,
    pub(super) position_description: Option<String>,
}

#[derive(FromRow)]
struct EntityCurrentAssociationRow {
    association_type: i16,
    property_key_id: Option<i64>,
    trait_id: Option<Uuid>,
    key: Option<String>,
    value_type: Option<String>,
    text_value: Option<String>,
    integer_value: Option<i64>,
    statement: Option<String>,
}

impl EntityCurrentAssociationRow {
    fn into_association(
        self,
    ) -> Result<(EntityCurrentStateCursorKey, EntityCurrentAssociation), WorldError> {
        match (
            self.association_type,
            self.property_key_id,
            self.trait_id,
            self.key,
            self.value_type.as_deref(),
            self.text_value,
            self.integer_value,
            self.statement,
        ) {
            (0, Some(property_key_id), None, Some(key), Some("text"), Some(value), None, None) => {
                Ok((
                    EntityCurrentStateCursorKey::Property(property_key_id),
                    EntityCurrentAssociation::Property {
                        key,
                        value: PropertyValue::Text(value),
                    },
                ))
            }
            (
                0,
                Some(property_key_id),
                None,
                Some(key),
                Some("integer"),
                None,
                Some(value),
                None,
            ) => Ok((
                EntityCurrentStateCursorKey::Property(property_key_id),
                EntityCurrentAssociation::Property {
                    key,
                    value: PropertyValue::Integer(value),
                },
            )),
            (1, None, Some(trait_id), None, None, None, None, Some(statement)) => Ok((
                EntityCurrentStateCursorKey::Trait(EntityTraitId(trait_id)),
                EntityCurrentAssociation::Trait(EntityTrait {
                    id: EntityTraitId(trait_id),
                    statement,
                }),
            )),
            _ => Err(invalid_stored_relation()),
        }
    }
}

#[derive(FromRow)]
struct CharacterRow {
    entity_id: EntityId,
    name: String,
    description: String,
    introduced_by_user_id: UserId,
    introduced_at: DateTime<Utc>,
    owner_user_id: UserId,
    position_activity_id: Option<ActivityId>,
    position_x_cm: Option<i64>,
    position_y_cm: Option<i64>,
    position_z_cm: Option<i64>,
    position_description: Option<String>,
    place_entity_id: Option<EntityId>,
    place_name: Option<String>,
    place_description: Option<String>,
    place_introduced_by_user_id: Option<UserId>,
    place_introduced_at: Option<DateTime<Utc>>,
    place_is_entry: Option<bool>,
    place_position_activity_id: Option<ActivityId>,
    place_position_x_cm: Option<i64>,
    place_position_y_cm: Option<i64>,
    place_position_z_cm: Option<i64>,
    place_position_description: Option<String>,
}

impl TryFrom<CharacterRow> for Character {
    type Error = WorldError;

    fn try_from(value: CharacterRow) -> Result<Self, Self::Error> {
        let position = optional_position(
            value.entity_id,
            value.position_activity_id,
            value.position_x_cm,
            value.position_y_cm,
            value.position_z_cm,
            value.position_description,
        )?;
        let current_place = match value.place_entity_id {
            None => None,
            Some(id) => Some(Place {
                entity: Entity {
                    id,
                    name: value.place_name.ok_or_else(invalid_stored_relation)?,
                    description: value
                        .place_description
                        .ok_or_else(invalid_stored_relation)?,
                    introduced_by_user_id: value
                        .place_introduced_by_user_id
                        .ok_or_else(invalid_stored_relation)?,
                    introduced_at: value
                        .place_introduced_at
                        .ok_or_else(invalid_stored_relation)?,
                },
                position: required_position(
                    id,
                    value.place_position_activity_id,
                    value.place_position_x_cm,
                    value.place_position_y_cm,
                    value.place_position_z_cm,
                    value.place_position_description,
                )?,
                is_entry: value.place_is_entry.ok_or_else(invalid_stored_relation)?,
            }),
        };
        Ok(Self {
            entity: Entity {
                id: value.entity_id,
                name: value.name,
                description: value.description,
                introduced_by_user_id: value.introduced_by_user_id,
                introduced_at: value.introduced_at,
            },
            owner_user_id: value.owner_user_id,
            position,
            current_place,
        })
    }
}

#[derive(FromRow)]
struct PlaceRow {
    entity_id: EntityId,
    name: String,
    description: String,
    introduced_by_user_id: UserId,
    introduced_at: DateTime<Utc>,
    is_entry: bool,
    position_activity_id: ActivityId,
    position_x_cm: i64,
    position_y_cm: i64,
    position_z_cm: i64,
    position_description: Option<String>,
}

impl TryFrom<PlaceRow> for Place {
    type Error = WorldError;

    fn try_from(value: PlaceRow) -> Result<Self, Self::Error> {
        Ok(Self {
            entity: Entity {
                id: value.entity_id,
                name: value.name,
                description: value.description,
                introduced_by_user_id: value.introduced_by_user_id,
                introduced_at: value.introduced_at,
            },
            position: Position {
                x_cm: value.position_x_cm,
                y_cm: value.position_y_cm,
                z_cm: value.position_z_cm,
                description: value.position_description,
                position_revision: PositionRevision::from_parts(
                    value.entity_id,
                    value.position_activity_id,
                ),
            },
            is_entry: value.is_entry,
        })
    }
}

fn optional_position(
    entity_id: EntityId,
    activity_id: Option<ActivityId>,
    x_cm: Option<i64>,
    y_cm: Option<i64>,
    z_cm: Option<i64>,
    description: Option<String>,
) -> Result<Option<Position>, WorldError> {
    match (activity_id, x_cm, y_cm, z_cm) {
        (None, None, None, None) if description.is_none() => Ok(None),
        (Some(activity_id), Some(x_cm), Some(y_cm), Some(z_cm)) => Ok(Some(Position {
            x_cm,
            y_cm,
            z_cm,
            description,
            position_revision: PositionRevision::from_parts(entity_id, activity_id),
        })),
        _ => Err(invalid_stored_relation()),
    }
}

fn required_position(
    entity_id: EntityId,
    activity_id: Option<ActivityId>,
    x_cm: Option<i64>,
    y_cm: Option<i64>,
    z_cm: Option<i64>,
    description: Option<String>,
) -> Result<Position, WorldError> {
    optional_position(entity_id, activity_id, x_cm, y_cm, z_cm, description)?
        .ok_or_else(invalid_stored_relation)
}
