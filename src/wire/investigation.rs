use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct StartInvestigationInput {
    /// Agent-generated UUID for one investigation attempt. Reuse only to recover
    /// the same stored attempt result after uncertain delivery.
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
pub struct InvestigationLimitsOutput {
    /// Maximum number of results permitted by a positive attempt; exactly one.
    #[schemars(range(min = 1, max = 1))]
    #[schema(minimum = 1, maximum = 1)]
    pub result_count: u8,
    /// Kind of result permitted by this attempt.
    pub kind: DiscoveryKindOutput,
}

impl From<InvestigationLimits> for InvestigationLimitsOutput {
    fn from(value: InvestigationLimits) -> Self {
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
    /// Stable identity of the stored investigation attempt.
    pub attempt_id: Uuid,
    /// Server-resolved result of this investigation attempt.
    pub outcome: InvestigationOutcomeOutput,
    /// Immutable bounds on what a positive attempt permits.
    pub limits: InvestigationLimitsOutput,
}

impl From<InvestigationResult> for InvestigationResultOutput {
    fn from(value: InvestigationResult) -> Self {
        Self {
            attempt_id: value.attempt_id.0,
            outcome: value.outcome.into(),
            limits: value.limits.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct DiscoveryFindInput {
    /// Display name. World trims it and accepts 1 through 120 Unicode characters.
    #[schemars(length(min = 1, max = 120))]
    #[schema(min_length = 1, max_length = 120)]
    pub name: String,
    /// Description. World trims it and accepts 1 through 4,000 Unicode characters.
    #[schemars(length(min = 1, max = 4000))]
    #[schema(min_length = 1, max_length = 4000)]
    pub description: String,
    /// Initial typed Properties of the one found Entity.
    #[serde(default)]
    #[schemars(length(max = 100))]
    #[schema(max_items = 100)]
    pub property: Vec<PropertyInput>,
    /// Initial Traits of the one found Entity.
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
            property: value.property.into_iter().map(Into::into).collect(),
            r#trait: value.r#trait.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct SubmitDiscoveryInput {
    /// Agent-generated UUID for this one intended discovery acceptance. Reuse only
    /// for an uncertain delivery retry of semantically identical input.
    pub request_id: Uuid,
    /// Positive investigation attempt being consumed exactly once.
    pub attempt_id: Uuid,
    /// Exact canonical English discovery prose previewed and explicitly confirmed
    /// by the User.
    #[schemars(length(min = 1, max = 4000))]
    #[schema(min_length = 1, max_length = 4000)]
    pub prose: String,
    /// Exactly one Entity found through this investigation.
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
    /// Immutable accepted discovery Activity with canonical prose and exact roles.
    pub activity: ActivityOutput,
    /// Complete newly found Entity.
    pub entity: EntityOutput,
    /// Complete Place at which the Entity was found and established.
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
