use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::{DateTime, SecondsFormat, Utc};
use schemars::{JsonSchema, Schema, SchemaGenerator};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    AcceptedAction, AcceptedActionConsequence, AcceptedInteraction, ActionConsequence, ActionField,
    Activity, ActivityCursor, ActivityEntityReference, ActivityEntityRole, ActivityId,
    ActivityOperation, ActivityPage, ActivityTraitChange, ChangeEntityProperty, ChangeEntityTrait,
    Character, CharacterEntityStatePage, CreateCharacter, CreateEntity, CreateEntryPlace,
    CurrentPlaceActivityPage, CurrentPlaceEntity, CurrentPlaceEntityPage,
    CurrentPlaceEntityStatePage, Entity, EntityCurrentAssociation, EntityCurrentStateCursor,
    EntityCurrentStatePage, EntityCursor, EntityField, EntityId, EntityPage, EntityPropertyChange,
    EntityPropertyChangeInput as WorldPropertyChange, EntitySummary, EntityTrait,
    EntityTraitChangeInput as WorldTraitChange, EntityTraitId, GetEntityAtCurrentPlace,
    GetEntityCurrentState, InteractionField, IntroduceEntity, InvalidReason, ListActivity,
    ListActivityAtCurrentPlace, ListEntity, ListEntityAtCurrentPlace, Place, PlaceRevision,
    PlaceSummary, PropertyField, PropertyInput as WorldPropertyInput, PropertyValue, SubmitAction,
    SubmitInteraction, User, UserId, WorldError, WorldView,
};

pub const USER_CONTEXT_HEADER: &str = "Aicadia-User-Id";

mod error;
mod input;
mod output;

pub use error::*;
pub use input::*;
pub use output::*;

#[cfg(test)]
mod test;
