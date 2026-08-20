use super::*;
use base64::Engine;

const PLACE_CURSOR_VERSION: &str = "lp1";
const CONNECTION_CURSOR_VERSION: &str = "lc1";
const MAX_SPATIAL_CURSOR_ENCODED_LENGTH: usize = 512;

#[derive(Clone, Copy, Debug, PartialEq, Eq, FromRow)]
pub(super) struct PlaceMapCandidate {
    pub(super) place_entity_id: EntityId,
    pub(super) position_activity_id: ActivityId,
    pub(super) x_cm: i64,
    pub(super) y_cm: i64,
    pub(super) z_cm: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct PlaceCursor {
    min_x_cm: i64,
    max_x_cm: i64,
    min_y_cm: i64,
    max_y_cm: i64,
    min_z_cm: i64,
    max_z_cm: i64,
    last_x_cm: i64,
    last_y_cm: i64,
    last_z_cm: i64,
    last_place_id: EntityId,
}

impl PlaceCursor {
    pub(super) fn from_candidate(request: &ListPlace, candidate: PlaceMapCandidate) -> Self {
        Self {
            min_x_cm: request.min_x_cm,
            max_x_cm: request.max_x_cm,
            min_y_cm: request.min_y_cm,
            max_y_cm: request.max_y_cm,
            min_z_cm: request.min_z_cm,
            max_z_cm: request.max_z_cm,
            last_x_cm: candidate.x_cm,
            last_y_cm: candidate.y_cm,
            last_z_cm: candidate.z_cm,
            last_place_id: candidate.place_entity_id,
        }
    }

    pub(super) fn encode(self) -> String {
        let plain = format!(
            "{PLACE_CURSOR_VERSION}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
            self.min_x_cm,
            self.max_x_cm,
            self.min_y_cm,
            self.max_y_cm,
            self.min_z_cm,
            self.max_z_cm,
            self.last_x_cm,
            self.last_y_cm,
            self.last_z_cm,
            self.last_place_id.0,
        );
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(plain)
    }

    pub(super) fn decode(value: &str, request: &ListPlace) -> Result<Self, WorldError> {
        if value.len() > MAX_SPATIAL_CURSOR_ENCODED_LENGTH {
            return Err(WorldError::InvalidRequest);
        }
        let decoded = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(value)
            .map_err(|_| WorldError::InvalidRequest)?;
        let decoded = std::str::from_utf8(&decoded).map_err(|_| WorldError::InvalidRequest)?;
        let part: Vec<_> = decoded.split(':').collect();
        if part.len() != 11 || part[0] != PLACE_CURSOR_VERSION {
            return Err(WorldError::InvalidRequest);
        }
        let parse_i64 = |value: &str| value.parse().map_err(|_| WorldError::InvalidRequest);
        let cursor = Self {
            min_x_cm: parse_i64(part[1])?,
            max_x_cm: parse_i64(part[2])?,
            min_y_cm: parse_i64(part[3])?,
            max_y_cm: parse_i64(part[4])?,
            min_z_cm: parse_i64(part[5])?,
            max_z_cm: parse_i64(part[6])?,
            last_x_cm: parse_i64(part[7])?,
            last_y_cm: parse_i64(part[8])?,
            last_z_cm: parse_i64(part[9])?,
            last_place_id: EntityId(
                Uuid::parse_str(part[10]).map_err(|_| WorldError::InvalidRequest)?,
            ),
        };
        if (
            cursor.min_x_cm,
            cursor.max_x_cm,
            cursor.min_y_cm,
            cursor.max_y_cm,
            cursor.min_z_cm,
            cursor.max_z_cm,
        ) != (
            request.min_x_cm,
            request.max_x_cm,
            request.min_y_cm,
            request.max_y_cm,
            request.min_z_cm,
            request.max_z_cm,
        ) {
            return Err(WorldError::InvalidRequest);
        }
        Ok(cursor)
    }

