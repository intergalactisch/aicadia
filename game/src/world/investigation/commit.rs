use super::super::*;
use super::attempt;
use super::model::*;

pub(super) async fn find_accepted(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: UserId,
    request_id: Uuid,
    request_fingerprint: &[u8],
    operation: &'static str,
) -> Result<Option<AcceptedDiscovery>, WorldError> {
    let Some(existing) = find_request_activity(transaction, user_id, request_id, operation).await?
    else {
        return Ok(None);
    };
    if existing.operation != ActivityOperation::SubmitDiscovery
        || existing.request_fingerprint != request_fingerprint
    {
        return Err(WorldError::DiscoveryRequestConflict);
    }
    reconstruct(transaction, user_id, existing.activity, operation)
        .await
        .map(Some)
}

async fn reconstruct(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: UserId,
    activity: Activity,
    operation: &'static str,
) -> Result<AcceptedDiscovery, WorldError> {
    if activity.operation != ActivityOperation::SubmitDiscovery {
        return Err(invalid_stored_relation());
    }
    if activity.involved_connection.is_empty() {
        if let Some(origin_position) =
            optional_one_position(&activity, ActivityPositionRole::Origin)?
        {
            let actor_id = activity
                .actor_character
                .as_ref()
                .map(|actor| actor.id)
                .ok_or_else(invalid_stored_relation)?;
            if origin_position.position_revision.entity_id() != actor_id {
                return Err(invalid_stored_relation());
            }
        }
        let subject_id = exactly_one_entity(&activity, ActivityEntityRole::Subject)?;
        let result_position = activity
            .involved_position
            .iter()
            .find(|reference| {
                reference.role == ActivityPositionRole::Result
                    && reference.position.position_revision.entity_id() == subject_id
            })
            .map(|reference| reference.position.clone())
            .ok_or_else(invalid_stored_relation)?;
        let (entity, position) = find_entity_with_position(
            transaction,
            subject_id,
            result_position.position_revision.activity_id(),
            operation,
        )
        .await?
        .ok_or_else(invalid_stored_relation)?;
        let location = optional_one_entity(&activity, ActivityEntityRole::Location)?;
        let place = match location {
            Some(place_id) => Some(
                find_place_by_id(transaction, place_id, operation)
                    .await?
                    .ok_or_else(invalid_stored_relation)?,
            ),
            None => None,
        };
        return Ok(AcceptedDiscovery::EntityAtPosition {
            activity,
            entity,
            position,
            place,
        });
    }

    if activity.involved_connection.len() != 1 {
        return Err(invalid_stored_relation());
    }
    let origin_position = exactly_one_position(&activity, ActivityPositionRole::Origin)?;
    let origin_id = exactly_one_entity(&activity, ActivityEntityRole::Location)?;
    let destination_id = exactly_one_entity(&activity, ActivityEntityRole::Destination)?;
    let origin = find_place_by_id(transaction, origin_id, operation)
        .await?
        .ok_or_else(invalid_stored_relation)?;
    let destination = find_place_by_id(transaction, destination_id, operation)
        .await?
        .ok_or_else(invalid_stored_relation)?;
    let connection = find_connection_by_id(
        transaction,
        activity.involved_connection[0].connection_id,
        operation,
    )
    .await?
    .ok_or_else(invalid_stored_relation)?;
    if connection.source.place.id != origin_id || connection.destination.place.id != destination_id
    {
        return Err(invalid_stored_relation());
    }
    let actor_id = activity
        .actor_character
        .as_ref()
        .map(|actor| actor.id)
        .ok_or_else(invalid_stored_relation)?;
    if origin_position.position_revision.entity_id() != actor_id {
        return Err(invalid_stored_relation());
    }
    let character_entity = find_entity(transaction, actor_id, operation)
        .await?
        .ok_or_else(invalid_stored_relation)?;
    let owner_user_id: UserId =
        sqlx::query_scalar("SELECT owner_user_id FROM character WHERE entity_id = $1")
            .bind(actor_id.0)
            .fetch_optional(&mut **transaction)
            .await
            .map_err(|error| storage_error(operation, error))?
            .ok_or_else(invalid_stored_relation)?;
    if owner_user_id != user_id {
        return Err(invalid_stored_relation());
    }
    Ok(AcceptedDiscovery::ConnectedPlace {
        activity,
        origin: origin.clone(),
        destination,
        connection,
        character: Character {
            entity: character_entity,
            owner_user_id,
            position: Some(origin_position),
            current_place: Some(origin),
        },
    })
}

