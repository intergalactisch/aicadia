use super::*;
use std::collections::HashSet;

impl World {
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

    pub async fn create_entity(
        &self,
        user_id: UserId,
        input: CreateEntity,
    ) -> Result<Entity, WorldError> {
        let input = input.normalize()?;
        let mut transaction = self.begin("create_entity").await?;
        lock_user(&mut transaction, user_id, "create_entity").await?;
        let context = find_character(&mut transaction, user_id, false, "create_entity").await?;
        if let Some(place) = context
            .as_ref()
            .and_then(|character| character.current_place.as_ref())
        {
            lock_place(&mut transaction, place.entity.id, "create_entity").await?;
        }
        let entity = insert_entity(&mut transaction, user_id, input.name, input.description)
            .await
            .map_err(|error| storage_error("create_entity", error))?;

        let activity_id = append_activity(
            &mut transaction,
            ActivityDraft {
                operation: ActivityOperation::CreateEntity,
                requested_by_user_id: user_id,
                actor_character_entity_id: context.as_ref().map(|character| character.entity.id),
                context_place_entity_id: context
                    .as_ref()
                    .and_then(|character| character.current_place.as_ref())
                    .map(|place| place.entity.id),
                involved: &[(entity.id, ActivityEntityRole::Subject)],
                prose: None,
                request_id: None,
                request_fingerprint: None,
                action_consequence: None,
            },
            "create_entity",
        )
        .await?;
        let property = property_writes_for_entity(entity.id, input.property);
        write_property_changes(&mut transaction, activity_id, &property)
            .await
            .map_err(|error| map_property_error(error, "create_entity"))?;
        let trait_change = trait_writes_for_entity(entity.id, input.r#trait);
        write_trait_changes(&mut transaction, activity_id, &trait_change, &[entity.id])
            .await
            .map_err(|error| map_trait_error(error, "create_entity"))?;
        if let Some(place) = context
            .as_ref()
            .and_then(|character| character.current_place.as_ref())
        {
            advance_place_revision(
                &mut transaction,
                place.entity.id,
                activity_id,
                "create_entity",
            )
            .await?;
        }
        transaction
            .commit()
            .await
            .map_err(|error| storage_error("create_entity", error))?;
        Ok(entity)
    }

    pub async fn create_character(
        &self,
        user_id: UserId,
        input: CreateCharacter,
    ) -> Result<Character, WorldError> {
        let input = input.normalize()?;
        let mut transaction = self.begin("create_character").await?;
        lock_user(&mut transaction, user_id, "create_character").await?;
        if find_character(&mut transaction, user_id, false, "create_character")
            .await?
            .is_some()
        {
            return Err(WorldError::CharacterAlreadyExists);
        }

        let entity = insert_entity(&mut transaction, user_id, input.name, input.description)
            .await
            .map_err(|error| storage_error("create_character", error))?;
        sqlx::query("INSERT INTO character (entity_id, owner_user_id) VALUES ($1, $2)")
            .bind(entity.id.0)
            .bind(user_id.0)
            .execute(&mut *transaction)
            .await
            .map_err(|error| storage_error("create_character", error))?;
        let activity_id = append_activity(
            &mut transaction,
            ActivityDraft {
                operation: ActivityOperation::CreateCharacter,
                requested_by_user_id: user_id,
                actor_character_entity_id: None,
                context_place_entity_id: None,
                involved: &[(entity.id, ActivityEntityRole::Subject)],
                prose: None,
                request_id: None,
                request_fingerprint: None,
                action_consequence: None,
            },
            "create_character",
        )
        .await?;
        let property = property_writes_for_entity(entity.id, input.property);
        write_property_changes(&mut transaction, activity_id, &property)
            .await
            .map_err(|error| map_property_error(error, "create_character"))?;
        let trait_change = trait_writes_for_entity(entity.id, input.r#trait);
        write_trait_changes(&mut transaction, activity_id, &trait_change, &[entity.id])
            .await
            .map_err(|error| map_trait_error(error, "create_character"))?;
        transaction
            .commit()
            .await
            .map_err(|error| storage_error("create_character", error))?;

        Ok(Character {
            entity,
            owner_user_id: user_id,
            position: None,
            current_place: None,
        })
    }

    pub async fn create_entry_place(
        &self,
        user_id: UserId,
        input: CreateEntryPlace,
    ) -> Result<Place, WorldError> {
        let input = input.normalize()?;
        let mut transaction = self.begin("create_entry_place").await?;
        lock_user(&mut transaction, user_id, "create_entry_place").await?;
        let character = find_character(&mut transaction, user_id, true, "create_entry_place")
            .await?
            .ok_or(WorldError::CharacterNotFound)?;
        if character.current_place.is_some() {
            return Err(WorldError::CharacterAlreadyEntered);
        }
        if find_entry_place(&mut transaction, "create_entry_place")
            .await?
            .is_some()
        {
            return Err(WorldError::EntryPlaceAlreadyExists);
        }

        let entity = insert_entity(&mut transaction, user_id, input.name, input.description)
            .await
            .map_err(|error| storage_error("create_entry_place", error))?;
        let activity_id = append_activity(
            &mut transaction,
            ActivityDraft {
                operation: ActivityOperation::CreateEntryPlace,
                requested_by_user_id: user_id,
                actor_character_entity_id: Some(character.entity.id),
                context_place_entity_id: None,
                involved: &[(entity.id, ActivityEntityRole::Subject)],
                prose: None,
                request_id: None,
                request_fingerprint: None,
                action_consequence: None,
            },
            "create_entry_place",
        )
        .await?;
        let property = property_writes_for_entity(entity.id, input.property);
        write_property_changes(&mut transaction, activity_id, &property)
            .await
            .map_err(|error| map_property_error(error, "create_entry_place"))?;
        let trait_change = trait_writes_for_entity(entity.id, input.r#trait);
        write_trait_changes(&mut transaction, activity_id, &trait_change, &[entity.id])
            .await
            .map_err(|error| map_trait_error(error, "create_entry_place"))?;
        let position = insert_root_position(
            &mut transaction,
            entity.id,
            activity_id,
            [0, 0, 0],
            None,
            "create_entry_place",
        )
        .await?;
        if let Err(error) = sqlx::query(
            "INSERT INTO place (entity_id, is_entry, latest_activity_id) VALUES ($1, true, $2)",
        )
        .bind(entity.id.0)
        .bind(activity_id.0)
        .execute(&mut *transaction)
        .await
        {
            if constraint(&error) == Some("place_one_entry_index") {
                return Err(WorldError::EntryPlaceAlreadyExists);
            }
            return Err(storage_error("create_entry_place", error));
        }
        insert_place_map_projection(&mut transaction, entity.id, &position, "create_entry_place")
            .await?;
        transaction
            .commit()
            .await
            .map_err(|error| storage_error("create_entry_place", error))?;
        Ok(Place {
            entity,
            position,
            is_entry: true,
        })
    }

    pub async fn enter_world(&self, user_id: UserId) -> Result<Character, WorldError> {
        let mut transaction = self.begin("enter_world").await?;
        lock_user(&mut transaction, user_id, "enter_world").await?;
        let mut character = find_character(&mut transaction, user_id, true, "enter_world")
            .await?
            .ok_or(WorldError::CharacterNotFound)?;
        if character.current_place.is_some() {
            return Ok(character);
        }
        let entry_place = find_entry_place(&mut transaction, "enter_world")
            .await?
            .ok_or(WorldError::EntryPlaceNotFound)?;
        lock_place(&mut transaction, entry_place.entity.id, "enter_world").await?;

        sqlx::query(
            "UPDATE character SET current_place_entity_id = $1 WHERE entity_id = $2 AND current_place_entity_id IS NULL",
        )
        .bind(entry_place.entity.id.0)
        .bind(character.entity.id.0)
        .execute(&mut *transaction)
        .await
        .map_err(|error| storage_error("enter_world", error))?;
        let activity_id = append_activity(
            &mut transaction,
            ActivityDraft {
                operation: ActivityOperation::EnterWorld,
                requested_by_user_id: user_id,
                actor_character_entity_id: Some(character.entity.id),
                context_place_entity_id: Some(entry_place.entity.id),
                involved: &[(entry_place.entity.id, ActivityEntityRole::Destination)],
                prose: None,
                request_id: None,
                request_fingerprint: None,
                action_consequence: None,
            },
            "enter_world",
        )
        .await?;
        let character_position = insert_root_position(
            &mut transaction,
            character.entity.id,
            activity_id,
            [
                entry_place.position.x_cm,
                entry_place.position.y_cm,
                entry_place.position.z_cm,
            ],
            None,
            "enter_world",
        )
        .await?;
        advance_place_revision(
            &mut transaction,
            entry_place.entity.id,
            activity_id,
            "enter_world",
        )
        .await?;
        transaction
            .commit()
            .await
            .map_err(|error| storage_error("enter_world", error))?;
        character.current_place = Some(entry_place);
        character.position = Some(character_position);
        Ok(character)
    }

    pub async fn submit_action(
        &self,
        user_id: UserId,
        input: SubmitAction,
    ) -> Result<AcceptedAction, WorldError> {
        let input = input.normalize()?;
        let request_fingerprint = action_fingerprint(&input);
        let mut transaction = self.begin("submit_action").await?;
        lock_user(&mut transaction, user_id, "submit_action").await?;

        if let Some(existing) =
            find_accepted_action(&mut transaction, user_id, input.request_id, "submit_action")
                .await?
        {
            if existing.request_fingerprint != request_fingerprint {
                return Err(WorldError::ActionRequestConflict);
            }
            return Ok(existing.accepted_action);
        }

        let character = find_character(&mut transaction, user_id, true, "submit_action")
            .await?
            .ok_or(WorldError::CharacterNotFound)?;
        let actor_position = character
            .position
            .clone()
            .ok_or(WorldError::CharacterNotEntered)?;
        let place = character
            .current_place
            .ok_or(WorldError::CharacterNotAtPlace)?;
        lock_place(&mut transaction, place.entity.id, "submit_action").await?;
        let current_revision =
            find_place_revision(&mut transaction, place.entity.id, "submit_action").await?;
        if input.expected_place_revision != current_revision {
            return Err(WorldError::PlaceRevisionConflict);
        }

        let (
            involved,
            property,
            trait_change,
            eligible_trait_entity,
            action_consequence,
            introduced_position,
        ) = match input.consequence {
            ActionConsequence::IntroduceEntity(consequence) => {
                let entity = insert_entity(
                    &mut transaction,
                    user_id,
                    consequence.name,
                    consequence.description,
                )
                .await
                .map_err(|error| storage_error("submit_action", error))?;
                (
                    vec![
                        (entity.id, ActivityEntityRole::Subject),
                        (place.entity.id, ActivityEntityRole::Location),
                    ],
                    property_writes_for_entity(entity.id, consequence.property),
                    trait_writes_for_entity(entity.id, consequence.r#trait),
                    vec![entity.id],
                    "introduce_entity",
                    Some((entity.id, consequence.position_description)),
                )
            }
            ActionConsequence::ChangeEntityState(consequence) => {
                let property = consequence
                    .property_change
                    .into_iter()
                    .map(|change| PropertyWrite {
                        entity_id: change.entity_id,
                        key: change.key,
                        value: change.value,
                    })
                    .collect::<Vec<_>>();
                require_local_property_entity(
                    &mut transaction,
                    character.entity.id,
                    place.entity.id,
                    &property,
                    "submit_action",
                )
                .await?;
                let trait_change = consequence
                    .trait_change
                    .into_iter()
                    .map(|change| match change {
                        EntityTraitChangeInput::Establish {
                            entity_id,
                            statement,
                        } => TraitWrite::Establish {
                            entity_id,
                            statement,
                        },
                        EntityTraitChangeInput::Develop {
                            trait_id,
                            statement,
                        } => TraitWrite::Develop {
                            trait_id: trait_id.0,
                            statement,
                        },
                    })
                    .collect::<Vec<_>>();
                let eligible_trait_entity = if trait_change.is_empty() {
                    Vec::new()
                } else {
                    find_local_entity_ids(
                        &mut transaction,
                        character.entity.id,
                        place.entity.id,
                        "submit_action",
                    )
                    .await?
                };
                let mut subject = property
                    .iter()
                    .map(|write| write.entity_id)
                    .collect::<Vec<_>>();
                subject.sort_unstable_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
                subject.dedup();
                let mut involved = subject
                    .into_iter()
                    .map(|entity_id| (entity_id, ActivityEntityRole::Subject))
                    .collect::<Vec<_>>();
                involved.push((place.entity.id, ActivityEntityRole::Location));
                (
                    involved,
                    property,
                    trait_change,
                    eligible_trait_entity,
                    "change_entity_state",
                    None,
                )
            }
        };
        let activity_id = append_activity(
            &mut transaction,
            ActivityDraft {
                operation: ActivityOperation::SubmitAction,
                requested_by_user_id: user_id,
                actor_character_entity_id: Some(character.entity.id),
                context_place_entity_id: Some(place.entity.id),
                involved: &involved,
                prose: Some(&input.prose),
                request_id: Some(input.request_id),
                request_fingerprint: Some(&request_fingerprint),
                action_consequence: Some(action_consequence),
            },
            "submit_action",
        )
        .await?;
        if let Some((entity_id, position_description)) = introduced_position {
            append_activity_position(
                &mut transaction,
                activity_id,
                ActivityPositionRole::Origin,
                actor_position.position_revision,
                "submit_action",
            )
            .await?;
            insert_root_position(
                &mut transaction,
                entity_id,
                activity_id,
                [
                    actor_position.x_cm,
                    actor_position.y_cm,
                    actor_position.z_cm,
                ],
                position_description.as_deref(),
                "submit_action",
            )
            .await?;
            sqlx::query("INSERT INTO entity_location (entity_id, place_entity_id) VALUES ($1, $2)")
                .bind(entity_id.0)
                .bind(place.entity.id.0)
                .execute(&mut *transaction)
                .await
                .map_err(|error| storage_error("submit_action", error))?;
        }
        write_property_changes(&mut transaction, activity_id, &property)
            .await
            .map_err(|error| map_property_error(error, "submit_action"))?;
        let stored_trait_change = write_trait_changes(
            &mut transaction,
            activity_id,
            &trait_change,
            &eligible_trait_entity,
        )
        .await
        .map_err(|error| map_trait_error(error, "submit_action"))?;
        if !stored_trait_change.is_empty() {
            let existing_subject = involved
                .iter()
                .filter(|(_, role)| *role == ActivityEntityRole::Subject)
                .map(|(entity_id, _)| *entity_id)
                .collect::<HashSet<_>>();
            let mut subject = stored_trait_change
                .iter()
                .map(|change| change.entity_id)
                .filter(|entity_id| !existing_subject.contains(entity_id))
                .collect::<Vec<_>>();
            subject.sort_unstable_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
            subject.dedup();
            append_activity_entity_roles(
                &mut transaction,
                activity_id,
                &subject
                    .into_iter()
                    .map(|entity_id| (entity_id, ActivityEntityRole::Subject))
                    .collect::<Vec<_>>(),
                "submit_action",
            )
            .await?;
        }
        advance_place_revision(
            &mut transaction,
            place.entity.id,
            activity_id,
            "submit_action",
        )
        .await?;
        let accepted =
            find_accepted_action(&mut transaction, user_id, input.request_id, "submit_action")
                .await?
                .ok_or_else(invalid_stored_relation)?
                .accepted_action;
        transaction
            .commit()
            .await
            .map_err(|error| storage_error("submit_action", error))?;
        Ok(accepted)
    }

    pub async fn submit_interaction(
        &self,
        user_id: UserId,
        input: SubmitInteraction,
    ) -> Result<AcceptedInteraction, WorldError> {
        let input = input.normalize()?;
        let request_fingerprint = interaction_fingerprint(&input);
        let mut transaction = self.begin("submit_interaction").await?;
        lock_user(&mut transaction, user_id, "submit_interaction").await?;

        if let Some(existing) = find_request_activity(
            &mut transaction,
            user_id,
            input.request_id,
            "submit_interaction",
        )
        .await?
        {
            if existing.operation != ActivityOperation::SubmitInteraction
                || existing.request_fingerprint != request_fingerprint
            {
                return Err(WorldError::InteractionRequestConflict);
            }
            return existing
                .into_accepted_interaction(&mut transaction, "submit_interaction")
                .await;
        }

        if input.has_duplicate_target {
            return Err(WorldError::InteractionTargetUnavailable);
        }

        let character = find_character(&mut transaction, user_id, true, "submit_interaction")
            .await?
            .ok_or(WorldError::CharacterNotFound)?;
        character
            .position
            .as_ref()
            .ok_or(WorldError::CharacterNotEntered)?;
        let place = character
            .current_place
            .ok_or(WorldError::CharacterNotAtPlace)?;
        lock_place(&mut transaction, place.entity.id, "submit_interaction").await?;
        let current_revision =
            find_place_revision(&mut transaction, place.entity.id, "submit_interaction").await?;
        if input.expected_place_revision != current_revision {
            return Err(WorldError::PlaceRevisionConflict);
        }

        let target_uuid = input
            .target_entity_id
            .iter()
            .map(|entity_id| entity_id.0)
            .collect::<Vec<_>>();
        let eligible_count: i64 = sqlx::query_scalar(
            r#"
            SELECT count(*)
            FROM UNNEST($1::uuid[]) AS submitted(entity_id)
            JOIN (
                SELECT character.entity_id
                FROM character
                WHERE character.current_place_entity_id = $2
                  AND character.entity_id <> $3

                UNION

                SELECT entity_location.entity_id
                FROM entity_location
                WHERE entity_location.place_entity_id = $2
                  AND NOT EXISTS (
                      SELECT 1
                      FROM character
                      WHERE character.entity_id = entity_location.entity_id
                  )
                  AND NOT EXISTS (
                      SELECT 1
                      FROM place
                      WHERE place.entity_id = entity_location.entity_id
                  )

                UNION

                SELECT $2::uuid
            ) eligible ON eligible.entity_id = submitted.entity_id
            "#,
        )
        .bind(&target_uuid)
        .bind(place.entity.id.0)
        .bind(character.entity.id.0)
        .fetch_one(&mut *transaction)
        .await
        .map_err(|error| storage_error("submit_interaction", error))?;
        if eligible_count != i64::try_from(input.target_entity_id.len()).unwrap_or(i64::MAX) {
            return Err(WorldError::InteractionTargetUnavailable);
        }
        if input.property_change.iter().any(|write| {
            write.entity_id != character.entity.id
                && input
                    .target_entity_id
                    .binary_search_by(|target| {
                        target.0.as_bytes().cmp(write.entity_id.0.as_bytes())
                    })
                    .is_err()
        }) {
            return Err(WorldError::PropertyEntityUnavailable);
        }

        let mut eligible_trait_entity = input.target_entity_id.clone();
        eligible_trait_entity.push(character.entity.id);
        eligible_trait_entity
            .sort_unstable_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
        eligible_trait_entity.dedup();

        let mut involved = input
            .target_entity_id
            .iter()
            .copied()
            .map(|entity_id| (entity_id, ActivityEntityRole::Target))
            .collect::<Vec<_>>();
        involved.push((place.entity.id, ActivityEntityRole::Location));
        let activity_id = append_activity(
            &mut transaction,
            ActivityDraft {
                operation: ActivityOperation::SubmitInteraction,
                requested_by_user_id: user_id,
                actor_character_entity_id: Some(character.entity.id),
                context_place_entity_id: Some(place.entity.id),
                involved: &involved,
                prose: Some(&input.prose),
                request_id: Some(input.request_id),
                request_fingerprint: Some(&request_fingerprint),
                action_consequence: None,
            },
            "submit_interaction",
        )
        .await?;
        write_property_changes(&mut transaction, activity_id, &input.property_change)
            .await
            .map_err(|error| map_property_error(error, "submit_interaction"))?;
        write_trait_changes(
            &mut transaction,
            activity_id,
            &input.trait_change,
            &eligible_trait_entity,
        )
        .await
        .map_err(|error| map_trait_error(error, "submit_interaction"))?;
        advance_place_revision(
            &mut transaction,
            place.entity.id,
            activity_id,
            "submit_interaction",
        )
        .await?;
        let accepted = find_request_activity(
            &mut transaction,
            user_id,
            input.request_id,
            "submit_interaction",
        )
        .await?
        .ok_or_else(invalid_stored_relation)?
        .into_accepted_interaction(&mut transaction, "submit_interaction")
        .await?;
        transaction
            .commit()
            .await
            .map_err(|error| storage_error("submit_interaction", error))?;
        Ok(accepted)
    }
}
