use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum DiscoveryKindInput {
    /// Search for one Entity at the Character's Position.
    EntityAtPosition,
    /// Search for one Place and its new Connection.
    ConnectedPlace,
}

impl From<DiscoveryKindInput> for DiscoveryKind {
    fn from(value: DiscoveryKindInput) -> Self {
        match value {
            DiscoveryKindInput::EntityAtPosition => Self::EntityAtPosition,
            DiscoveryKindInput::ConnectedPlace => Self::ConnectedPlace,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct StartInvestigationInput {
    /// Stable id for exact retry of this investigation start.
    pub request_id: Uuid,
    /// Kind the accepted discovery must match.
    pub kind: DiscoveryKindInput,
}

impl From<StartInvestigationInput> for StartInvestigation {
    fn from(value: StartInvestigationInput) -> Self {
        Self {
            request_id: value.request_id,
            kind: value.kind.into(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum InvestigationOutcomeOutput {
    /// This investigation found no result.
    Zero,
    /// This investigation permits one matching discovery submission.
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
    /// One Entity at a Position.
    EntityAtPosition,
    /// One Place and its new Connection.
    ConnectedPlace,
}

impl From<DiscoveryKind> for DiscoveryKindOutput {
    fn from(value: DiscoveryKind) -> Self {
        match value {
            DiscoveryKind::EntityAtPosition => Self::EntityAtPosition,
            DiscoveryKind::ConnectedPlace => Self::ConnectedPlace,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct InvestigationLimitOutput {
    /// Maximum discoveries this attempt can accept.
    #[schemars(range(min = 1, max = 1))]
    #[schema(minimum = 1, maximum = 1)]
    pub result_count: u8,
    /// Required accepted discovery kind.
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
    /// Stable attempt id for a positive discovery submission.
    pub attempt_id: Uuid,
    /// Stored investigation outcome.
    pub outcome: InvestigationOutcomeOutput,
    /// Exact submission boundary for this attempt.
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
pub struct PlaceEntityInputWire {
    /// New Place name.
    #[schemars(length(min = 1, max = 120))]
    #[schema(min_length = 1, max_length = 120)]
    pub name: String,
    /// New Place description.
    #[schemars(length(min = 1, max = 4000))]
    #[schema(min_length = 1, max_length = 4000)]
    pub description: String,
    /// Initial Place Properties.
    #[serde(default)]
    #[schemars(length(max = 100))]
    #[schema(max_items = 100)]
    pub property: Vec<PropertyInput>,
    /// Initial Place Traits.
    #[serde(default)]
    #[schemars(length(max = 100))]
    #[schema(max_items = 100)]
    pub r#trait: Vec<TraitInput>,
}

impl From<PlaceEntityInputWire> for crate::PlaceEntityInput {
    fn from(value: PlaceEntityInputWire) -> Self {
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
pub struct DirectPositionInputWire {
    /// World X coordinate in centimetres.
    #[schemars(range(min = -1000000000000000_i64, max = 1000000000000000_i64))]
    #[schema(minimum = -1000000000000000_i64, maximum = 1000000000000000_i64)]
    pub x_cm: i64,
    /// World Y coordinate in centimetres.
    #[schemars(range(min = -1000000000000000_i64, max = 1000000000000000_i64))]
    #[schema(minimum = -1000000000000000_i64, maximum = 1000000000000000_i64)]
    pub y_cm: i64,
    /// World Z coordinate in centimetres.
    #[schemars(range(min = -1000000000000000_i64, max = 1000000000000000_i64))]
    #[schema(minimum = -1000000000000000_i64, maximum = 1000000000000000_i64)]
    pub z_cm: i64,
    /// Optional narrative guidance for this Position.
    #[serde(default)]
    #[schemars(length(min = 1, max = 4000))]
    #[schema(min_length = 1, max_length = 4000)]
    pub description: Option<String>,
}

impl From<DirectPositionInputWire> for crate::DirectPositionInput {
    fn from(value: DirectPositionInputWire) -> Self {
        Self {
            x_cm: value.x_cm,
            y_cm: value.y_cm,
            z_cm: value.z_cm,
            description: value.description,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct ConnectionPointInputWire {
    /// Exact course-point X coordinate in centimetres.
    #[schemars(range(min = -1000000000000000_i64, max = 1000000000000000_i64))]
    #[schema(minimum = -1000000000000000_i64, maximum = 1000000000000000_i64)]
    pub x_cm: i64,
    /// Exact course-point Y coordinate in centimetres.
    #[schemars(range(min = -1000000000000000_i64, max = 1000000000000000_i64))]
    #[schema(minimum = -1000000000000000_i64, maximum = 1000000000000000_i64)]
    pub y_cm: i64,
    /// Exact course-point Z coordinate in centimetres.
    #[schemars(range(min = -1000000000000000_i64, max = 1000000000000000_i64))]
    #[schema(minimum = -1000000000000000_i64, maximum = 1000000000000000_i64)]
    pub z_cm: i64,
}

impl From<ConnectionPointInputWire> for crate::ConnectionPointInput {
    fn from(value: ConnectionPointInputWire) -> Self {
        Self {
            x_cm: value.x_cm,
            y_cm: value.y_cm,
            z_cm: value.z_cm,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct ConnectionInputWire {
    /// Connection name.
    #[schemars(length(min = 1, max = 120))]
    #[schema(min_length = 1, max_length = 120)]
    pub name: String,
    /// Connection description.
    #[schemars(length(min = 1, max = 4000))]
    #[schema(min_length = 1, max_length = 4000)]
    pub description: String,
    /// Optional narrative guidance for the Connection shape.
    #[serde(default)]
    #[schemars(length(min = 1, max = 4000))]
    #[schema(min_length = 1, max_length = 4000)]
    pub shape_description: Option<String>,
    /// Whether destination-to-source Movement is allowed.
    pub allows_reverse: bool,
    /// Empty for unshaped travel, otherwise two through 128 ordered points.
    #[serde(default)]
    #[schemars(schema_with = "connection_course_schema")]
    #[schema(schema_with = connection_course_openapi_schema)]
    pub course: Vec<ConnectionPointInputWire>,
}

fn connection_course_schema(generator: &mut SchemaGenerator) -> Schema {
    let point = generator.subschema_for::<ConnectionPointInputWire>();
    schemars::json_schema!({
        "oneOf": [
            {"type": "array", "maxItems": 0, "items": point.clone()},
            {"type": "array", "minItems": 2, "maxItems": 128, "items": point}
        ]
    })
}

fn connection_course_openapi_schema() -> utoipa::openapi::schema::Schema {
    use utoipa::openapi::schema::{ArrayBuilder, OneOfBuilder, Schema};

    let point = <ConnectionPointInputWire as utoipa::PartialSchema>::schema();
    Schema::OneOf(
        OneOfBuilder::new()
            .item(ArrayBuilder::new().items(point.clone()).max_items(Some(0)))
            .item(
                ArrayBuilder::new()
                    .items(point)
                    .min_items(Some(2))
                    .max_items(Some(128)),
            )
            .build(),
    )
}

impl From<ConnectionInputWire> for crate::ConnectionInput {
    fn from(value: ConnectionInputWire) -> Self {
        Self {
            name: value.name,
            description: value.description,
            shape_description: value.shape_description,
            allows_reverse: value.allows_reverse,
            course: value.course.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub enum DiscoveryOriginInputWire {
    /// Use the attempt's current Place.
    AttemptPlace,
    New {
        /// Complete new origin Place state.
        entity: PlaceEntityInputWire,
        /// Optional narrative guidance for the origin Position.
        #[serde(default)]
        #[schemars(length(min = 1, max = 4000))]
        #[schema(min_length = 1, max_length = 4000)]
        position_description: Option<String>,
    },
    /// Use an existing Place at the Character's exact Position.
    Existing {
        /// Existing origin Place id from World.
        place_id: Uuid,
    },
}

impl From<DiscoveryOriginInputWire> for crate::DiscoveryOriginInput {
    fn from(value: DiscoveryOriginInputWire) -> Self {
        match value {
            DiscoveryOriginInputWire::AttemptPlace => Self::AttemptPlace,
            DiscoveryOriginInputWire::New {
                entity,
                position_description,
            } => Self::New {
                entity: entity.into(),
                position_description,
            },
            DiscoveryOriginInputWire::Existing { place_id } => Self::Existing {
                place_id: EntityId(place_id),
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub enum DiscoveryDestinationInputWire {
    /// Create a destination Place at one exact Position.
    New {
        /// Complete new destination Place state.
        entity: PlaceEntityInputWire,
        /// Exact new destination Position.
        position: DirectPositionInputWire,
    },
    /// Use one existing destination Place.
    Existing {
        /// Existing destination Place id from World.
        place_id: Uuid,
    },
}

impl From<DiscoveryDestinationInputWire> for crate::DiscoveryDestinationInput {
    fn from(value: DiscoveryDestinationInputWire) -> Self {
        match value {
            DiscoveryDestinationInputWire::New { entity, position } => Self::New {
                entity: entity.into(),
                position: position.into(),
            },
            DiscoveryDestinationInputWire::Existing { place_id } => Self::Existing {
                place_id: EntityId(place_id),
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub enum DiscoveryResultInputWire {
    /// Establish one found Entity at the Character's Position.
    EntityAtPosition {
        /// Found Entity name.
        #[schemars(length(min = 1, max = 120))]
        #[schema(min_length = 1, max_length = 120)]
        name: String,
        /// Found Entity description.
        #[schemars(length(min = 1, max = 4000))]
        #[schema(min_length = 1, max_length = 4000)]
        description: String,
        /// Optional narrative guidance for the found Entity Position.
        #[serde(default)]
        #[schemars(length(min = 1, max = 4000))]
        #[schema(min_length = 1, max_length = 4000)]
        position_description: Option<String>,
        /// Initial found Entity Properties.
        #[serde(default)]
        #[schemars(length(max = 100))]
        #[schema(max_items = 100)]
        property: Vec<PropertyInput>,
        /// Initial found Entity Traits.
        #[serde(default)]
        #[schemars(length(max = 100))]
        #[schema(max_items = 100)]
        r#trait: Vec<TraitInput>,
    },
    /// Establish one Place and one new Connection.
    ConnectedPlace {
        /// Selected or new origin Place.
        origin: DiscoveryOriginInputWire,
        /// Selected or new destination Place.
        destination: Box<DiscoveryDestinationInputWire>,
        /// Complete new Connection.
        connection: Box<ConnectionInputWire>,
    },
}

impl From<DiscoveryResultInputWire> for DiscoveryResultInput {
    fn from(value: DiscoveryResultInputWire) -> Self {
        match value {
            DiscoveryResultInputWire::EntityAtPosition {
                name,
                description,
                position_description,
                property,
                r#trait,
            } => Self::EntityAtPosition {
                name,
                description,
                position_description,
                property: property.into_iter().map(Into::into).collect(),
                r#trait: r#trait.into_iter().map(Into::into).collect(),
            },
            DiscoveryResultInputWire::ConnectedPlace {
                origin,
                destination,
                connection,
            } => Self::ConnectedPlace {
                origin: origin.into(),
                destination: (*destination).into(),
                connection: (*connection).into(),
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct SubmitDiscoveryInput {
    /// Stable id for exact retry of this discovery submission.
    pub request_id: Uuid,
    /// Positive attempt id returned by `start_investigation`.
    pub attempt_id: Uuid,
    /// Complete accepted discovery narration.
    #[schemars(length(min = 1, max = 4000))]
    #[schema(min_length = 1, max_length = 4000)]
    pub prose: String,
    /// Exact result matching the attempt kind.
    pub result: DiscoveryResultInputWire,
}

impl From<SubmitDiscoveryInput> for SubmitDiscovery {
    fn from(value: SubmitDiscoveryInput) -> Self {
        Self {
            request_id: value.request_id,
            attempt_id: InvestigationAttemptId(value.attempt_id),
            prose: value.prose,
            result: value.result.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub enum AcceptedDiscoveryOutput {
    /// Accepted Entity discovery at one Position.
    EntityAtPosition {
        /// Immutable accepted discovery history.
        activity: ActivityOutput,
        /// Found Entity.
        entity: EntityOutput,
        /// Exact found Position.
        position: PositionOutput,
        /// Existing Place at that Position, when one exists.
        #[schemars(schema_with = "nullable_place", required)]
        #[schema(required = true, nullable = true)]
        place: Option<PlaceOutput>,
    },
    /// Accepted Place discovery and new Connection.
    ConnectedPlace {
        /// Immutable accepted discovery history.
        activity: ActivityOutput,
        /// Accepted origin Place.
        origin: PlaceOutput,
        /// Accepted destination Place.
        destination: PlaceOutput,
        /// Accepted new Connection.
        connection: Box<ConnectionOutput>,
        /// Character after discovery without Movement.
        character: Box<CharacterOutput>,
    },
}

fn nullable_place(generator: &mut SchemaGenerator) -> Schema {
    let place = generator.subschema_for::<PlaceOutput>();
    schemars::json_schema!({"oneOf": [place, {"type": "null"}]})
}

impl From<AcceptedDiscovery> for AcceptedDiscoveryOutput {
    fn from(value: AcceptedDiscovery) -> Self {
        match value {
            AcceptedDiscovery::EntityAtPosition {
                activity,
                entity,
                position,
                place,
            } => Self::EntityAtPosition {
                activity: activity.into(),
                entity: entity.into(),
                position: position.into(),
                place: place.map(Into::into),
            },
            AcceptedDiscovery::ConnectedPlace {
                activity,
                origin,
                destination,
                connection,
                character,
            } => Self::ConnectedPlace {
                activity: activity.into(),
                origin: origin.into(),
                destination: destination.into(),
                connection: Box::new(connection.into()),
                character: Box::new(character.into()),
            },
        }
    }
}