    pub(super) fn last_tuple(self) -> (i64, i64, i64, Uuid) {
        (
            self.last_x_cm,
            self.last_y_cm,
            self.last_z_cm,
            self.last_place_id.0,
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct ConnectionCursor {
    place_id: EntityId,
    last_connection_id: ConnectionId,
}

impl ConnectionCursor {
    pub(super) fn new(place_id: EntityId, last_connection_id: ConnectionId) -> Self {
        Self {
            place_id,
            last_connection_id,
        }
    }

    pub(super) fn encode(self) -> String {
        let plain = format!(
            "{CONNECTION_CURSOR_VERSION}:{}:{}",
            self.place_id.0, self.last_connection_id.0
        );
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(plain)
    }

    pub(super) fn decode(value: &str, place_id: EntityId) -> Result<Self, WorldError> {
        if value.len() > MAX_SPATIAL_CURSOR_ENCODED_LENGTH {
            return Err(WorldError::InvalidRequest);
        }
        let decoded = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(value)
            .map_err(|_| WorldError::InvalidRequest)?;
        let decoded = std::str::from_utf8(&decoded).map_err(|_| WorldError::InvalidRequest)?;
        let part: Vec<_> = decoded.split(':').collect();
        if part.len() != 3 || part[0] != CONNECTION_CURSOR_VERSION {
            return Err(WorldError::InvalidRequest);
        }
        let cursor = Self {
            place_id: EntityId(Uuid::parse_str(part[1]).map_err(|_| WorldError::InvalidRequest)?),
            last_connection_id: ConnectionId(
                Uuid::parse_str(part[2]).map_err(|_| WorldError::InvalidRequest)?,
            ),
        };
        if cursor.place_id != place_id {
            return Err(WorldError::InvalidRequest);
        }
        Ok(cursor)
    }

    pub(super) fn last_connection_id(self) -> ConnectionId {
        self.last_connection_id
    }
}

#[cfg(test)]
mod cursor_test {
    use super::*;

    #[test]
    fn worst_case_valid_place_cursor_fits_the_encoded_input_bound() {
        let request = ListPlace {
            min_x_cm: -MAX_COORDINATE_CM,
            max_x_cm: -MAX_COORDINATE_CM + MAX_PLACE_WINDOW_SPAN_CM,
            min_y_cm: -MAX_COORDINATE_CM,
            max_y_cm: -MAX_COORDINATE_CM + MAX_PLACE_WINDOW_SPAN_CM,
            min_z_cm: -MAX_COORDINATE_CM,
            max_z_cm: -MAX_COORDINATE_CM + MAX_PLACE_WINDOW_SPAN_CM,
            cursor: None,
            limit: 100,
        };
        let cursor = PlaceCursor::from_candidate(
            &request,
            PlaceMapCandidate {
                place_entity_id: EntityId(Uuid::from_u128(u128::MAX)),
                position_activity_id: ActivityId(Uuid::from_u128(u128::MAX)),
                x_cm: -MAX_COORDINATE_CM,
                y_cm: -MAX_COORDINATE_CM,
                z_cm: -MAX_COORDINATE_CM,
            },
        )
        .encode();

        assert!(cursor.len() <= MAX_SPATIAL_CURSOR_ENCODED_LENGTH);
        assert!(PlaceCursor::decode(&cursor, &request).is_ok());
    }
}

#[derive(Debug, FromRow)]
pub(super) struct PlacePositionRow {
    pub(super) id: EntityId,
    pub(super) name: String,
    pub(super) description: String,
    pub(super) is_entry: bool,
    pub(super) position_activity_id: ActivityId,
    pub(super) x_cm: i64,
    pub(super) y_cm: i64,
    pub(super) z_cm: i64,
    pub(super) position_description: Option<String>,
}

impl From<PlacePositionRow> for PlacePosition {
    fn from(row: PlacePositionRow) -> Self {
        Self {
            id: row.id,
            name: row.name,
            description: row.description,
            is_entry: row.is_entry,
            position: Position {
                x_cm: row.x_cm,
                y_cm: row.y_cm,
                z_cm: row.z_cm,
                description: row.position_description,
                position_revision: PositionRevision::from_parts(row.id, row.position_activity_id),
            },
        }
    }
}

#[derive(Debug, FromRow)]
pub(super) struct ConnectionRow {
    pub(super) id: ConnectionId,
    pub(super) allows_reverse: bool,
    pub(super) has_course: bool,
    pub(super) name: String,
    pub(super) description: String,
    pub(super) shape_description: Option<String>,
    pub(super) source_id: EntityId,
    pub(super) source_name: String,
    pub(super) source_description: String,
    pub(super) source_is_entry: bool,
    pub(super) source_position_activity_id: ActivityId,
    pub(super) source_x_cm: i64,
    pub(super) source_y_cm: i64,
    pub(super) source_z_cm: i64,
    pub(super) source_position_description: Option<String>,
    pub(super) destination_id: EntityId,
    pub(super) destination_name: String,
    pub(super) destination_description: String,
    pub(super) destination_is_entry: bool,
    pub(super) destination_position_activity_id: ActivityId,
    pub(super) destination_x_cm: i64,
    pub(super) destination_y_cm: i64,
    pub(super) destination_z_cm: i64,
    pub(super) destination_position_description: Option<String>,
}

impl ConnectionRow {
    fn endpoint(
        id: EntityId,
        name: String,
        description: String,
        is_entry: bool,
        activity_id: ActivityId,
        x_cm: i64,
        y_cm: i64,
        z_cm: i64,
        position_description: Option<String>,
    ) -> ConnectionEndpoint {
        ConnectionEndpoint {
            place: PlacePosition {
                id,
                name,
                description,
                is_entry,
                position: Position {
                    x_cm,
                    y_cm,
                    z_cm,
                    description: position_description,
                    position_revision: PositionRevision::from_parts(id, activity_id),
                },
            },
        }
    }

    pub(super) fn into_summary(self) -> ConnectionSummary {
        let source = Self::endpoint(
            self.source_id,
            self.source_name,
            self.source_description,
            self.source_is_entry,
            self.source_position_activity_id,
            self.source_x_cm,
            self.source_y_cm,
            self.source_z_cm,
            self.source_position_description,
        );
        let destination = Self::endpoint(
            self.destination_id,
            self.destination_name,
            self.destination_description,
            self.destination_is_entry,
            self.destination_position_activity_id,
            self.destination_x_cm,
            self.destination_y_cm,
            self.destination_z_cm,
            self.destination_position_description,
        );
        ConnectionSummary {
            id: self.id,
            source,
            destination,
            allows_reverse: self.allows_reverse,
            name: self.name,
            description: self.description,
            has_course: self.has_course,
        }
    }

    pub(super) fn into_connection(self, course: Vec<ConnectionPoint>) -> Connection {
        let shape_description = self.shape_description.clone();
        let summary = self.into_summary();
        Connection {
            id: summary.id,
            source: summary.source,
            destination: summary.destination,
            allows_reverse: summary.allows_reverse,
            name: summary.name,
            description: summary.description,
            shape_description,
            course,
        }
    }
}

#[derive(Debug, FromRow)]
pub(super) struct ConnectionPointRow {
    pub(super) ordinal: i16,
    pub(super) x_cm: i64,
    pub(super) y_cm: i64,
    pub(super) z_cm: i64,
}

impl TryFrom<ConnectionPointRow> for ConnectionPoint {
    type Error = WorldError;

    fn try_from(row: ConnectionPointRow) -> Result<Self, Self::Error> {
        Ok(Self {
            ordinal: u16::try_from(row.ordinal).map_err(|_| invalid_stored_relation())?,
            x_cm: row.x_cm,
            y_cm: row.y_cm,
            z_cm: row.z_cm,
        })
    }
}

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
