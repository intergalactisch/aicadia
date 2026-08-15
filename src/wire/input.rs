use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct GetEntityInput {
    /// Stable Entity id.
    #[schemars(with = "Uuid")]
    #[schema(value_type = Uuid)]
    pub entity_id: String,
}

impl GetEntityInput {
    pub fn parse(self) -> Result<EntityId, ErrorOutput> {
        Uuid::parse_str(&self.entity_id)
            .map(EntityId)
            .map_err(|_| ErrorOutput::invalid_entity_id())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema, IntoParams)]
#[serde(default, deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
#[into_params(parameter_in = Query)]
pub struct ListEntityInput {
    /// Opaque cursor returned as `next` by a previous list response.
    pub cursor: Option<String>,
    /// Page size. Defaults to 25. The World accepts values from 1 through 100.
    #[serde(default = "default_page_limit")]
    #[schemars(default = "default_page_limit", range(min = 1, max = 100))]
    #[param(default = 25, minimum = 1, maximum = 100)]
    pub limit: i64,
}

impl Default for ListEntityInput {
    fn default() -> Self {
        Self {
            cursor: None,
            limit: default_page_limit(),
        }
    }
}

impl ListEntityInput {
    pub fn parse(self) -> Result<ListEntity, ErrorOutput> {
        let limit = u16::try_from(self.limit)
            .map_err(|_| ErrorOutput::from_world(WorldError::InvalidEntityLimit))?;
        Ok(ListEntity {
            cursor: self.cursor.as_deref().map(decode_cursor).transpose()?,
            limit,
        })
    }
}

const fn default_page_limit() -> i64 {
    25
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema, IntoParams)]
#[serde(default, deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
#[into_params(parameter_in = Query)]
pub struct ListActivityInput {
    /// Opaque cursor returned as `next` by a previous activity page.
    pub cursor: Option<String>,
    /// Page size. Defaults to 25. The World accepts values from 1 through 100.
    #[serde(default = "default_page_limit")]
    #[schemars(default = "default_page_limit", range(min = 1, max = 100))]
    #[param(default = 25, minimum = 1, maximum = 100)]
    pub limit: i64,
}

impl Default for ListActivityInput {
    fn default() -> Self {
        Self {
            cursor: None,
            limit: default_page_limit(),
        }
    }
}

