use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::{DateTime, SecondsFormat, Utc};
use schemars::{JsonSchema, Schema, SchemaGenerator};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    AcceptedAction, AcceptedActionConsequence, AcceptedDiscovery, AcceptedInteraction,
    ActionConsequence, ActionField, Activity, ActivityCursor, ActivityEntityReference,
    ActivityEntityRole, ActivityId, ActivityOperation, ActivityPage, ActivityTraitChange,
    ChangeEntityState, Character, CharacterEntityStatePage, CreateCharacter, CreateEntity,
    CreateEntryPlace, CurrentPlaceActivityPage, CurrentPlaceEntity, CurrentPlaceEntityPage,
    CurrentPlaceEntityStatePage, DiscoveryField, DiscoveryKind, DiscoveryResultInput, Entity,
    EntityCurrentAssociation, EntityCurrentStateCursor, EntityCurrentStatePage, EntityCursor,
    EntityField, EntityId, EntityPropertyChange, EntityPropertyChangeInput as WorldPropertyChange,
    EntitySummary, EntityTrait, EntityTraitChangeInput as WorldTraitChange, EntityTraitId,
    GetEntityAtCurrentPlace, GetEntityCurrentState, InteractionField, IntroduceEntity,
    InvalidReason, InvestigationAttemptId, InvestigationLimit, InvestigationOutcome,
    InvestigationResult, ListActivity, ListActivityAtCurrentPlace, ListEntityAtCurrentPlace, Place,
    PlaceRevision, PlaceSummary, PlaceWindowField, PlaceWindowReason, Position, PositionField,
    PositionRevision, PropertyField, PropertyInput as WorldPropertyInput, PropertyValue,
    StartInvestigation, SubmitAction, SubmitDiscovery, SubmitInteraction,
    TraitInput as WorldTraitInput, User, UserId, WorldError, WorldView,
};

pub const USER_CONTEXT_HEADER: &str = "Aicadia-User-Id";

mod error;
mod input;
mod investigation;
mod output;
mod spatial;

pub use error::*;
pub use input::*;
pub use investigation::*;
pub use output::*;
pub use spatial::*;

#[cfg(test)]
mod test;
