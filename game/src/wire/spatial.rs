use super::*;

const DEFAULT_PAGE_LIMIT: i64 = 25;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct PositionOutput {
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
    #[schemars(schema_with = "nullable_description", required)]
    #[schema(required = true, nullable = true, min_length = 1, max_length = 4000)]
    pub description: Option<String>,
    /// Opaque current Position revision.
    #[schemars(length(min = 1, max = 256))]
    #[schema(min_length = 1, max_length = 256)]
    pub position_revision: String,
}

impl From<Position> for PositionOutput {
    fn from(value: Position) -> Self {
        Self {
            x_cm: value.x_cm,
            y_cm: value.y_cm,
            z_cm: value.z_cm,
            description: value.description,
            position_revision: encode_position_revision(value.position_revision),
        }
    }
}

fn nullable_description(_: &mut SchemaGenerator) -> Schema {
    schemars::json_schema!({"type": ["string", "null"], "minLength": 1, "maxLength": 4000})
}

fn nullable_cursor(_: &mut SchemaGenerator) -> Schema {
    schemars::json_schema!({"type": ["string", "null"], "minLength": 1, "maxLength": 512})
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct PlacePositionOutput {
    /// Place id.
    pub id: Uuid,
    /// Current name.
    #[schemars(length(min = 1, max = 120))]
    #[schema(min_length = 1, max_length = 120)]
    pub name: String,
    /// Current description.
    #[schemars(length(min = 1, max = 4000))]
    #[schema(min_length = 1, max_length = 4000)]
    pub description: String,
    /// True only for the entry Place.
    pub is_entry: bool,
    /// Exact current Position.
    pub position: PositionOutput,
}

impl From<crate::PlacePosition> for PlacePositionOutput {
    fn from(value: crate::PlacePosition) -> Self {
        Self {
            id: value.id.0,
            name: value.name,
            description: value.description,
            is_entry: value.is_entry,
            position: value.position.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct ConnectionEndpointOutput {
    /// Place at this endpoint.
    pub place: PlacePositionOutput,
}

impl From<crate::ConnectionEndpoint> for ConnectionEndpointOutput {
    fn from(value: crate::ConnectionEndpoint) -> Self {
        Self {
            place: value.place.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct ConnectionSummaryOutput {
    /// Stable Connection id.
    pub id: Uuid,
    /// Source endpoint in the Connection's stored direction.
    pub source: ConnectionEndpointOutput,
    /// Destination endpoint in the Connection's stored direction.
    pub destination: ConnectionEndpointOutput,
    /// Whether destination-to-source Movement is allowed.
    pub allows_reverse: bool,
    /// Current Connection name.
    #[schemars(length(min = 1, max = 120))]
    #[schema(min_length = 1, max_length = 120)]
    pub name: String,
    /// Current Connection description.
    #[schemars(length(min = 1, max = 4000))]
    #[schema(min_length = 1, max_length = 4000)]
    pub description: String,
    /// Whether `get_connection` returns a course.
    pub has_course: bool,
}

impl From<crate::ConnectionSummary> for ConnectionSummaryOutput {
    fn from(value: crate::ConnectionSummary) -> Self {
        Self {
            id: value.id.0,
            source: value.source.into(),
            destination: value.destination.into(),
            allows_reverse: value.allows_reverse,
            name: value.name,
            description: value.description,
            has_course: value.has_course,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct ConnectionPointOutput {
    /// Zero-based point order within the selected course.
    #[schemars(range(max = 127))]
    #[schema(maximum = 127)]
    pub ordinal: u16,
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

impl From<crate::ConnectionPoint> for ConnectionPointOutput {
    fn from(value: crate::ConnectionPoint) -> Self {
        Self {
            ordinal: value.ordinal,
            x_cm: value.x_cm,
            y_cm: value.y_cm,
            z_cm: value.z_cm,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct ConnectionOutput {
    /// Stable Connection id.
    pub id: Uuid,
    /// Source endpoint in the Connection's stored direction.
    pub source: ConnectionEndpointOutput,
    /// Destination endpoint in the Connection's stored direction.
    pub destination: ConnectionEndpointOutput,
    /// Whether destination-to-source Movement is allowed.
    pub allows_reverse: bool,
    /// Current Connection name.
    #[schemars(length(min = 1, max = 120))]
    #[schema(min_length = 1, max_length = 120)]
    pub name: String,
    /// Current Connection description.
    #[schemars(length(min = 1, max = 4000))]
    #[schema(min_length = 1, max_length = 4000)]
    pub description: String,
    /// Optional narrative guidance for the Connection shape.
    #[schemars(schema_with = "nullable_description", required)]
    #[schema(required = true, nullable = true, min_length = 1, max_length = 4000)]
    pub shape_description: Option<String>,
    /// Empty when unshaped, otherwise the complete ordered course.
    #[schemars(schema_with = "connection_output_course_schema")]
    #[schema(schema_with = connection_output_course_openapi_schema)]
    pub course: Vec<ConnectionPointOutput>,
}

fn connection_output_course_schema(generator: &mut SchemaGenerator) -> Schema {
    let point = generator.subschema_for::<ConnectionPointOutput>();
    schemars::json_schema!({
        "oneOf": [
            {"type": "array", "maxItems": 0, "items": point.clone()},
            {"type": "array", "minItems": 2, "maxItems": 128, "items": point}
        ]
    })
}

fn connection_output_course_openapi_schema() -> utoipa::openapi::schema::Schema {
    use utoipa::openapi::schema::{ArrayBuilder, OneOfBuilder, Schema};

    let point = <ConnectionPointOutput as utoipa::PartialSchema>::schema();
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

impl From<crate::Connection> for ConnectionOutput {
    fn from(value: crate::Connection) -> Self {
        Self {
            id: value.id.0,
            source: value.source.into(),
            destination: value.destination.into(),
            allows_reverse: value.allows_reverse,
            name: value.name,
            description: value.description,
            shape_description: value.shape_description,
            course: value.course.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct PlacePageOutput {
    /// At most 100 Places inside the requested window.
    #[schemars(length(max = 100))]
    #[schema(max_items = 100)]
    pub place: Vec<PlacePositionOutput>,
    /// Opaque continuation cursor, or null after the final page.
    #[schemars(schema_with = "nullable_cursor", required)]
    #[schema(required = true, nullable = true, min_length = 1, max_length = 512)]
    pub next: Option<String>,
}

impl From<crate::PlacePage> for PlacePageOutput {
    fn from(value: crate::PlacePage) -> Self {
        Self {
            place: value.place.into_iter().map(Into::into).collect(),
            next: value.next,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct ConnectionPageOutput {
    /// Anchor Place used for this page.
    pub place: PlacePositionOutput,
    /// At most 100 incident Connection summaries without courses.
    #[schemars(length(max = 100))]
    #[schema(max_items = 100)]
    pub connection: Vec<ConnectionSummaryOutput>,
    /// Opaque continuation cursor, or null after the final page.
    #[schemars(schema_with = "nullable_cursor", required)]
    #[schema(required = true, nullable = true, min_length = 1, max_length = 512)]
    pub next: Option<String>,
}

impl From<crate::ConnectionPage> for ConnectionPageOutput {
    fn from(value: crate::ConnectionPage) -> Self {
        Self {
            place: value.place.into(),
            connection: value.connection.into_iter().map(Into::into).collect(),
            next: value.next,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema, IntoParams)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
#[into_params(parameter_in = Query)]
pub struct ListPlaceInput {
    /// Inclusive minimum World X coordinate in centimetres.
    #[schemars(range(min = -1000000000000000_i64, max = 1000000000000000_i64))]
    #[param(minimum = -1000000000000000_i64, maximum = 1000000000000000_i64)]
    pub min_x_cm: i64,
    /// Inclusive maximum World X coordinate in centimetres.
    #[schemars(range(min = -1000000000000000_i64, max = 1000000000000000_i64))]
    #[param(minimum = -1000000000000000_i64, maximum = 1000000000000000_i64)]
    pub max_x_cm: i64,
    /// Inclusive minimum World Y coordinate in centimetres.
    #[schemars(range(min = -1000000000000000_i64, max = 1000000000000000_i64))]
    #[param(minimum = -1000000000000000_i64, maximum = 1000000000000000_i64)]
    pub min_y_cm: i64,
    /// Inclusive maximum World Y coordinate in centimetres.
    #[schemars(range(min = -1000000000000000_i64, max = 1000000000000000_i64))]
    #[param(minimum = -1000000000000000_i64, maximum = 1000000000000000_i64)]
    pub max_y_cm: i64,
    /// Inclusive minimum World Z coordinate in centimetres.
    #[schemars(range(min = -1000000000000000_i64, max = 1000000000000000_i64))]
    #[param(minimum = -1000000000000000_i64, maximum = 1000000000000000_i64)]
    pub min_z_cm: i64,
    /// Inclusive maximum World Z coordinate in centimetres.
    #[schemars(range(min = -1000000000000000_i64, max = 1000000000000000_i64))]
    #[param(minimum = -1000000000000000_i64, maximum = 1000000000000000_i64)]
    pub max_z_cm: i64,
    /// Opaque cursor returned for this unchanged window.
    #[schemars(length(min = 1, max = 512))]
    #[param(min_length = 1, max_length = 512)]
    pub cursor: Option<String>,
    /// Maximum Places to return, defaulting to 25.
    #[serde(default = "default_limit")]
    #[schemars(default = "default_limit", range(min = 1, max = 100))]
    #[param(default = 25, minimum = 1, maximum = 100)]
    pub limit: i64,
}

const fn default_limit() -> i64 {
    DEFAULT_PAGE_LIMIT
}

impl Default for ListPlaceInput {
    fn default() -> Self {
        Self {
            min_x_cm: 0,
            max_x_cm: 0,
            min_y_cm: 0,
            max_y_cm: 0,
            min_z_cm: 0,
            max_z_cm: 0,
            cursor: None,
            limit: default_limit(),
        }
    }
}

impl ListPlaceInput {
    pub fn parse(self) -> Result<crate::ListPlace, ErrorOutput> {
        Ok(crate::ListPlace {
            min_x_cm: self.min_x_cm,
            max_x_cm: self.max_x_cm,
            min_y_cm: self.min_y_cm,
            max_y_cm: self.max_y_cm,
            min_z_cm: self.min_z_cm,
            max_z_cm: self.max_z_cm,
            cursor: self.cursor,
            limit: u16::try_from(self.limit)
                .map_err(|_| ErrorOutput::from_world(WorldError::InvalidPlaceLimit))?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct ListConnectionInput {
    /// Anchor Place id from World.
    pub place_id: Uuid,
    /// Opaque cursor returned for this unchanged Place.
    #[serde(default)]
    #[schemars(length(min = 1, max = 512))]
    #[schema(min_length = 1, max_length = 512)]
    pub cursor: Option<String>,
    /// Maximum Connection summaries to return, defaulting to 25.
    #[serde(default = "default_limit")]
    #[schemars(default = "default_limit", range(min = 1, max = 100))]
    #[schema(default = 25, minimum = 1, maximum = 100)]
    pub limit: i64,
}

impl ListConnectionInput {
    pub fn parse(self) -> Result<crate::ListConnection, ErrorOutput> {
        Ok(crate::ListConnection {
            place_id: EntityId(self.place_id),
            cursor: self.cursor,
            limit: u16::try_from(self.limit)
                .map_err(|_| ErrorOutput::from_world(WorldError::InvalidConnectionLimit))?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema, IntoParams)]
#[serde(default, deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
#[into_params(parameter_in = Query)]
pub struct ListConnectionPageInput {
    /// Opaque cursor returned for this unchanged Place.
    #[schemars(length(min = 1, max = 512))]
    #[param(min_length = 1, max_length = 512)]
    pub cursor: Option<String>,
    /// Maximum Connection summaries to return, defaulting to 25.
    #[serde(default = "default_limit")]
    #[schemars(default = "default_limit", range(min = 1, max = 100))]
    #[param(default = 25, minimum = 1, maximum = 100)]
    pub limit: i64,
}

impl Default for ListConnectionPageInput {
    fn default() -> Self {
        Self {
            cursor: None,
            limit: default_limit(),
        }
    }
}

impl ListConnectionPageInput {
    pub fn parse(self, place_id: Uuid) -> Result<crate::ListConnection, ErrorOutput> {
        ListConnectionInput {
            place_id,
            cursor: self.cursor,
            limit: self.limit,
        }
        .parse()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct GetConnectionInput {
    /// Anchor Place id from World.
    pub place_id: Uuid,
    /// Incident Connection id from World.
    pub connection_id: Uuid,
}

impl From<GetConnectionInput> for crate::GetConnection {
    fn from(value: GetConnectionInput) -> Self {
        Self {
            place_id: EntityId(value.place_id),
            connection_id: crate::ConnectionId(value.connection_id),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum MovementDirectionInput {
    /// Move from the stored source toward the destination.
    SourceToDestination,
    /// Move from the stored destination toward the source.
    DestinationToSource,
}

impl From<MovementDirectionInput> for crate::MovementDirection {
    fn from(value: MovementDirectionInput) -> Self {
        match value {
            MovementDirectionInput::SourceToDestination => Self::SourceToDestination,
            MovementDirectionInput::DestinationToSource => Self::DestinationToSource,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub enum MovementTargetInput {
    /// Arrive at the selected direction's endpoint Place.
    Complete,
    Partial {
        /// Segment containing the current Position.
        #[schemars(range(max = 126))]
        #[schema(maximum = 126)]
        origin_segment_ordinal: u8,
        /// Segment containing the requested target.
        #[schemars(range(max = 126))]
        #[schema(maximum = 126)]
        target_segment_ordinal: u8,
        /// Exact target X coordinate in centimetres.
        #[schemars(range(min = -1000000000000000_i64, max = 1000000000000000_i64))]
        #[schema(minimum = -1000000000000000_i64, maximum = 1000000000000000_i64)]
        x_cm: i64,
        /// Exact target Y coordinate in centimetres.
        #[schemars(range(min = -1000000000000000_i64, max = 1000000000000000_i64))]
        #[schema(minimum = -1000000000000000_i64, maximum = 1000000000000000_i64)]
        y_cm: i64,
        /// Exact target Z coordinate in centimetres.
        #[schemars(range(min = -1000000000000000_i64, max = 1000000000000000_i64))]
        #[schema(minimum = -1000000000000000_i64, maximum = 1000000000000000_i64)]
        z_cm: i64,
    },
}

impl From<MovementTargetInput> for crate::MovementTarget {
    fn from(value: MovementTargetInput) -> Self {
        match value {
            MovementTargetInput::Complete => Self::Complete,
            MovementTargetInput::Partial {
                origin_segment_ordinal,
                target_segment_ordinal,
                x_cm,
                y_cm,
                z_cm,
            } => Self::Partial {
                origin_segment_ordinal,
                target_segment_ordinal,
                x_cm,
                y_cm,
                z_cm,
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct MoveCharacterInput {
    /// Stable id for exact retry of this Movement.
    pub request_id: Uuid,
    /// Selected Connection id from World.
    pub connection_id: Uuid,
    /// Opaque current Position revision returned by World.
    #[schemars(length(min = 1, max = 256))]
    #[schema(min_length = 1, max_length = 256)]
    pub expected_position_revision: String,
    /// Travel direction over the selected Connection.
    pub direction: MovementDirectionInput,
    /// Complete arrival or one exact forward course point.
    pub target: MovementTargetInput,
}

impl MoveCharacterInput {
    pub fn parse(self) -> Result<crate::MoveCharacter, ErrorOutput> {
        Ok(crate::MoveCharacter {
            request_id: self.request_id,
            connection_id: crate::ConnectionId(self.connection_id),
            expected_position_revision: decode_position_revision(&self.expected_position_revision)?,
            direction: self.direction.into(),
            target: self.target.into(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct AcceptedMovementOutput {
    /// Immutable accepted Movement history.
    pub activity: ActivityOutput,
    /// Character after the accepted Movement.
    pub character: CharacterOutput,
    /// Complete traversed Connection.
    pub connection: ConnectionOutput,
}

impl From<crate::AcceptedMovement> for AcceptedMovementOutput {
    fn from(value: crate::AcceptedMovement) -> Self {
        Self {
            activity: value.activity.into(),
            character: value.character.into(),
            connection: value.connection.into(),
        }
    }
}

pub(super) fn encode_position_revision(revision: PositionRevision) -> String {
    URL_SAFE_NO_PAD.encode(format!(
        "ps1|{}|{}",
        revision.entity_id().0,
        revision.activity_id().0
    ))
}

pub(super) fn decode_position_revision(value: &str) -> Result<PositionRevision, ErrorOutput> {
    if value.len() > 256 {
        return Err(ErrorOutput::invalid_position_revision());
    }
    let decoded = URL_SAFE_NO_PAD
        .decode(value)
        .map_err(|_| ErrorOutput::invalid_position_revision())?;
    let decoded =
        std::str::from_utf8(&decoded).map_err(|_| ErrorOutput::invalid_position_revision())?;
    let mut part = decoded.split('|');
    let (version, entity_id, activity_id) = (part.next(), part.next(), part.next());
    if version != Some("ps1") || part.next().is_some() {
        return Err(ErrorOutput::invalid_position_revision());
    }
    let entity_id = Uuid::parse_str(entity_id.ok_or_else(ErrorOutput::invalid_position_revision)?)
        .map_err(|_| ErrorOutput::invalid_position_revision())?;
    let activity_id =
        Uuid::parse_str(activity_id.ok_or_else(ErrorOutput::invalid_position_revision)?)
            .map_err(|_| ErrorOutput::invalid_position_revision())?;
    Ok(PositionRevision::from_parts(
        EntityId(entity_id),
        ActivityId(activity_id),
    ))
}