impl ListActivityInput {
    pub fn parse(self) -> Result<ListActivity, ErrorOutput> {
        let limit = u16::try_from(self.limit)
            .map_err(|_| ErrorOutput::from_world(WorldError::InvalidActivityLimit))?;
        Ok(ListActivity {
            cursor: self
                .cursor
                .as_deref()
                .map(decode_activity_cursor)
                .transpose()?,
            limit,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema, IntoParams)]
#[serde(default, deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
#[into_params(parameter_in = Query)]
pub struct ListEntityAtCurrentPlaceInput {
    /// Opaque cursor returned as `next` by a previous exact-Place Entity page.
    pub cursor: Option<String>,
    /// Page size. Defaults to 25. The World accepts values from 1 through 100.
    #[serde(default = "default_page_limit")]
    #[schemars(default = "default_page_limit", range(min = 1, max = 100))]
    #[param(default = 25, minimum = 1, maximum = 100)]
    pub limit: i64,
}

impl Default for ListEntityAtCurrentPlaceInput {
    fn default() -> Self {
        Self {
            cursor: None,
            limit: default_page_limit(),
        }
    }
}

impl ListEntityAtCurrentPlaceInput {
    pub fn parse(self) -> Result<ListEntityAtCurrentPlace, ErrorOutput> {
        let limit = u16::try_from(self.limit)
            .map_err(|_| ErrorOutput::from_world(WorldError::InvalidEntityLimit))?;
        Ok(ListEntityAtCurrentPlace {
            cursor: self
                .cursor
                .as_deref()
                .map(decode_place_entity_cursor)
                .transpose()?,
            limit,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema, IntoParams)]
#[serde(default, deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
#[into_params(parameter_in = Query)]
pub struct ListActivityAtCurrentPlaceInput {
    /// Opaque cursor returned as `next` by a previous exact-Place Activity page.
    pub cursor: Option<String>,
    /// Page size. Defaults to 25. The World accepts values from 1 through 100.
    #[serde(default = "default_page_limit")]
    #[schemars(default = "default_page_limit", range(min = 1, max = 100))]
    #[param(default = 25, minimum = 1, maximum = 100)]
    pub limit: i64,
}

impl Default for ListActivityAtCurrentPlaceInput {
    fn default() -> Self {
        Self {
            cursor: None,
            limit: default_page_limit(),
        }
    }
}

impl ListActivityAtCurrentPlaceInput {
    pub fn parse(self) -> Result<ListActivityAtCurrentPlace, ErrorOutput> {
        let limit = u16::try_from(self.limit)
            .map_err(|_| ErrorOutput::from_world(WorldError::InvalidActivityLimit))?;
        Ok(ListActivityAtCurrentPlace {
            cursor: self
                .cursor
                .as_deref()
                .map(decode_place_activity_cursor)
                .transpose()?,
            limit,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema, IntoParams)]
#[serde(default, deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
#[into_params(parameter_in = Query)]
pub struct GetEntityCurrentStateInput {
    /// Opaque cursor returned by the same full-Entity fetch.
    pub cursor: Option<String>,
    /// Combined Property/Trait page size. Defaults to 25; World accepts 1 through 100.
    #[serde(default = "default_page_limit")]
    #[schemars(default = "default_page_limit", range(min = 1, max = 100))]
    #[param(default = 25, minimum = 1, maximum = 100)]
    pub limit: i64,
}

impl Default for GetEntityCurrentStateInput {
    fn default() -> Self {
        Self {
            cursor: None,
            limit: default_page_limit(),
        }
    }
}

impl GetEntityCurrentStateInput {
    pub fn parse_character(self) -> Result<GetEntityCurrentState, ErrorOutput> {
        let limit = u16::try_from(self.limit)
            .map_err(|_| ErrorOutput::from_world(WorldError::InvalidEntityLimit))?;
        Ok(GetEntityCurrentState {
            cursor: self
                .cursor
                .as_deref()
                .map(decode_character_state_cursor)
                .transpose()?,
            limit,
        })
    }

    pub fn parse_current_place_entity(
        self,
        entity_id: EntityId,
    ) -> Result<GetEntityAtCurrentPlace, ErrorOutput> {
        let limit = u16::try_from(self.limit)
            .map_err(|_| ErrorOutput::from_world(WorldError::InvalidEntityLimit))?;
        Ok(GetEntityAtCurrentPlace {
            entity_id,
            cursor: self
                .cursor
                .as_deref()
                .map(decode_current_place_entity_state_cursor)
                .transpose()?,
            limit,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct GetEntityAtCurrentPlaceInput {
    /// Stable id selected from compact exact-current-Place orientation.
    pub entity_id: Uuid,
    /// Opaque cursor returned by this same scoped Entity fetch.
    #[serde(default)]
    pub cursor: Option<String>,
    /// Combined Property/Trait page size. Defaults to 25; World accepts 1 through 100.
    #[serde(default = "default_page_limit")]
    #[schemars(default = "default_page_limit", range(min = 1, max = 100))]
    #[schema(default = 25, minimum = 1, maximum = 100)]
    pub limit: i64,
}

impl GetEntityAtCurrentPlaceInput {
    pub fn parse(self) -> Result<GetEntityAtCurrentPlace, ErrorOutput> {
        GetEntityCurrentStateInput {
            cursor: self.cursor,
            limit: self.limit,
        }
        .parse_current_place_entity(EntityId(self.entity_id))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct CreateEntityInput {
    /// Display name. The World trims it and accepts 1 through 120 Unicode characters.
    #[schemars(length(min = 1, max = 120))]
    #[schema(min_length = 1, max_length = 120)]
    pub name: String,
    /// Description. The World trims it and accepts 1 through 4,000 Unicode characters.
    #[schemars(length(min = 1, max = 4000))]
    #[schema(min_length = 1, max_length = 4000)]
    pub description: String,
    #[serde(default)]
    #[schemars(length(max = 100))]
    #[schema(max_items = 100)]
    pub property: Vec<PropertyInput>,
    #[serde(default)]
    #[schemars(length(max = 100))]
    #[schema(max_items = 100)]
    pub r#trait: Vec<TraitInput>,
}

impl From<CreateEntityInput> for CreateEntity {
    fn from(value: CreateEntityInput) -> Self {
        Self {
            name: value.name,
            description: value.description,
            property: value.property.into_iter().map(Into::into).collect(),
            r#trait: value.r#trait.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct CreateCharacterInput {
    /// Display name. The World trims it and accepts 1 through 120 Unicode characters.
    #[schemars(length(min = 1, max = 120))]
    #[schema(min_length = 1, max_length = 120)]
    pub name: String,
    /// Description. The World trims it and accepts 1 through 4,000 Unicode characters.
    #[schemars(length(min = 1, max = 4000))]
    #[schema(min_length = 1, max_length = 4000)]
    pub description: String,
    #[serde(default)]
    #[schemars(length(max = 100))]
    #[schema(max_items = 100)]
    pub property: Vec<PropertyInput>,
    #[serde(default)]
    #[schemars(length(max = 100))]
    #[schema(max_items = 100)]
    pub r#trait: Vec<TraitInput>,
}

impl From<CreateCharacterInput> for CreateCharacter {
    fn from(value: CreateCharacterInput) -> Self {
        Self {
            name: value.name,
            description: value.description,
            property: value.property.into_iter().map(Into::into).collect(),
            r#trait: value.r#trait.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct CreateEntryPlaceInput {
    /// Display name. The World trims it and accepts 1 through 120 Unicode characters.
    #[schemars(length(min = 1, max = 120))]
    #[schema(min_length = 1, max_length = 120)]
    pub name: String,
    /// Description. The World trims it and accepts 1 through 4,000 Unicode characters.
    #[schemars(length(min = 1, max = 4000))]
    #[schema(min_length = 1, max_length = 4000)]
    pub description: String,
    #[serde(default)]
    #[schemars(length(max = 100))]
    #[schema(max_items = 100)]
    pub property: Vec<PropertyInput>,
    #[serde(default)]
    #[schemars(length(max = 100))]
    #[schema(max_items = 100)]
    pub r#trait: Vec<TraitInput>,
}

impl From<CreateEntryPlaceInput> for CreateEntryPlace {
    fn from(value: CreateEntryPlaceInput) -> Self {
        Self {
            name: value.name,
            description: value.description,
            property: value.property.into_iter().map(Into::into).collect(),
            r#trait: value.r#trait.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub enum ActionConsequenceInput {
    /// Introduce one new Entity and establish it at the derived current Place.
    IntroduceEntity {
        /// Display name. World trims it and accepts 1 through 120 Unicode characters.
        #[schemars(length(min = 1, max = 120))]
        #[schema(min_length = 1, max_length = 120)]
        name: String,
        /// Description. World trims it and accepts 1 through 4,000 Unicode characters.
        #[schemars(length(min = 1, max = 4000))]
        #[schema(min_length = 1, max_length = 4000)]
        description: String,
        #[serde(default)]
        #[schemars(length(max = 100))]
        #[schema(max_items = 100)]
        property: Vec<PropertyInput>,
        #[serde(default)]
        #[schemars(length(max = 100))]
        #[schema(max_items = 100)]
        r#trait: Vec<TraitInput>,
    },
    /// Change Properties and/or Traits of exact local Entities atomically.
    ChangeEntityState {
        #[serde(default)]
        #[schemars(length(max = 100))]
        #[schema(max_items = 100)]
        property_change: Vec<EntityPropertyChangeInput>,
        #[serde(default)]
        #[schemars(length(max = 100))]
        #[schema(max_items = 100)]
        trait_change: Vec<EntityTraitChangeInput>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct SubmitActionInput {
    /// Agent-generated UUID for this one intended action. Reuse only for an
    /// uncertain delivery retry of byte-equivalent semantic input.
    pub request_id: Uuid,
    /// Opaque exact-Place revision copied unchanged from a grounded Place read.
    pub expected_place_revision: String,
    /// Exact canonical English prose previewed and explicitly confirmed by the User.
    #[schemars(length(min = 1, max = 4000))]
    #[schema(min_length = 1, max_length = 4000)]
    pub prose: String,
    /// The one closed first-slice consequence.
    pub consequence: ActionConsequenceInput,
}

impl SubmitActionInput {
    pub fn parse(self) -> Result<SubmitAction, ErrorOutput> {
        let consequence = match self.consequence {
            ActionConsequenceInput::IntroduceEntity {
                name,
                description,
                property,
                r#trait,
            } => ActionConsequence::IntroduceEntity(IntroduceEntity {
                name,
                description,
                property: property.into_iter().map(Into::into).collect(),
                r#trait: r#trait.into_iter().map(Into::into).collect(),
            }),
            ActionConsequenceInput::ChangeEntityState {
                property_change,
                trait_change,
            } => ActionConsequence::ChangeEntityState(ChangeEntityState {
                property_change: property_change.into_iter().map(Into::into).collect(),
                trait_change: trait_change.into_iter().map(Into::into).collect(),
            }),
        };
        Ok(SubmitAction {
            request_id: self.request_id,
            expected_place_revision: decode_place_revision(&self.expected_place_revision)?,
            prose: self.prose,
            consequence,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct SubmitInteractionInput {
    /// Agent-generated UUID for this one intended Interaction. Reuse only for an
    /// uncertain delivery retry of semantically identical input.
    pub request_id: Uuid,
    /// Opaque exact-Place revision copied unchanged from a grounded Place read.
    pub expected_place_revision: String,
    /// Exact canonical English outward behavior previewed and explicitly confirmed
    /// by the User. It never authors a target's response, thought or private intent.
    #[schemars(length(min = 1, max = 4000))]
    #[schema(min_length = 1, max_length = 4000)]
    pub prose: String,
    /// Unordered set of 1 through 100 distinct target Entity ids selected from the
    /// current exact-Place Entity read.
    #[schemars(length(min = 1, max = 100))]
    #[schema(min_items = 1, max_items = 100)]
    pub target_entity_id: Vec<Uuid>,
    /// Optional typed changes to the actor or explicit targets.
    #[serde(default)]
    #[schemars(length(max = 100))]
    #[schema(max_items = 100)]
    pub property_change: Vec<EntityPropertyChangeInput>,
    /// Optional mixed establishment/development of Traits owned by the actor or
    /// explicit targets. This may coexist atomically with Property changes.
    #[serde(default)]
    #[schemars(length(max = 100))]
    #[schema(max_items = 100)]
    pub trait_change: Vec<EntityTraitChangeInput>,
}

impl SubmitInteractionInput {
    pub fn parse(self) -> Result<SubmitInteraction, ErrorOutput> {
        Ok(SubmitInteraction {
            request_id: self.request_id,
            expected_place_revision: decode_place_revision(&self.expected_place_revision)?,
            prose: self.prose,
            target_entity_id: self.target_entity_id.into_iter().map(EntityId).collect(),
            property_change: self.property_change.into_iter().map(Into::into).collect(),
            trait_change: self.trait_change.into_iter().map(Into::into).collect(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct AcceptedInteractionOutput {
    /// Immutable accepted Interaction with actor, location, targets and canonical
    /// outward behavior.
    pub activity: ActivityOutput,
    /// Flat safe id, name and description of the exact Place at which the
    /// Interaction was accepted; complete Entity provenance and entry status are omitted.
    pub place: CurrentPlaceOutput,
}

impl From<AcceptedInteraction> for AcceptedInteractionOutput {
    fn from(value: AcceptedInteraction) -> Self {
        Self {
            activity: value.activity.into(),
            place: value.place.into(),
        }
    }
}