fn exactly_one_entity(
    activity: &Activity,
    role: ActivityEntityRole,
) -> Result<EntityId, WorldError> {
    let mut id = activity
        .involved_entity
        .iter()
        .filter(|reference| reference.role == role)
        .map(|reference| reference.entity.id);
    let first = id.next().ok_or_else(invalid_stored_relation)?;
    if id.next().is_some() {
        return Err(invalid_stored_relation());
    }
    Ok(first)
}

fn optional_one_entity(
    activity: &Activity,
    role: ActivityEntityRole,
) -> Result<Option<EntityId>, WorldError> {
    let mut id = activity
        .involved_entity
        .iter()
        .filter(|reference| reference.role == role)
        .map(|reference| reference.entity.id);
    let first = id.next();
    if id.next().is_some() {
        return Err(invalid_stored_relation());
    }
    Ok(first)
}

fn exactly_one_position(
    activity: &Activity,
    role: ActivityPositionRole,
) -> Result<Position, WorldError> {
    let mut position = activity
        .involved_position
        .iter()
        .filter(|reference| reference.role == role)
        .map(|reference| reference.position.clone());
    let first = position.next().ok_or_else(invalid_stored_relation)?;
    if position.next().is_some() {
        return Err(invalid_stored_relation());
    }
    Ok(first)
}

fn optional_one_position(
    activity: &Activity,
    role: ActivityPositionRole,
) -> Result<Option<Position>, WorldError> {
    let mut position = activity
        .involved_position
        .iter()
        .filter(|reference| reference.role == role)
        .map(|reference| reference.position.clone());
    let first = position.next();
    if position.next().is_some() {
        return Err(invalid_stored_relation());
    }
    Ok(first)
}

async fn find_entity(
    transaction: &mut Transaction<'_, Postgres>,
    entity_id: EntityId,
    operation: &'static str,
) -> Result<Option<Entity>, WorldError> {
    sqlx::query_as::<_, Entity>(
        "SELECT id, name, description, introduced_by_user_id, introduced_at FROM entity WHERE id = $1",
    )
    .bind(entity_id.0)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|error| storage_error(operation, error))
}

pub(super) async fn accept(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: UserId,
    character: Character,
    input: NormalizedSubmitDiscovery,
    request_fingerprint: &[u8],
    operation: &'static str,
) -> Result<AcceptedDiscovery, WorldError> {
    if !attempt::lock_available(
        transaction,
        input.attempt_id,
        user_id,
        &character,
        input.result.kind(),
        operation,
    )
    .await?
    {
        return Err(WorldError::DiscoveryAttemptUnavailable);
    }

    match input.result {
        DiscoveryResultInput::EntityAtPosition {
            name,
            description,
            position_description,
            property,
            r#trait,
        } => {
            accept_entity_at_position(
                transaction,
                user_id,
                character,
                input.request_id,
                input.attempt_id,
                input.prose,
                name,
                description,
                position_description,
                property,
                r#trait,
                request_fingerprint,
                operation,
            )
            .await
        }
        DiscoveryResultInput::ConnectedPlace {
            origin,
            destination,
            connection,
        } => {
            accept_connected_place(
                transaction,
                user_id,
                character,
                input.request_id,
                input.attempt_id,
                input.prose,
                origin,
                destination,
                connection,
                request_fingerprint,
                operation,
            )
            .await
        }
    }
}

