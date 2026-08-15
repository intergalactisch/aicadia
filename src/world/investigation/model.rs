use super::super::*;

const MAX_DISCOVERY_PROSE_LENGTH: usize = 4_000;

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
    EntityAtCurrentPlace,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvestigationLimits {
    pub result_count: u8,
    pub kind: DiscoveryKind,
}

impl InvestigationLimits {
    pub(super) const CURRENT: Self = Self {
        result_count: 1,
        kind: DiscoveryKind::EntityAtCurrentPlace,
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StartInvestigation {
    pub request_id: Uuid,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvestigationResult {
    pub attempt_id: InvestigationAttemptId,
    pub outcome: InvestigationOutcome,
    pub limits: InvestigationLimits,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiscoveryFind {
    pub name: String,
    pub description: String,
    pub property: Vec<PropertyInput>,
    pub r#trait: Vec<TraitInput>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubmitDiscovery {
    pub request_id: Uuid,
    pub attempt_id: InvestigationAttemptId,
    pub prose: String,
    pub find: DiscoveryFind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AcceptedDiscovery {
    pub activity: Activity,
    pub entity: Entity,
    pub place: Place,
}

pub(super) struct NormalizedSubmitDiscovery {
    pub(super) request_id: Uuid,
    pub(super) attempt_id: InvestigationAttemptId,
    pub(super) prose: String,
    pub(super) find: DiscoveryFind,
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
        let (name, description) =
            normalize_entity_text(self.find.name, self.find.description, |field, reason| {
                WorldError::InvalidEntity { field, reason }
            })?;
        let property = normalize_property_input(self.find.property, PropertyField::Property)?;
        let r#trait = normalize_trait_input(self.find.r#trait)?;
        Ok(NormalizedSubmitDiscovery {
            request_id: self.request_id,
            attempt_id: self.attempt_id,
            prose,
            find: DiscoveryFind {
                name,
                description,
                property,
                r#trait,
            },
        })
    }
}

pub(super) fn discovery_fingerprint(input: &NormalizedSubmitDiscovery) -> Vec<u8> {
    let mut hash = Sha256::new();
    for field in [
        b"aicadia-submit-discovery-fingerprint-v1".as_slice(),
        input.attempt_id.0.as_bytes(),
        input.prose.as_bytes(),
        input.find.name.as_bytes(),
        input.find.description.as_bytes(),
    ] {
        fingerprint_field(&mut hash, field);
    }
    for property in &input.find.property {
        fingerprint_field(&mut hash, property.key.as_bytes());
        match &property.value {
            PropertyValue::Text(value) => {
                fingerprint_field(&mut hash, b"text");
                fingerprint_field(&mut hash, value.as_bytes());
            }
            PropertyValue::Integer(value) => {
                fingerprint_field(&mut hash, b"integer");
                fingerprint_field(&mut hash, &value.to_be_bytes());
            }
        }
    }
    for r#trait in &input.find.r#trait {
        fingerprint_field(&mut hash, r#trait.statement.as_bytes());
    }
    hash.finalize().to_vec()
}

fn fingerprint_field(hash: &mut Sha256, field: &[u8]) {
    hash.update((field.len() as u64).to_be_bytes());
    hash.update(field);
}
