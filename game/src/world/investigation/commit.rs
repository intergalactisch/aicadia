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
    reconstruct(transaction, existing.activity, operation)
        .await
        .map(Some)
}

async fn reconstruct(
    transaction: &mut Transaction<'_, Postgres>,
    activity: Activity,
    operation: &'static str,
) -> Result<AcceptedDiscovery, WorldError> {
    let subject_id = activity
        .involved_entity
        .iter()
        .find(|reference| reference.role == ActivityEntityRole::Subject)
        .map(|reference| reference.entity.id)
        .ok_or_else(invalid_stored_relation)?;
    let location_id = activity
        .involved_entity
        .iter()
        .find(|reference| reference.role == ActivityEntityRole::Location)
        .map(|reference| reference.entity.id)
        .ok_or_else(invalid_stored_relation)?;
    if activity
        .involved_entity
        .iter()
        .filter(|reference| reference.role == ActivityEntityRole::Subject)
        .count()
        != 1
        || activity
            .involved_entity
            .iter()
            .filter(|reference| reference.role == ActivityEntityRole::Location)
            .count()
            != 1
    {
        return Err(invalid_stored_relation());
    }
    let entity = find_placed_entity(transaction, subject_id, location_id, operation)
        .await?
        .ok_or_else(invalid_stored_relation)?;
    let place = find_place_by_id(transaction, location_id, operation)
        .await?
        .ok_or_else(invalid_stored_relation)?;
    Ok(AcceptedDiscovery {
        activity,
        entity,
        place,
    })
}

pub(super) async fn accept(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: UserId,
    character: Character,
    place: Place,
    input: NormalizedSubmitDiscovery,
    request_fingerprint: &[u8],
    operation: &'static str,
) -> Result<AcceptedDiscovery, WorldError> {
    let entity = insert_entity(
        transaction,
        user_id,
        input.find.name,
        input.find.description,
    )
    .await
    .map_err(|error| storage_error(operation, error))?;
    let involved = [
        (entity.id, ActivityEntityRole::Subject),
        (place.entity.id, ActivityEntityRole::Location),
    ];
    let activity_id = append_activity(
        transaction,
        ActivityDraft {
            operation: ActivityOperation::SubmitDiscovery,
            requested_by_user_id: user_id,
            actor_character_entity_id: Some(character.entity.id),
            context_place_entity_id: Some(place.entity.id),
            involved: &involved,
            prose: Some(&input.prose),
            request_id: Some(input.request_id),
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
        input.find.position_description.as_deref(),
        operation,
    )
    .await?;
    sqlx::query("INSERT INTO entity_location (entity_id, place_entity_id) VALUES ($1, $2)")
        .bind(entity.id.0)
        .bind(place.entity.id.0)
        .execute(&mut **transaction)
        .await
        .map_err(|error| storage_error(operation, error))?;

    let property = property_writes_for_entity(entity.id, input.find.property);
    write_property_changes(transaction, activity_id, &property)
        .await
        .map_err(|error| map_property_error(error, operation))?;
    let trait_change = trait_writes_for_entity(entity.id, input.find.r#trait);
    write_trait_changes(transaction, activity_id, &trait_change, &[entity.id])
        .await
        .map_err(|error| map_trait_error(error, operation))?;
    attempt::consume(transaction, input.attempt_id, activity_id, operation).await?;
    advance_place_revision(transaction, place.entity.id, activity_id, operation).await?;

    let activity = find_request_activity(transaction, user_id, input.request_id, operation)
        .await?
        .ok_or_else(invalid_stored_relation)?
        .activity;
    reconstruct(transaction, activity, operation).await
}
