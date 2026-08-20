use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MovementDirection {
    SourceToDestination,
    DestinationToSource,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MovementTarget {
    Complete,
    Partial {
        origin_segment_ordinal: u8,
        target_segment_ordinal: u8,
        x_cm: i64,
        y_cm: i64,
        z_cm: i64,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MoveCharacter {
    pub request_id: Uuid,
    pub connection_id: ConnectionId,
    pub expected_position_revision: PositionRevision,
    pub direction: MovementDirection,
    pub target: MovementTarget,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AcceptedMovement {
    pub activity: Activity,
    pub character: Character,
    pub connection: Connection,
}

impl MoveCharacter {
    fn normalize(self) -> Result<Self, WorldError> {
        if let MovementTarget::Partial {
            x_cm, y_cm, z_cm, ..
        } = self.target
            && [x_cm, y_cm, z_cm]
                .into_iter()
                .any(|coordinate| !(-MAX_COORDINATE_CM..=MAX_COORDINATE_CM).contains(&coordinate))
        {
            return Err(WorldError::InvalidMovement {
                field: MovementField::Target,
                reason: InvalidReason::OutOfRange,
            });
        }
        Ok(self)
    }
}

pub(super) fn movement_fingerprint(input: &MoveCharacter) -> Vec<u8> {
    let mut hash = Sha256::new();
    for field in [
        b"aicadia-move-character-fingerprint-v1".as_slice(),
        input.connection_id.0.as_bytes(),
        input.expected_position_revision.entity_id().0.as_bytes(),
        input.expected_position_revision.activity_id().0.as_bytes(),
    ] {
        fingerprint_field(&mut hash, field);
    }
    fingerprint_field(
        &mut hash,
        match input.direction {
            MovementDirection::SourceToDestination => b"source_to_destination",
            MovementDirection::DestinationToSource => b"destination_to_source",
        },
    );
    match input.target {
        MovementTarget::Complete => fingerprint_field(&mut hash, b"complete"),
        MovementTarget::Partial {
            origin_segment_ordinal,
            target_segment_ordinal,
            x_cm,
            y_cm,
            z_cm,
        } => {
            fingerprint_field(&mut hash, b"partial");
            fingerprint_field(&mut hash, &[origin_segment_ordinal]);
            fingerprint_field(&mut hash, &[target_segment_ordinal]);
            fingerprint_field(&mut hash, &x_cm.to_be_bytes());
            fingerprint_field(&mut hash, &y_cm.to_be_bytes());
            fingerprint_field(&mut hash, &z_cm.to_be_bytes());
        }
    }
    hash.finalize().to_vec()
}

impl World {
    pub async fn move_character(
        &self,
        user_id: UserId,
        input: MoveCharacter,
    ) -> Result<AcceptedMovement, WorldError> {
        let input = input.normalize()?;
        let fingerprint = movement_fingerprint(&input);
        let mut transaction = self.begin_spatial_mutation("move_character").await?;

        if let Some(existing) = find_movement_retry(
            &mut transaction,
            user_id,
            input.request_id,
            &fingerprint,
            "move_character",
        )
        .await?
        {
            return Ok(existing);
        }

        lock_user(&mut transaction, user_id, "move_character").await?;
        if let Some(existing) = find_movement_retry(
            &mut transaction,
            user_id,
            input.request_id,
            &fingerprint,
            "move_character",
        )
        .await?
        {
            return Ok(existing);
        }

        let mut character = find_character(&mut transaction, user_id, true, "move_character")
            .await?
            .ok_or(WorldError::CharacterNotFound)?;
        let old_position = character
            .position
            .clone()
            .ok_or(WorldError::CharacterNotEntered)?;
        if old_position.position_revision != input.expected_position_revision {
            return Err(WorldError::PositionRevisionConflict);
        }

        let connection =
            find_connection_by_id(&mut transaction, input.connection_id, "move_character")
                .await?
                .ok_or(WorldError::ConnectionUnavailable)?;
        if input.direction == MovementDirection::DestinationToSource && !connection.allows_reverse {
            return Err(WorldError::ConnectionDirectionDisallowed);
        }

        let target = movement_target(&character, &connection, input.direction, input.target)?;
        let old_place_id = character
            .current_place
            .as_ref()
            .map(|place| place.entity.id);
        let destination_place = target.place(&connection);
        let destination_place_id = destination_place.map(|place| place.id);
        let mut involved = Vec::with_capacity(2);
        if let Some(place_id) = old_place_id {
            involved.push((place_id, ActivityEntityRole::Location));
        }
        if let Some(place_id) = destination_place_id {
            involved.push((place_id, ActivityEntityRole::Destination));
        }
        let activity_id = append_activity(
            &mut transaction,
            ActivityDraft {
                operation: ActivityOperation::MoveCharacter,
                requested_by_user_id: user_id,
                actor_character_entity_id: Some(character.entity.id),
                context_place_entity_id: destination_place_id,
                involved: &involved,
                prose: None,
                request_id: Some(input.request_id),
                request_fingerprint: Some(&fingerprint),
                action_consequence: None,
            },
            "move_character",
        )
        .await?;
        append_activity_position(
            &mut transaction,
            activity_id,
            ActivityPositionRole::Origin,
            old_position.position_revision,
            "move_character",
        )
        .await?;
        let new_position = append_character_position(
            &mut transaction,
            character.entity.id,
            activity_id,
            &old_position,
            target.point(),
            "move_character",
        )
        .await?;
        let changed =
            sqlx::query("UPDATE character SET current_place_entity_id = $1 WHERE entity_id = $2")
                .bind(destination_place_id.map(|id| id.0))
                .bind(character.entity.id.0)
                .execute(&mut *transaction)
                .await
                .map_err(|error| storage_error("move_character", error))?;
        if changed.rows_affected() != 1 {
            return Err(invalid_stored_relation());
        }
        sqlx::query("INSERT INTO activity_connection (activity_id, connection_id) VALUES ($1, $2)")
            .bind(activity_id.0)
            .bind(connection.id.0)
            .execute(&mut *transaction)
            .await
            .map_err(|error| storage_error("move_character", error))?;

        character.position = Some(new_position);
        character.current_place = match destination_place_id {
            Some(place_id) => Some(
                find_place_by_id(&mut transaction, place_id, "move_character")
                    .await?
                    .ok_or_else(invalid_stored_relation)?,
            ),
            None => None,
        };
        let activity = find_request_activity(
            &mut transaction,
            user_id,
            input.request_id,
            "move_character",
        )
        .await?
        .ok_or_else(invalid_stored_relation)?
        .activity;
        let accepted = AcceptedMovement {
            activity,
            character,
            connection,
        };
        transaction
            .commit()
            .await
            .map_err(|error| storage_error("move_character", error))?;
        Ok(accepted)
    }
}

#[derive(Clone, Copy)]
enum AcceptedTarget {
    Complete { point: Point, place_id: EntityId },
    Partial { point: Point },
}

impl AcceptedTarget {
    fn point(self) -> Point {
        match self {
            Self::Complete { point, .. } | Self::Partial { point } => point,
        }
    }

    fn place(self, connection: &Connection) -> Option<&PlacePosition> {
        let place_id = match self {
            Self::Complete { place_id, .. } => place_id,
            Self::Partial { .. } => return None,
        };
        [&connection.source.place, &connection.destination.place]
            .into_iter()
            .find(|place| place.id == place_id)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn from_position(position: &Position) -> Self {
        Self {
            x: position.x_cm,
            y: position.y_cm,
            z: position.z_cm,
        }
    }

    fn from_course(point: &ConnectionPoint) -> Self {
        Self {
            x: point.x_cm,
            y: point.y_cm,
            z: point.z_cm,
        }
    }
}

fn movement_target(
    character: &Character,
    connection: &Connection,
    direction: MovementDirection,
    target: MovementTarget,
) -> Result<AcceptedTarget, WorldError> {
    let current = Point::from_position(
        character
            .position
            .as_ref()
            .ok_or(WorldError::CharacterNotEntered)?,
    );
    validate_current_place(character, connection, current)?;
    match target {
        MovementTarget::Complete => complete_target(
            connection,
            direction,
            current,
            character
                .current_place
                .as_ref()
                .map(|place| place.entity.id),
        ),
        MovementTarget::Partial {
            origin_segment_ordinal,
            target_segment_ordinal,
            x_cm,
            y_cm,
            z_cm,
        } => partial_target(
            connection,
            direction,
            current,
            usize::from(origin_segment_ordinal),
            usize::from(target_segment_ordinal),
            Point {
                x: x_cm,
                y: y_cm,
                z: z_cm,
            },
        ),
    }
}

fn validate_current_place(
    character: &Character,
    connection: &Connection,
    current: Point,
) -> Result<(), WorldError> {
    let source = Point::from_position(&connection.source.place.position);
    let destination = Point::from_position(&connection.destination.place.position);
    let actual_place = character
        .current_place
        .as_ref()
        .map(|place| place.entity.id);
    if (current == source && actual_place == Some(connection.source.place.id))
        || (current == destination && actual_place == Some(connection.destination.place.id))
    {
        return Ok(());
    }
    if actual_place.is_some() || current == source || current == destination {
        return Err(WorldError::ConnectionUnavailable);
    }
    if !point_on_course(connection, current)? {
        return Err(WorldError::ConnectionUnavailable);
    }
    Ok(())
}

fn complete_target(
    connection: &Connection,
    direction: MovementDirection,
    current: Point,
    current_place_id: Option<EntityId>,
) -> Result<AcceptedTarget, WorldError> {
    let (departure, arrival) = match direction {
        MovementDirection::SourceToDestination => {
            (&connection.source.place, &connection.destination.place)
        }
        MovementDirection::DestinationToSource => {
            (&connection.destination.place, &connection.source.place)
        }
    };
    let departure_point = Point::from_position(&departure.position);
    let arrival_point = Point::from_position(&arrival.position);
    if current_place_id == Some(arrival.id) {
        return Err(WorldError::MovementNoProgress);
    }
    if connection.course.is_empty() && current != departure_point {
        return Err(WorldError::ConnectionUnavailable);
    }
    Ok(AcceptedTarget::Complete {
        point: arrival_point,
        place_id: arrival.id,
    })
}

fn partial_target(
    connection: &Connection,
    direction: MovementDirection,
    current: Point,
    origin_segment_ordinal: usize,
    target_segment_ordinal: usize,
    target: Point,
) -> Result<AcceptedTarget, WorldError> {
    if connection.course.is_empty() {
        return Err(WorldError::InvalidMovement {
            field: MovementField::Target,
            reason: InvalidReason::InvalidFormat,
        });
    }
    let segment_count = connection.course.len() - 1;
    if origin_segment_ordinal >= segment_count {
        return Err(WorldError::InvalidMovement {
            field: MovementField::OriginSegmentOrdinal,
            reason: InvalidReason::OutOfRange,
        });
    }
    if target_segment_ordinal >= segment_count {
        return Err(WorldError::InvalidMovement {
            field: MovementField::TargetSegmentOrdinal,
            reason: InvalidReason::OutOfRange,
        });
    }
    let origin_segment =
        segment_points(&connection.course[origin_segment_ordinal..=origin_segment_ordinal + 1]);
    let target_segment =
        segment_points(&connection.course[target_segment_ordinal..=target_segment_ordinal + 1]);
    if !point_on_segment(current, origin_segment)? || !point_on_segment(target, target_segment)? {
        return Err(WorldError::MovementOffCourse);
    }
    let source = Point::from_position(&connection.source.place.position);
    let destination = Point::from_position(&connection.destination.place.position);
    if target == source || target == destination {
        return Err(WorldError::InvalidMovement {
            field: MovementField::Target,
            reason: InvalidReason::InvalidFormat,
        });
    }
    if target == current {
        return Err(WorldError::MovementNoProgress);
    }
    let progresses = match direction {
        MovementDirection::SourceToDestination => {
            target_segment_ordinal > origin_segment_ordinal
                || (target_segment_ordinal == origin_segment_ordinal
                    && segment_parameter(target, target_segment)?
                        > segment_parameter(current, origin_segment)?)
        }
        MovementDirection::DestinationToSource => {
            target_segment_ordinal < origin_segment_ordinal
                || (target_segment_ordinal == origin_segment_ordinal
                    && segment_parameter(target, target_segment)?
                        < segment_parameter(current, origin_segment)?)
        }
    };
    if !progresses {
        return Err(WorldError::MovementNoProgress);
    }
    Ok(AcceptedTarget::Partial { point: target })
}

fn segment_points(segment: &[ConnectionPoint]) -> (Point, Point) {
    (
        Point::from_course(&segment[0]),
        Point::from_course(&segment[1]),
    )
}

fn checked_sub(left: i128, right: i128) -> Result<i128, WorldError> {
    left.checked_sub(right).ok_or(WorldError::InvalidMovement {
        field: MovementField::Target,
        reason: InvalidReason::OutOfRange,
    })
}

fn checked_mul(left: i128, right: i128) -> Result<i128, WorldError> {
    left.checked_mul(right).ok_or(WorldError::InvalidMovement {
        field: MovementField::Target,
        reason: InvalidReason::OutOfRange,
    })
}

fn checked_add(left: i128, right: i128) -> Result<i128, WorldError> {
    left.checked_add(right).ok_or(WorldError::InvalidMovement {
        field: MovementField::Target,
        reason: InvalidReason::OutOfRange,
    })
}

fn vector(from: Point, to: Point) -> Result<[i128; 3], WorldError> {
    Ok([
        checked_sub(i128::from(to.x), i128::from(from.x))?,
        checked_sub(i128::from(to.y), i128::from(from.y))?,
        checked_sub(i128::from(to.z), i128::from(from.z))?,
    ])
}

fn dot(left: [i128; 3], right: [i128; 3]) -> Result<i128, WorldError> {
    checked_add(
        checked_add(
            checked_mul(left[0], right[0])?,
            checked_mul(left[1], right[1])?,
        )?,
        checked_mul(left[2], right[2])?,
    )
}

fn point_on_segment(point: Point, (start, end): (Point, Point)) -> Result<bool, WorldError> {
    let segment = vector(start, end)?;
    let offset = vector(start, point)?;
    let cross = [
        checked_sub(
            checked_mul(segment[1], offset[2])?,
            checked_mul(segment[2], offset[1])?,
        )?,
        checked_sub(
            checked_mul(segment[2], offset[0])?,
            checked_mul(segment[0], offset[2])?,
        )?,
        checked_sub(
            checked_mul(segment[0], offset[1])?,
            checked_mul(segment[1], offset[0])?,
        )?,
    ];
    if cross != [0, 0, 0] {
        return Ok(false);
    }
    let length_squared = dot(segment, segment)?;
    let parameter = dot(offset, segment)?;
    Ok(parameter >= 0 && parameter <= length_squared)
}

fn segment_parameter(point: Point, (start, end): (Point, Point)) -> Result<i128, WorldError> {
    dot(vector(start, point)?, vector(start, end)?)
}

async fn append_character_position(
    transaction: &mut Transaction<'_, Postgres>,
    character_id: EntityId,
    activity_id: ActivityId,
    previous: &Position,
    point: Point,
    operation: &'static str,
) -> Result<Position, WorldError> {
    sqlx::query(
        r#"
        INSERT INTO position_version (
            entity_id, activity_id, previous_activity_id,
            x_cm, y_cm, z_cm, description
        ) VALUES ($1, $2, $3, $4, $5, $6, NULL)
        "#,
    )
    .bind(character_id.0)
    .bind(activity_id.0)
    .bind(previous.position_revision.activity_id().0)
    .bind(point.x)
    .bind(point.y)
    .bind(point.z)
    .execute(&mut **transaction)
    .await
    .map_err(|error| storage_error(operation, error))?;
    let advanced = sqlx::query(
        "UPDATE position SET current_activity_id = $1 WHERE entity_id = $2 AND current_activity_id = $3",
    )
    .bind(activity_id.0)
    .bind(character_id.0)
    .bind(previous.position_revision.activity_id().0)
    .execute(&mut **transaction)
    .await
    .map_err(|error| storage_error(operation, error))?;
    if advanced.rows_affected() != 1 {
        return Err(WorldError::PositionRevisionConflict);
    }
    let revision = PositionRevision::from_parts(character_id, activity_id);
    append_activity_position(
        transaction,
        activity_id,
        ActivityPositionRole::Result,
        revision,
        operation,
    )
    .await?;
    Ok(Position {
        x_cm: point.x,
        y_cm: point.y,
        z_cm: point.z,
        description: None,
        position_revision: revision,
    })
}

async fn find_movement_retry(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: UserId,
    request_id: Uuid,
    fingerprint: &[u8],
    operation: &'static str,
) -> Result<Option<AcceptedMovement>, WorldError> {
    let Some(stored) = find_request_activity(transaction, user_id, request_id, operation).await?
    else {
        return Ok(None);
    };
    if stored.operation != ActivityOperation::MoveCharacter
        || stored.request_fingerprint != fingerprint
    {
        return Err(WorldError::MovementRequestConflict);
    }
    reconstruct_movement(transaction, user_id, stored.activity, operation)
        .await
        .map(Some)
}

async fn reconstruct_movement(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: UserId,
    activity: Activity,
    operation: &'static str,
) -> Result<AcceptedMovement, WorldError> {
    if activity.operation != ActivityOperation::MoveCharacter
        || activity.prose.is_some()
        || !activity.property_change.is_empty()
        || !activity.trait_change.is_empty()
    {
        return Err(invalid_stored_relation());
    }
    let actor_id = activity
        .actor_character
        .as_ref()
        .map(|actor| actor.id)
        .ok_or_else(invalid_stored_relation)?;
    let result = activity
        .involved_position
        .iter()
        .filter(|reference| reference.role == ActivityPositionRole::Result)
        .collect::<Vec<_>>();
    let origin = activity
        .involved_position
        .iter()
        .filter(|reference| reference.role == ActivityPositionRole::Origin)
        .collect::<Vec<_>>();
    if origin.len() != 1
        || result.len() != 1
        || origin[0].position.position_revision.entity_id() != actor_id
        || result[0].position.position_revision.entity_id() != actor_id
        || result[0].position.position_revision.activity_id() != activity.id
        || activity.involved_connection.len() != 1
    {
        return Err(invalid_stored_relation());
    }
    let destination = activity
        .involved_entity
        .iter()
        .filter(|reference| reference.role == ActivityEntityRole::Destination)
        .collect::<Vec<_>>();
    let location = activity
        .involved_entity
        .iter()
        .filter(|reference| reference.role == ActivityEntityRole::Location)
        .collect::<Vec<_>>();
    if destination.len() > 1
        || location.len() > 1
        || activity.involved_entity.len() != destination.len() + location.len()
    {
        return Err(invalid_stored_relation());
    }
    validate_movement_result_version(
        transaction,
        actor_id,
        activity.id,
        origin[0].position.position_revision.activity_id(),
        operation,
    )
    .await?;
    let (entity, position) =
        find_entity_with_position(transaction, actor_id, activity.id, operation)
            .await?
            .ok_or_else(invalid_stored_relation)?;
    let owner_user_id: Option<UserId> =
        sqlx::query_scalar("SELECT owner_user_id FROM character WHERE entity_id = $1")
            .bind(actor_id.0)
            .fetch_optional(&mut **transaction)
            .await
            .map_err(|error| storage_error(operation, error))?;
    if owner_user_id != Some(user_id) {
        return Err(invalid_stored_relation());
    }
    let owner_user_id = owner_user_id.ok_or_else(invalid_stored_relation)?;
    let current_place = match destination.first() {
        Some(reference) => Some(
            find_place_by_id(transaction, reference.entity.id, operation)
                .await?
                .ok_or_else(invalid_stored_relation)?,
        ),
        None => None,
    };
    if activity.context_place.as_ref().map(|place| place.entity.id)
        != current_place.as_ref().map(|place| place.entity.id)
    {
        return Err(invalid_stored_relation());
    }
    let connection = find_connection_by_id(
        transaction,
        activity.involved_connection[0].connection_id,
        operation,
    )
    .await?
    .ok_or_else(invalid_stored_relation)?;
    validate_reconstructed_footprint(
        &activity,
        &connection,
        &origin[0].position,
        &result[0].position,
        location.first().map(|reference| reference.entity.id),
        destination.first().map(|reference| reference.entity.id),
    )?;
    Ok(AcceptedMovement {
        activity,
        character: Character {
            entity,
            owner_user_id,
            position: Some(position),
            current_place,
        },
        connection,
    })
}

async fn validate_movement_result_version(
    transaction: &mut Transaction<'_, Postgres>,
    actor_id: EntityId,
    result_activity_id: ActivityId,
    expected_previous_activity_id: ActivityId,
    operation: &'static str,
) -> Result<(), WorldError> {
    let result: Option<(Option<ActivityId>, Option<String>)> = sqlx::query_as(
        r#"
        SELECT previous_activity_id, description
        FROM position_version
        WHERE entity_id = $1 AND activity_id = $2
        "#,
    )
    .bind(actor_id.0)
    .bind(result_activity_id.0)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|error| storage_error(operation, error))?;
    if result != Some((Some(expected_previous_activity_id), None)) {
        return Err(invalid_stored_relation());
    }
    Ok(())
}

fn validate_reconstructed_footprint(
    activity: &Activity,
    connection: &Connection,
    origin: &Position,
    result: &Position,
    location_id: Option<EntityId>,
    destination_id: Option<EntityId>,
) -> Result<(), WorldError> {
    let origin_point = Point::from_position(origin);
    let result_point = Point::from_position(result);
    if origin_point == result_point && location_id == destination_id {
        return Err(invalid_stored_relation());
    }
    let source_id = connection.source.place.id;
    let destination_endpoint_id = connection.destination.place.id;
    let source_point = Point::from_position(&connection.source.place.position);
    let destination_point = Point::from_position(&connection.destination.place.position);

    match location_id {
        Some(id) if id == source_id && origin_point == source_point => {}
        Some(id) if id == destination_endpoint_id && origin_point == destination_point => {}
        Some(_) => return Err(invalid_stored_relation()),
        None => {
            if origin_point == source_point
                || origin_point == destination_point
                || !point_on_course(connection, origin_point)?
            {
                return Err(invalid_stored_relation());
            }
        }
    }

    match destination_id {
        Some(id) => {
            let expected_result = if id == source_id {
                source_point
            } else if id == destination_endpoint_id {
                destination_point
            } else {
                return Err(invalid_stored_relation());
            };
            if result_point != expected_result
                || activity.context_place.as_ref().map(|place| place.entity.id) != Some(id)
                || location_id == Some(id)
            {
                return Err(invalid_stored_relation());
            }
        }
        None => {
            if activity.context_place.is_some()
                || result_point == source_point
                || result_point == destination_point
                || !point_on_course(connection, result_point)?
            {
                return Err(invalid_stored_relation());
            }
        }
    }
    Ok(())
}

fn point_on_course(connection: &Connection, point: Point) -> Result<bool, WorldError> {
    if connection.course.is_empty() {
        return Ok(false);
    }
    for segment in connection.course.windows(2) {
        if point_on_segment(point, segment_points(segment))? {
            return Ok(true);
        }
    }
    Ok(false)
}

#[cfg(test)]
mod test;
