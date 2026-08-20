use super::super::*;

const MAX_DISCOVERY_PROSE_LENGTH: usize = 4_000;
const MAX_CONNECTION_COURSE_POINT_COUNT: usize = 128;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, sqlx::Type)]
#[sqlx(transparent)]
pub struct InvestigationAttemptId(pub Uuid);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InvestigationOutcome {
    Zero,
    Positive,
}

impl InvestigationOutcome {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::Zero => "zero",
            Self::Positive => "positive",
        }
    }

    pub(super) fn parse(value: &str) -> Result<Self, WorldError> {
        match value {
            "zero" => Ok(Self::Zero),
            "positive" => Ok(Self::Positive),
            _ => Err(invalid_stored_relation()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiscoveryKind {
    EntityAtPosition,
    ConnectedPlace,
}

impl DiscoveryKind {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::EntityAtPosition => "entity_at_position",
            Self::ConnectedPlace => "connected_place",
        }
    }

    pub(super) fn parse(value: &str) -> Result<Self, WorldError> {
        match value {
            "entity_at_position" => Ok(Self::EntityAtPosition),
            "connected_place" => Ok(Self::ConnectedPlace),
            _ => Err(invalid_stored_relation()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvestigationLimit {
    pub result_count: u8,
    pub kind: DiscoveryKind,
}

impl InvestigationLimit {
    pub(super) const fn for_kind(kind: DiscoveryKind) -> Self {
        Self {
            result_count: 1,
            kind,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StartInvestigation {
    pub request_id: Uuid,
    pub kind: DiscoveryKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvestigationResult {
    pub attempt_id: InvestigationAttemptId,
    pub outcome: InvestigationOutcome,
    pub limit: InvestigationLimit,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlaceEntityInput {
    pub name: String,
    pub description: String,
    pub property: Vec<PropertyInput>,
    pub r#trait: Vec<TraitInput>,
}

impl PlaceEntityInput {
    fn normalize(self) -> Result<Self, WorldError> {
        let (name, description) =
            normalize_entity_text(self.name, self.description, |field, reason| {
                WorldError::InvalidPlace { field, reason }
            })?;
        Ok(Self {
            name,
            description,
            property: normalize_property_input(self.property, PropertyField::Property)?,
            r#trait: normalize_trait_input(self.r#trait)?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DirectPositionInput {
    pub x_cm: i64,
    pub y_cm: i64,
    pub z_cm: i64,
    pub description: Option<String>,
}

impl DirectPositionInput {
    fn normalize(self) -> Result<Self, WorldError> {
        for (field, coordinate) in [
            (PositionField::XCm, self.x_cm),
            (PositionField::YCm, self.y_cm),
            (PositionField::ZCm, self.z_cm),
        ] {
            if !(-MAX_COORDINATE_CM..=MAX_COORDINATE_CM).contains(&coordinate) {
                return Err(WorldError::InvalidPosition {
                    field,
                    reason: InvalidReason::OutOfRange,
                });
            }
        }
        let description = normalize_position_text(self.description)?;
        Ok(Self {
            x_cm: self.x_cm,
            y_cm: self.y_cm,
            z_cm: self.z_cm,
            description,
        })
    }
}

fn normalize_position_text(value: Option<String>) -> Result<Option<String>, WorldError> {
    value
        .map(|value| {
            let value = value.trim().to_owned();
            let reason = if value.is_empty() {
                Some(InvalidReason::Empty)
            } else if value.contains('\0') {
                Some(InvalidReason::ContainsNul)
            } else if value.chars().count() > MAX_POSITION_DESCRIPTION_LENGTH {
                Some(InvalidReason::TooLong)
            } else {
                None
            };
            if let Some(reason) = reason {
                return Err(WorldError::InvalidPosition {
                    field: PositionField::Description,
                    reason,
                });
            }
            Ok(value)
        })
        .transpose()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConnectionPointInput {
    pub x_cm: i64,
    pub y_cm: i64,
    pub z_cm: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConnectionInput {
    pub name: String,
    pub description: String,
    pub shape_description: Option<String>,
    pub allows_reverse: bool,
    pub course: Vec<ConnectionPointInput>,
}

impl ConnectionInput {
    fn normalize(self) -> Result<Self, WorldError> {
        let name = normalize_connection_text(self.name, ConnectionField::Name, 120)?;
        let description =
            normalize_connection_text(self.description, ConnectionField::Description, 4_000)?;
        let shape_description = self
            .shape_description
            .map(|value| normalize_connection_text(value, ConnectionField::ShapeDescription, 4_000))
            .transpose()?;
        if !(self.course.is_empty()
            || (2..=MAX_CONNECTION_COURSE_POINT_COUNT).contains(&self.course.len()))
        {
            return Err(invalid_connection(
                ConnectionField::Course,
                InvalidReason::OutOfRange,
            ));
        }
        for point in &self.course {
            if [point.x_cm, point.y_cm, point.z_cm]
                .into_iter()
                .any(|coordinate| !(-MAX_COORDINATE_CM..=MAX_COORDINATE_CM).contains(&coordinate))
            {
                return Err(invalid_connection(
                    ConnectionField::Course,
                    InvalidReason::OutOfRange,
                ));
            }
        }
        Ok(Self {
            name,
            description,
            shape_description,
            allows_reverse: self.allows_reverse,
            course: self.course,
        })
    }
}

fn normalize_connection_text(
    value: String,
    field: ConnectionField,
    maximum: usize,
) -> Result<String, WorldError> {
    let value = value.trim().to_owned();
    let reason = if value.is_empty() {
        Some(InvalidReason::Empty)
    } else if value.contains('\0') {
        Some(InvalidReason::ContainsNul)
    } else if value.chars().count() > maximum {
        Some(InvalidReason::TooLong)
    } else {
        None
    };
    if let Some(reason) = reason {
        return Err(invalid_connection(field, reason));
    }
    Ok(value)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiscoveryOriginInput {
    AttemptPlace,
    New {
        entity: PlaceEntityInput,
        position_description: Option<String>,
    },
    Existing {
        place_id: EntityId,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiscoveryDestinationInput {
    New {
        entity: PlaceEntityInput,
        position: DirectPositionInput,
    },
    Existing {
        place_id: EntityId,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(clippy::large_enum_variant)]
pub enum DiscoveryResultInput {
    EntityAtPosition {
        name: String,
        description: String,
        position_description: Option<String>,
        property: Vec<PropertyInput>,
        r#trait: Vec<TraitInput>,
    },
    ConnectedPlace {
        origin: DiscoveryOriginInput,
        destination: DiscoveryDestinationInput,
        connection: ConnectionInput,
    },
}

impl DiscoveryResultInput {
    pub(super) fn kind(&self) -> DiscoveryKind {
        match self {
            Self::EntityAtPosition { .. } => DiscoveryKind::EntityAtPosition,
            Self::ConnectedPlace { .. } => DiscoveryKind::ConnectedPlace,
        }
    }

    fn normalize(self) -> Result<Self, WorldError> {
        match self {
            Self::EntityAtPosition {
                name,
                description,
                position_description,
                property,
                r#trait,
            } => {
                let (name, description) =
                    normalize_entity_text(name, description, |field, reason| {
                        WorldError::InvalidEntity { field, reason }
                    })?;
                Ok(Self::EntityAtPosition {
                    name,
                    description,
                    position_description: normalize_position_text(position_description)?,
                    property: normalize_property_input(property, PropertyField::Property)?,
                    r#trait: normalize_trait_input(r#trait)?,
                })
            }
            Self::ConnectedPlace {
                origin,
                destination,
                connection,
            } => {
                let origin = match origin {
                    DiscoveryOriginInput::AttemptPlace => DiscoveryOriginInput::AttemptPlace,
                    DiscoveryOriginInput::New {
                        entity,
                        position_description,
                    } => DiscoveryOriginInput::New {
                        entity: entity.normalize()?,
                        position_description: normalize_position_text(position_description)?,
                    },
                    DiscoveryOriginInput::Existing { place_id } => {
                        DiscoveryOriginInput::Existing { place_id }
                    }
                };
                let destination = match destination {
                    DiscoveryDestinationInput::New { entity, position } => {
                        DiscoveryDestinationInput::New {
                            entity: entity.normalize()?,
                            position: position.normalize()?,
                        }
                    }
                    DiscoveryDestinationInput::Existing { place_id } => {
                        DiscoveryDestinationInput::Existing { place_id }
                    }
                };
                Ok(Self::ConnectedPlace {
                    origin,
                    destination,
                    connection: connection.normalize()?,
                })
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubmitDiscovery {
    pub request_id: Uuid,
    pub attempt_id: InvestigationAttemptId,
    pub prose: String,
    pub result: DiscoveryResultInput,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(clippy::large_enum_variant)]
pub enum AcceptedDiscovery {
    EntityAtPosition {
        activity: Activity,
        entity: Entity,
        position: Position,
        place: Option<Place>,
    },
    ConnectedPlace {
        activity: Activity,
        origin: Place,
        destination: Place,
        connection: Connection,
        character: Character,
    },
}

pub(super) struct NormalizedSubmitDiscovery {
    pub(super) request_id: Uuid,
    pub(super) attempt_id: InvestigationAttemptId,
    pub(super) prose: String,
    pub(super) result: DiscoveryResultInput,
}

impl SubmitDiscovery {
    pub(super) fn normalize(self) -> Result<NormalizedSubmitDiscovery, WorldError> {
        let prose = self.prose.trim().to_owned();
        let reason = if prose.is_empty() {
            Some(InvalidReason::Empty)
        } else if prose.contains('\0') {
            Some(InvalidReason::ContainsNul)
        } else if prose.chars().count() > MAX_DISCOVERY_PROSE_LENGTH {
            Some(InvalidReason::TooLong)
        } else {
            None
        };
        if let Some(reason) = reason {
            return Err(WorldError::InvalidDiscovery {
                field: DiscoveryField::Prose,
                reason,
            });
        }
        Ok(NormalizedSubmitDiscovery {
            request_id: self.request_id,
            attempt_id: self.attempt_id,
            prose,
            result: self.result.normalize()?,
        })
    }
}

pub(super) fn discovery_fingerprint(input: &NormalizedSubmitDiscovery) -> Vec<u8> {
    let mut hash = Sha256::new();
    match &input.result {
        DiscoveryResultInput::EntityAtPosition {
            name,
            description,
            position_description,
            property,
            r#trait,
        } => {
            for field in [
                b"aicadia-submit-discovery-fingerprint-v1".as_slice(),
                input.attempt_id.0.as_bytes(),
                input.prose.as_bytes(),
                name.as_bytes(),
                description.as_bytes(),
            ] {
                fingerprint_field(&mut hash, field);
            }
            fingerprint_field(&mut hash, b"property");
            if let Some(position_description) = position_description {
                fingerprint_field(&mut hash, b"position_description_v1");
                fingerprint_field(&mut hash, position_description.as_bytes());
            }
            fingerprint_field(&mut hash, &(property.len() as u64).to_be_bytes());
            fingerprint_property_input(&mut hash, property);
            fingerprint_field(&mut hash, b"trait");
            fingerprint_field(&mut hash, &(r#trait.len() as u64).to_be_bytes());
            fingerprint_trait_input(&mut hash, r#trait);
        }
        DiscoveryResultInput::ConnectedPlace {
            origin,
            destination,
            connection,
        } => {
            for field in [
                b"aicadia-submit-discovery-fingerprint-v2".as_slice(),
                input.attempt_id.0.as_bytes(),
                input.prose.as_bytes(),
            ] {
                fingerprint_field(&mut hash, field);
            }
            fingerprint_field(&mut hash, b"connected_place");
            fingerprint_origin(&mut hash, origin);
            fingerprint_destination(&mut hash, destination);
            fingerprint_field(&mut hash, connection.name.as_bytes());
            fingerprint_field(&mut hash, connection.description.as_bytes());
            fingerprint_optional_text(&mut hash, connection.shape_description.as_deref());
            fingerprint_field(&mut hash, &[u8::from(connection.allows_reverse)]);
            fingerprint_field(&mut hash, &(connection.course.len() as u64).to_be_bytes());
            for point in &connection.course {
                fingerprint_field(&mut hash, &point.x_cm.to_be_bytes());
                fingerprint_field(&mut hash, &point.y_cm.to_be_bytes());
                fingerprint_field(&mut hash, &point.z_cm.to_be_bytes());
            }
        }
    }
    hash.finalize().to_vec()
}

fn fingerprint_origin(hash: &mut Sha256, origin: &DiscoveryOriginInput) {
    match origin {
        DiscoveryOriginInput::AttemptPlace => fingerprint_field(hash, b"attempt_place"),
        DiscoveryOriginInput::New {
            entity,
            position_description,
        } => {
            fingerprint_field(hash, b"new");
            fingerprint_place_entity(hash, entity);
            fingerprint_optional_text(hash, position_description.as_deref());
        }
        DiscoveryOriginInput::Existing { place_id } => {
            fingerprint_field(hash, b"existing");
            fingerprint_field(hash, place_id.0.as_bytes());
        }
    }
}

fn fingerprint_destination(hash: &mut Sha256, destination: &DiscoveryDestinationInput) {
    match destination {
        DiscoveryDestinationInput::New { entity, position } => {
            fingerprint_field(hash, b"new");
            fingerprint_place_entity(hash, entity);
            for coordinate in [position.x_cm, position.y_cm, position.z_cm] {
                fingerprint_field(hash, &coordinate.to_be_bytes());
            }
            fingerprint_optional_text(hash, position.description.as_deref());
        }
        DiscoveryDestinationInput::Existing { place_id } => {
            fingerprint_field(hash, b"existing");
            fingerprint_field(hash, place_id.0.as_bytes());
        }
    }
}

fn fingerprint_place_entity(hash: &mut Sha256, entity: &PlaceEntityInput) {
    fingerprint_field(hash, entity.name.as_bytes());
    fingerprint_field(hash, entity.description.as_bytes());
    fingerprint_field(hash, &(entity.property.len() as u64).to_be_bytes());
    fingerprint_property_input(hash, &entity.property);
    fingerprint_field(hash, &(entity.r#trait.len() as u64).to_be_bytes());
    fingerprint_trait_input(hash, &entity.r#trait);
}

fn fingerprint_optional_text(hash: &mut Sha256, value: Option<&str>) {
    match value {
        Some(value) => {
            fingerprint_field(hash, b"some");
            fingerprint_field(hash, value.as_bytes());
        }
        None => fingerprint_field(hash, b"none"),
    }
}
