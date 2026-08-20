use super::*;

pub(super) fn normalize_position_description(
    description: Option<String>,
) -> Result<Option<String>, WorldError> {
    description
        .map(|description| {
            let description = description.trim().to_owned();
            if description.is_empty() {
                return Err(WorldError::InvalidRequest);
            }
            if description.contains('\0')
                || description.chars().count() > MAX_POSITION_DESCRIPTION_LENGTH
            {
                return Err(WorldError::InvalidRequest);
            }
            Ok(description)
        })
        .transpose()
}

pub(super) fn validate_coordinate(value: i64) -> Result<(), WorldError> {
    if (-MAX_COORDINATE_CM..=MAX_COORDINATE_CM).contains(&value) {
        Ok(())
    } else {
        Err(WorldError::InvalidRequest)
    }
}

pub(super) async fn insert_root_position(
    transaction: &mut Transaction<'_, Postgres>,
    entity_id: EntityId,
    activity_id: ActivityId,
    x_cm: i64,
    y_cm: i64,
    z_cm: i64,
    description: Option<&str>,
    operation: &'static str,
) -> Result<Position, WorldError> {
    for coordinate in [x_cm, y_cm, z_cm] {
        validate_coordinate(coordinate)?;
    }
    sqlx::query(
        r#"
        INSERT INTO position_version (
            entity_id, activity_id, previous_activity_id,
            x_cm, y_cm, z_cm, description
        ) VALUES ($1, $2, NULL, $3, $4, $5, $6)
        "#,
    )
    .bind(entity_id.0)
    .bind(activity_id.0)
    .bind(x_cm)
    .bind(y_cm)
    .bind(z_cm)
    .bind(description)
    .execute(&mut **transaction)
    .await
    .map_err(|error| storage_error(operation, error))?;
    sqlx::query("INSERT INTO position (entity_id, current_activity_id) VALUES ($1, $2)")
        .bind(entity_id.0)
        .bind(activity_id.0)
        .execute(&mut **transaction)
        .await
        .map_err(|error| storage_error(operation, error))?;
    append_activity_position(
        transaction,
        activity_id,
        ActivityPositionRole::Result,
        PositionRevision::from_parts(entity_id, activity_id),
        operation,
    )
    .await?;
    Ok(Position {
        x_cm,
        y_cm,
        z_cm,
        description: description.map(ToOwned::to_owned),
        position_revision: PositionRevision::from_parts(entity_id, activity_id),
    })
}

pub(super) async fn append_activity_position(
    transaction: &mut Transaction<'_, Postgres>,
    activity_id: ActivityId,
    role: ActivityPositionRole,
    position_revision: PositionRevision,
    operation: &'static str,
) -> Result<(), WorldError> {
    sqlx::query(
        r#"
        INSERT INTO activity_position (
            activity_id, role, position_entity_id, position_activity_id
        ) VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(activity_id.0)
    .bind(role.as_str())
    .bind(position_revision.entity_id().0)
    .bind(position_revision.activity_id().0)
    .execute(&mut **transaction)
    .await
    .map_err(|error| storage_error(operation, error))?;
    Ok(())
}

pub(super) async fn insert_place_map_projection(
    transaction: &mut Transaction<'_, Postgres>,
    place_entity_id: EntityId,
    position: &Position,
    operation: &'static str,
) -> Result<(), WorldError> {
    sqlx::query(
        r#"
        INSERT INTO place_map_index (
            place_entity_id, position_activity_id, x_cm, y_cm, z_cm
        ) VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(place_entity_id.0)
    .bind(position.position_revision.activity_id().0)
    .bind(position.x_cm)
    .bind(position.y_cm)
    .bind(position.z_cm)
    .execute(&mut **transaction)
    .await
    .map_err(|error| storage_error(operation, error))?;
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActivityPositionRole {
    Origin,
    Result,
}

impl ActivityPositionRole {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::Origin => "origin",
            Self::Result => "result",
        }
    }

    pub(super) fn parse(value: &str) -> Result<Self, WorldError> {
        match value {
            "origin" => Ok(Self::Origin),
            "result" => Ok(Self::Result),
            _ => Err(invalid_stored_relation()),
        }
    }
}
