mod attempt;
pub(super) mod chance;
mod commit;
mod model;

pub use model::*;

use super::*;
use chance::ChancePolicy;

impl World {
    pub async fn start_investigation(
        &self,
        user_id: UserId,
        input: StartInvestigation,
    ) -> Result<InvestigationResult, WorldError> {
        let mut transaction = self.begin_spatial_mutation("start_investigation").await?;
        lock_user(&mut transaction, user_id, "start_investigation").await?;
        if let Some(result) = attempt::find_result(
            &mut transaction,
            user_id,
            input.request_id,
            input.kind,
            "start_investigation",
        )
        .await?
        {
            return Ok(result);
        }
        let character = find_character(&mut transaction, user_id, false, "start_investigation")
            .await?
            .ok_or(WorldError::CharacterNotFound)?;
        character
            .position
            .as_ref()
            .ok_or(WorldError::CharacterNotEntered)?;
        let place = character.current_place.as_ref();
        let database_now =
            attempt::admission_time_for_user(&mut transaction, user_id, "start_investigation")
                .await?;
        let discovery_count = match place {
            Some(place) => {
                attempt::recent_discovery_count(
                    &mut transaction,
                    place.entity.id,
                    "start_investigation",
                )
                .await?
            }
            None => 0,
        };
        let draw = self.chance.draw().map_err(|()| {
            eprintln!(
                "{}",
                serde_json::json!({
                    "owner": "world",
                    "operation": "start_investigation",
                    "status": "unavailable",
                    "category": "entropy",
                    "recovery": "retry_later"
                })
            );
            WorldError::Unavailable
        })?;
        let outcome = ChancePolicy::resolve(discovery_count, draw);
        let result = attempt::insert_attempt(
            &mut transaction,
            user_id,
            input,
            &character,
            outcome,
            database_now,
        )
        .await?;
        transaction
            .commit()
            .await
            .map_err(|error| storage_error("start_investigation", error))?;
        Ok(result)
    }

    pub async fn submit_discovery(
        &self,
        user_id: UserId,
        input: SubmitDiscovery,
    ) -> Result<AcceptedDiscovery, WorldError> {
        let input = input.normalize()?;
        let request_fingerprint = discovery_fingerprint(&input);
        let mut transaction = self.begin_spatial_mutation("submit_discovery").await?;
        lock_user(&mut transaction, user_id, "submit_discovery").await?;
        if let Some(accepted) = commit::find_accepted(
            &mut transaction,
            user_id,
            input.request_id,
            &request_fingerprint,
            "submit_discovery",
        )
        .await?
        {
            return Ok(accepted);
        }
        let character = find_character(&mut transaction, user_id, true, "submit_discovery")
            .await?
            .ok_or(WorldError::CharacterNotFound)?;
        let accepted = commit::accept(
            &mut transaction,
            user_id,
            character,
            input,
            &request_fingerprint,
            "submit_discovery",
        )
        .await?;
        transaction.commit().await.map_err(|error| {
            if constraint(&error) == Some("connection_complete_check") {
                invalid_connection(ConnectionField::Course, InvalidReason::InvalidFormat)
            } else {
                storage_error("submit_discovery", error)
            }
        })?;
        Ok(accepted)
    }
}

#[cfg(test)]
mod test;