#[allow(clippy::too_many_arguments)]
async fn accept_entity_at_position(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: UserId,
    character: Character,
    request_id: Uuid,
    attempt_id: InvestigationAttemptId,
    prose: String,
    name: String,
    description: String,
    position_description: Option<String>,
    property: Vec<PropertyInput>,
    r#trait: Vec<TraitInput>,
    request_fingerprint: &[u8],
    operation: &'static str,
) -> Result<AcceptedDiscovery, WorldError> {
    let place = character.current_place.clone();
    if let Some(place) = &place {
        lock_place(transaction, place.entity.id, operation).await?;
    }
    let entity = insert_entity(transaction, user_id, name, description)
        .await
        .map_err(|error| storage_error(operation, error))?;
    let mut involved = vec![(entity.id, ActivityEntityRole::Subject)];
    if let Some(place) = &place {
        involved.push((place.entity.id, ActivityEntityRole::Location));
    }
    let activity_id = append_activity(
        transaction,
        ActivityDraft {
            operation: ActivityOperation::SubmitDiscovery,
            requested_by_user_id: user_id,
            actor_character_entity_id: Some(character.entity.id),
            context_place_entity_id: place.as_ref().map(|place| place.entity.id),
            involved: &involved,
            prose: Some(&prose),
            request_id: Some(request_id),
            request_fingerprint: Some(request_fingerprint),
            action_consequence: None,
        },
        operation,
    )
    .await?;
    let actor_position = character
        .position
        .as_ref()
        .ok_or_else(invalid_stored_relation)?;
    append_activity_position(
        transaction,
        activity_id,
        ActivityPositionRole::Origin,
        actor_position.position_revision,
        operation,
    )
    .await?;
    insert_root_position(
        transaction,
        entity.id,
        activity_id,
        actor_position.x_cm,
        actor_position.y_cm,
        actor_position.z_cm,
        position_description.as_deref(),
        operation,
    )
    .await?;
    if let Some(place) = &place {
        sqlx::query("INSERT INTO entity_location (entity_id, place_entity_id) VALUES ($1, $2)")
            .bind(entity.id.0)
            .bind(place.entity.id.0)
            .execute(&mut **transaction)
            .await
            .map_err(|error| storage_error(operation, error))?;
    }
    write_initial_state(
        transaction,
        activity_id,
        entity.id,
        property,
        r#trait,
        operation,
    )
    .await?;
    attempt::consume(transaction, attempt_id, activity_id, operation).await?;
    if let Some(place) = &place {
        advance_place_revision(transaction, place.entity.id, activity_id, operation).await?;
    }
    let activity = find_request_activity(transaction, user_id, request_id, operation)
        .await?
        .ok_or_else(invalid_stored_relation)?
        .activity;
    reconstruct(transaction, user_id, activity, operation).await
}

