use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct StartInvestigationInput {
    /// Fresh UUID for this one attempt; reuse only to retry an uncertain delivery.
    pub request_id: Uuid,
}

impl From<StartInvestigationInput> for StartInvestigation {
    fn from(value: StartInvestigationInput) -> Self {
        Self {
            request_id: value.request_id,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum InvestigationOutcomeOutput {
    Zero,
    Positive,
}

impl From<InvestigationOutcome> for InvestigationOutcomeOutput {
    fn from(value: InvestigationOutcome) -> Self {
        match value {
            InvestigationOutcome::Zero => Self::Zero,
            InvestigationOutcome::Positive => Self::Positive,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum DiscoveryKindOutput {
    EntityAtCurrentPlace,
}

impl From<DiscoveryKind> for DiscoveryKindOutput {
    fn from(value: DiscoveryKind) -> Self {
        match value {
            DiscoveryKind::EntityAtCurrentPlace => Self::EntityAtCurrentPlace,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct InvestigationLimitOutput {
    /// Number of results a positive attempt permits: one.
    #[schemars(range(min = 1, max = 1))]
    #[schema(minimum = 1, maximum = 1)]
    pub result_count: u8,
    /// Kind of result permitted.
    pub kind: DiscoveryKindOutput,
}

impl From<InvestigationLimit> for InvestigationLimitOutput {
    fn from(value: InvestigationLimit) -> Self {
        Self {
            result_count: value.result_count,
            kind: value.kind.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct InvestigationResultOutput {
    /// Attempt id.
    pub attempt_id: Uuid,
    /// Outcome: zero or positive.
    pub outcome: InvestigationOutcomeOutput,
    /// What a positive attempt permits.
    pub limit: InvestigationLimitOutput,
}

impl From<InvestigationResult> for InvestigationResultOutput {
    fn from(value: InvestigationResult) -> Self {
        Self {
            attempt_id: value.attempt_id.0,
            outcome: value.outcome.into(),
            limit: value.limit.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct DiscoveryFindInput {
    /// Display name.
    #[schemars(length(min = 1, max = 120))]
    #[schema(min_length = 1, max_length = 120)]
    pub name: String,
    /// Description.
    #[schemars(length(min = 1, max = 4000))]
    #[schema(min_length = 1, max_length = 4000)]
    pub description: String,
    /// Initial Properties of the found Entity.
    #[serde(default)]
    #[schemars(length(max = 100))]
    #[schema(max_items = 100)]
    pub property: Vec<PropertyInput>,
    /// Initial Traits of the found Entity.
    #[serde(default)]
    #[schemars(length(max = 100))]
    #[schema(max_items = 100)]
    pub r#trait: Vec<TraitInput>,
}

impl From<DiscoveryFindInput> for DiscoveryFind {
    fn from(value: DiscoveryFindInput) -> Self {
        Self {
            name: value.name,
            description: value.description,
            position_description: None,
            property: value.property.into_iter().map(Into::into).collect(),
            r#trait: value.r#trait.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct SubmitDiscoveryInput {
    /// Fresh UUID for this one discovery; reuse only to retry an uncertain delivery.
    pub request_id: Uuid,
    /// The positive attempt this discovery completes.
    pub attempt_id: Uuid,
    /// English discovery prose the User confirmed.
    #[schemars(length(min = 1, max = 4000))]
    #[schema(min_length = 1, max_length = 4000)]
    pub prose: String,
    /// The one found Entity.
    pub find: DiscoveryFindInput,
}

impl From<SubmitDiscoveryInput> for SubmitDiscovery {
    fn from(value: SubmitDiscoveryInput) -> Self {
        Self {
            request_id: value.request_id,
            attempt_id: InvestigationAttemptId(value.attempt_id),
            prose: value.prose,
            find: value.find.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct AcceptedDiscoveryOutput {
    /// The accepted discovery Activity.
    pub activity: ActivityOutput,
    /// The found Entity.
    pub entity: EntityOutput,
    /// The Place where the Entity was found.
    pub place: PlaceOutput,
}

impl From<AcceptedDiscovery> for AcceptedDiscoveryOutput {
    fn from(value: AcceptedDiscovery) -> Self {
        Self {
            activity: value.activity.into(),
            entity: value.entity.into(),
            place: value.place.into(),
        }
    }
}