#[allow(clippy::too_many_arguments)]
async fn accept_connected_place(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: UserId,
    character: Character,
    request_id: Uuid,
    attempt_id: InvestigationAttemptId,
    prose: String,
    origin_input: DiscoveryOriginInput,
    destination_input: DiscoveryDestinationInput,
    connection_input: ConnectionInput,
    request_fingerprint: &[u8],
    operation: &'static str,
) -> Result<AcceptedDiscovery, WorldError> {
    let loose = character.current_place.is_none();
    let (origin_entity, new_origin) = match (character.current_place.clone(), origin_input) {
        (Some(place), DiscoveryOriginInput::AttemptPlace) => (place.entity, None),
        (Some(_), _) | (None, DiscoveryOriginInput::AttemptPlace) => {
            return Err(WorldError::PlaceUnavailable);
        }
        (
            None,
            DiscoveryOriginInput::New {
                entity,
                position_description,
            },
        ) => {
            let inserted = insert_entity(
                transaction,
                user_id,
                entity.name.clone(),
                entity.description.clone(),
            )
            .await
            .map_err(|error| storage_error(operation, error))?;
            (inserted, Some((entity, position_description)))
        }
        (None, DiscoveryOriginInput::Existing { place_id }) => {
            let place = find_place_by_id(transaction, place_id, operation)
                .await?
                .ok_or(WorldError::PlaceUnavailable)?;
            let actor = character
                .position
                .as_ref()
                .ok_or_else(invalid_stored_relation)?;
            if (
                place.position.x_cm,
                place.position.y_cm,
                place.position.z_cm,
            ) != (actor.x_cm, actor.y_cm, actor.z_cm)
            {
                return Err(WorldError::PlaceUnavailable);
            }
            (place.entity, None)
        }
    };

    let (destination_entity, new_destination) = match destination_input {
        DiscoveryDestinationInput::New { entity, position } => {
            let inserted = insert_entity(
                transaction,
                user_id,
                entity.name.clone(),
                entity.description.clone(),
            )
            .await
            .map_err(|error| storage_error(operation, error))?;
            (inserted, Some((entity, position)))
        }
        DiscoveryDestinationInput::Existing { place_id } => {
            let place = find_place_by_id(transaction, place_id, operation)
                .await?
                .ok_or(WorldError::PlaceUnavailable)?;
            (place.entity, None)
        }
    };
    if origin_entity.id == destination_entity.id {
        return Err(WorldError::PlaceUnavailable);
    }

    let mut involved = Vec::new();
    if new_origin.is_some() {
        involved.push((origin_entity.id, ActivityEntityRole::Subject));
    }
    if new_destination.is_some() {
        involved.push((destination_entity.id, ActivityEntityRole::Subject));
    }
    involved.push((origin_entity.id, ActivityEntityRole::Location));
    involved.push((destination_entity.id, ActivityEntityRole::Destination));
    let context_place_entity_id = new_origin.is_none().then_some(origin_entity.id);
    let activity_id = append_activity(
        transaction,
        ActivityDraft {
            operation: ActivityOperation::SubmitDiscovery,
            requested_by_user_id: user_id,
            actor_character_entity_id: Some(character.entity.id),
            context_place_entity_id,
            involved: &involved,
            prose: Some(&prose),
            request_id: Some(request_id),
            request_fingerprint: Some(request_fingerprint),
            action_consequence: None,
        },
        operation,
    )
    .await?;
    let actor_position = character
        .position
        .as_ref()
        .ok_or_else(invalid_stored_relation)?;
    append_activity_position(
        transaction,
        activity_id,
        ActivityPositionRole::Origin,
        actor_position.position_revision,
        operation,
    )
    .await?;

    let mut initial_state = Vec::new();
    let origin = match new_origin {
        Some((entity_input, position_description)) => {
            let place = create_place(
                transaction,
                origin_entity,
                activity_id,
                actor_position.x_cm,
                actor_position.y_cm,
                actor_position.z_cm,
                position_description.as_deref(),
                operation,
            )
            .await?;
            initial_state.push((place.entity.id, entity_input.property, entity_input.r#trait));
            place
        }
        None => find_place_by_id(transaction, origin_entity.id, operation)
            .await?
            .ok_or(WorldError::PlaceUnavailable)?,
    };
    let destination = match new_destination {
        Some((entity_input, position)) => {
            let place = create_place(
                transaction,
                destination_entity,
                activity_id,
                position.x_cm,
                position.y_cm,
                position.z_cm,
                position.description.as_deref(),
                operation,
            )
            .await?;
            initial_state.push((place.entity.id, entity_input.property, entity_input.r#trait));
            place
        }
        None => find_place_by_id(transaction, destination_entity.id, operation)
            .await?
            .ok_or(WorldError::PlaceUnavailable)?,
    };

    write_initial_state_packages(transaction, activity_id, initial_state, operation).await?;

    validate_connection_course(&origin.position, &destination.position, &connection_input)?;
    let connection_id = insert_connection(
        transaction,
        activity_id,
        &origin,
        &destination,
        &connection_input,
        operation,
    )
    .await?;
    if loose {
        let updated = sqlx::query(
            "UPDATE character SET current_place_entity_id = $1 WHERE entity_id = $2 AND current_place_entity_id IS NULL",
        )
        .bind(origin.entity.id.0)
        .bind(character.entity.id.0)
        .execute(&mut **transaction)
        .await
        .map_err(|error| storage_error(operation, error))?;
        if updated.rows_affected() != 1 {
            return Err(WorldError::DiscoveryAttemptUnavailable);
        }
    }
    attempt::consume(transaction, attempt_id, activity_id, operation).await?;
    let activity = find_request_activity(transaction, user_id, request_id, operation)
        .await?
        .ok_or_else(invalid_stored_relation)?
        .activity;
    if activity
        .involved_connection
        .first()
        .map(|item| item.connection_id)
        != Some(connection_id)
    {
        return Err(invalid_stored_relation());
    }
    reconstruct(transaction, user_id, activity, operation).await
}

#[allow(clippy::too_many_arguments)]
async fn create_place(
    transaction: &mut Transaction<'_, Postgres>,
    entity: Entity,
    activity_id: ActivityId,
    x_cm: i64,
    y_cm: i64,
    z_cm: i64,
    description: Option<&str>,
    operation: &'static str,
) -> Result<Place, WorldError> {
    let position = insert_root_position(
        transaction,
        entity.id,
        activity_id,
        x_cm,
        y_cm,
        z_cm,
        description,
        operation,
    )
    .await?;
    sqlx::query(
        "INSERT INTO place (entity_id, is_entry, latest_activity_id) VALUES ($1, false, $2)",
    )
    .bind(entity.id.0)
    .bind(activity_id.0)
    .execute(&mut **transaction)
    .await
    .map_err(|error| storage_error(operation, error))?;
    insert_place_map_projection(transaction, entity.id, &position, operation).await?;
    Ok(Place {
        entity,
        position,
        is_entry: false,
    })
}

async fn write_initial_state_packages(
    transaction: &mut Transaction<'_, Postgres>,
    activity_id: ActivityId,
    state: Vec<(EntityId, Vec<PropertyInput>, Vec<TraitInput>)>,
    operation: &'static str,
) -> Result<(), WorldError> {
    let mut entity_id = Vec::with_capacity(state.len());
    let mut property = Vec::new();
    let mut r#trait = Vec::new();
    for (current_entity_id, current_property, current_trait) in state {
        entity_id.push(current_entity_id);
        property.extend(property_writes_for_entity(
            current_entity_id,
            current_property,
        ));
        r#trait.extend(trait_writes_for_entity(current_entity_id, current_trait));
    }
    write_property_changes(transaction, activity_id, &property)
        .await
        .map_err(|error| map_property_error(error, operation))?;
    write_trait_changes(transaction, activity_id, &r#trait, &entity_id)
        .await
        .map_err(|error| map_trait_error(error, operation))?;
    Ok(())
}

async fn write_initial_state(
    transaction: &mut Transaction<'_, Postgres>,
    activity_id: ActivityId,
    entity_id: EntityId,
    property: Vec<PropertyInput>,
    r#trait: Vec<TraitInput>,
    operation: &'static str,
) -> Result<(), WorldError> {
    let property = property_writes_for_entity(entity_id, property);
    write_property_changes(transaction, activity_id, &property)
        .await
        .map_err(|error| map_property_error(error, operation))?;
    let trait_change = trait_writes_for_entity(entity_id, r#trait);
    write_trait_changes(transaction, activity_id, &trait_change, &[entity_id])
        .await
        .map_err(|error| map_trait_error(error, operation))?;
    Ok(())
}

async fn insert_connection(
    transaction: &mut Transaction<'_, Postgres>,
    activity_id: ActivityId,
    origin: &Place,
    destination: &Place,
    input: &ConnectionInput,
    operation: &'static str,
) -> Result<ConnectionId, WorldError> {
    let connection_id = ConnectionId(Uuid::new_v4());
    sqlx::query(
        r#"
        INSERT INTO connection (
            id, source_place_entity_id, destination_place_entity_id,
            source_position_activity_id, destination_position_activity_id,
            allows_reverse, has_course, name, description, shape_description,
            created_by_activity_id
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        "#,
    )
    .bind(connection_id.0)
    .bind(origin.entity.id.0)
    .bind(destination.entity.id.0)
    .bind(origin.position.position_revision.activity_id().0)
    .bind(destination.position.position_revision.activity_id().0)
    .bind(input.allows_reverse)
    .bind(!input.course.is_empty())
    .bind(&input.name)
    .bind(&input.description)
    .bind(input.shape_description.as_deref())
    .bind(activity_id.0)
    .execute(&mut **transaction)
    .await
    .map_err(|error| storage_error(operation, error))?;
    if !input.course.is_empty() {
        let ordinal = (0..input.course.len())
            .map(|value| i16::try_from(value).expect("course is bounded to 128 points"))
            .collect::<Vec<_>>();
        let x = input
            .course
            .iter()
            .map(|point| point.x_cm)
            .collect::<Vec<_>>();
        let y = input
            .course
            .iter()
            .map(|point| point.y_cm)
            .collect::<Vec<_>>();
        let z = input
            .course
            .iter()
            .map(|point| point.z_cm)
            .collect::<Vec<_>>();
        sqlx::query(
            r#"
            INSERT INTO connection_point (connection_id, ordinal, x_cm, y_cm, z_cm)
            SELECT $1, point.ordinal, point.x_cm, point.y_cm, point.z_cm
            FROM UNNEST($2::smallint[], $3::bigint[], $4::bigint[], $5::bigint[])
                 AS point(ordinal, x_cm, y_cm, z_cm)
            "#,
        )
        .bind(connection_id.0)
        .bind(&ordinal)
        .bind(&x)
        .bind(&y)
        .bind(&z)
        .execute(&mut **transaction)
        .await
        .map_err(|error| storage_error(operation, error))?;
    }
    sqlx::query("INSERT INTO activity_connection (activity_id, connection_id) VALUES ($1, $2)")
        .bind(activity_id.0)
        .bind(connection_id.0)
        .execute(&mut **transaction)
        .await
        .map_err(|error| storage_error(operation, error))?;
    Ok(connection_id)
}

fn validate_connection_course(
    origin: &Position,
    destination: &Position,
    input: &ConnectionInput,
) -> Result<(), WorldError> {
    if input.course.is_empty() {
        return Ok(());
    }
    let point = &input.course;
    if (point[0].x_cm, point[0].y_cm, point[0].z_cm) != (origin.x_cm, origin.y_cm, origin.z_cm)
        || (
            point[point.len() - 1].x_cm,
            point[point.len() - 1].y_cm,
            point[point.len() - 1].z_cm,
        ) != (destination.x_cm, destination.y_cm, destination.z_cm)
    {
        return Err(invalid_connection(
            ConnectionField::Course,
            InvalidReason::InvalidFormat,
        ));
    }
    if point.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err(invalid_connection(
            ConnectionField::Course,
            InvalidReason::Duplicate,
        ));
    }
    Ok(())
}
