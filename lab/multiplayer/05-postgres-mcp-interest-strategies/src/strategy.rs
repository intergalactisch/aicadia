use std::fmt::{self, Display};
use std::str::FromStr;

use anyhow::{Context, bail};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::load::{
    ScaleKey, ScaleRouting, SyntheticChange, SyntheticChangeScope, SyntheticInterest,
};
use crate::world::CommittedChange;

const URI_PREFIX: &str = "aicadia://";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Strategy {
    GlobalFirehose,
    Place,
    ExactOnly,
    PlaceAndExact,
    Structural,
}

impl Strategy {
    pub const ALL: [Self; 5] = [
        Self::GlobalFirehose,
        Self::Place,
        Self::ExactOnly,
        Self::PlaceAndExact,
        Self::Structural,
    ];

    pub const fn name(self) -> &'static str {
        match self {
            Self::GlobalFirehose => "global_firehose",
            Self::Place => "place",
            Self::ExactOnly => "exact_only",
            Self::PlaceAndExact => "place_and_exact",
            Self::Structural => "structural",
        }
    }

    /// Maps the same accepted change to invalidation resources for this strategy.
    pub fn resources_for_change(self, change: &CommittedChange) -> Vec<ResourceKey> {
        let mut resources = match self {
            Self::GlobalFirehose => vec![ResourceKey::Global],
            Self::Place | Self::PlaceAndExact => change
                .affected_place_ids
                .iter()
                .copied()
                .map(ResourceKey::Place)
                .collect(),
            Self::ExactOnly => Vec::new(),
            Self::Structural => match change.scope {
                ChangeScope::Local => change
                    .affected_place_ids
                    .iter()
                    .copied()
                    .map(ResourceKey::Place)
                    .collect(),
                ChangeScope::Area { area_id } => vec![ResourceKey::Area(area_id)],
                ChangeScope::World => vec![ResourceKey::World],
            },
        };

        if matches!(
            self,
            Self::ExactOnly | Self::PlaceAndExact | Self::Structural
        ) {
            resources.extend(
                change
                    .changed_entity_ids
                    .iter()
                    .copied()
                    .map(ResourceKey::Entity),
            );
        }
        canonicalize(resources)
    }

    /// Maps a host's declared interest through the same five comparison variants.
    pub fn resources_for_interest(self, interest: &InterestSpec) -> Vec<ResourceKey> {
        let mut resources = match self {
            Self::GlobalFirehose => vec![ResourceKey::Global],
            Self::Place | Self::PlaceAndExact => interest
                .current_place_id
                .map(ResourceKey::Place)
                .into_iter()
                .collect(),
            Self::ExactOnly => Vec::new(),
            Self::Structural => {
                let mut chain = vec![ResourceKey::World];
                chain.extend(interest.current_area_id.map(ResourceKey::Area));
                chain.extend(interest.current_place_id.map(ResourceKey::Place));
                chain
            }
        };

        if matches!(
            self,
            Self::ExactOnly | Self::PlaceAndExact | Self::Structural
        ) {
            resources.extend(
                interest
                    .exact_entity_ids
                    .iter()
                    .copied()
                    .map(ResourceKey::Entity),
            );
        }
        canonicalize(resources)
    }
}

impl Display for Strategy {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

impl ScaleRouting for Strategy {
    fn strategy_name(&self) -> &'static str {
        self.name()
    }

    fn interest_keys(&self, interest: &SyntheticInterest, output: &mut Vec<ScaleKey>) {
        match self {
            Self::GlobalFirehose => output.push(ScaleKey::World),
            Self::Place => output.push(ScaleKey::Place(interest.place_id)),
            Self::ExactOnly => output.push(ScaleKey::Entity(interest.selected_entity_id)),
            Self::PlaceAndExact => {
                output.push(ScaleKey::Place(interest.place_id));
                output.push(ScaleKey::Entity(interest.selected_entity_id));
            }
            Self::Structural => {
                output.push(ScaleKey::World);
                output.push(ScaleKey::Area(interest.area_id));
                output.push(ScaleKey::Place(interest.place_id));
                output.push(ScaleKey::Entity(interest.selected_entity_id));
            }
        }
    }

    fn change_keys(&self, change: &SyntheticChange, output: &mut Vec<ScaleKey>) {
        if matches!(self, Self::GlobalFirehose) {
            output.push(ScaleKey::World);
            return;
        }

        if matches!(self, Self::Place | Self::PlaceAndExact)
            && let SyntheticChangeScope::Places {
                place_ids,
                place_count,
                ..
            } = change.scope
        {
            output.extend(
                place_ids
                    .iter()
                    .copied()
                    .take(usize::from(place_count))
                    .map(ScaleKey::Place),
            );
        }

        if matches!(self, Self::Structural) {
            match change.scope {
                SyntheticChangeScope::Places {
                    place_ids,
                    place_count,
                    ..
                } => output.extend(
                    place_ids
                        .iter()
                        .copied()
                        .take(usize::from(place_count))
                        .map(ScaleKey::Place),
                ),
                SyntheticChangeScope::Area(area_id) => output.push(ScaleKey::Area(area_id)),
                SyntheticChangeScope::World => output.push(ScaleKey::World),
            }
        }

        if matches!(
            self,
            Self::ExactOnly | Self::PlaceAndExact | Self::Structural
        ) {
            output.extend(change.entity_id.map(ScaleKey::Entity));
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(tag = "kind", content = "id", rename_all = "snake_case")]
pub enum ResourceKey {
    /// A logical bounded recent-Activity view. It has no current-state row or revision.
    Global,
    World,
    Area(Uuid),
    Place(Uuid),
    Entity(Uuid),
}

impl ResourceKey {
    pub fn uri(self) -> String {
        match self {
            Self::Global => format!("{URI_PREFIX}global"),
            Self::World => format!("{URI_PREFIX}world"),
            Self::Area(id) => format!("{URI_PREFIX}area/{id}"),
            Self::Place(id) => format!("{URI_PREFIX}place/{id}"),
            Self::Entity(id) => format!("{URI_PREFIX}entity/{id}"),
        }
    }

    pub const fn kind(self) -> &'static str {
        match self {
            Self::Global => "global",
            Self::World => "world",
            Self::Area(_) => "area",
            Self::Place(_) => "place",
            Self::Entity(_) => "entity",
        }
    }
}

impl Display for ResourceKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.uri())
    }
}

impl FromStr for ResourceKey {
    type Err = anyhow::Error;

    fn from_str(uri: &str) -> Result<Self, Self::Err> {
        let path = uri
            .strip_prefix(URI_PREFIX)
            .with_context(|| format!("resource URI must start with {URI_PREFIX}"))?;
        match path.split('/').collect::<Vec<_>>().as_slice() {
            ["global"] => Ok(Self::Global),
            ["world"] => Ok(Self::World),
            ["area", id] => Ok(Self::Area(parse_id("area", id)?)),
            ["place", id] => Ok(Self::Place(parse_id("place", id)?)),
            ["entity", id] => Ok(Self::Entity(parse_id("entity", id)?)),
            _ => bail!("unsupported resource URI: {uri}"),
        }
    }
}

fn parse_id(kind: &str, id: &str) -> anyhow::Result<Uuid> {
    Uuid::parse_str(id).with_context(|| format!("invalid {kind} resource id: {id}"))
}

fn canonicalize(mut resources: Vec<ResourceKey>) -> Vec<ResourceKey> {
    resources.sort_unstable();
    resources.dedup();
    resources
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ChangeScope {
    Local,
    Area { area_id: Uuid },
    World,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct InterestSpec {
    pub current_area_id: Option<Uuid>,
    pub current_place_id: Option<Uuid>,
    pub exact_entity_ids: Vec<Uuid>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::CommittedChange;

    const AREA: Uuid = Uuid::from_u128(1);
    const PLACE_A: Uuid = Uuid::from_u128(2);
    const PLACE_B: Uuid = Uuid::from_u128(3);
    const TREE: Uuid = Uuid::from_u128(4);

    fn change(scope: ChangeScope) -> CommittedChange {
        CommittedChange {
            change_id: Uuid::from_u128(10),
            scope,
            primary_entity_id: Some(TREE),
            primary_place_id: Some(PLACE_A),
            affected_place_ids: vec![PLACE_A, PLACE_B],
            changed_entity_ids: vec![TREE],
            resource_versions: Vec::new(),
        }
    }

    #[test]
    fn five_strategies_map_one_fixture_without_hidden_scope_inference() {
        assert_eq!(
            Strategy::GlobalFirehose.resources_for_change(&change(ChangeScope::World)),
            vec![ResourceKey::Global]
        );
        assert_eq!(
            Strategy::Place.resources_for_change(&change(ChangeScope::World)),
            vec![ResourceKey::Place(PLACE_A), ResourceKey::Place(PLACE_B)]
        );
        assert_eq!(
            Strategy::ExactOnly.resources_for_change(&change(ChangeScope::World)),
            vec![ResourceKey::Entity(TREE)]
        );
        assert_eq!(
            Strategy::PlaceAndExact.resources_for_change(&change(ChangeScope::World)),
            vec![
                ResourceKey::Place(PLACE_A),
                ResourceKey::Place(PLACE_B),
                ResourceKey::Entity(TREE),
            ]
        );
        assert_eq!(
            Strategy::Structural.resources_for_change(&change(ChangeScope::World)),
            vec![ResourceKey::World, ResourceKey::Entity(TREE)]
        );
    }

    #[test]
    fn structural_interest_contains_scope_chain_and_exact_focus() {
        let interest = InterestSpec {
            current_area_id: Some(AREA),
            current_place_id: Some(PLACE_A),
            exact_entity_ids: vec![TREE],
        };
        assert_eq!(
            Strategy::Structural.resources_for_interest(&interest),
            vec![
                ResourceKey::World,
                ResourceKey::Area(AREA),
                ResourceKey::Place(PLACE_A),
                ResourceKey::Entity(TREE),
            ]
        );
    }

    #[test]
    fn resource_uris_round_trip() {
        for key in [
            ResourceKey::Global,
            ResourceKey::World,
            ResourceKey::Area(AREA),
            ResourceKey::Place(PLACE_A),
            ResourceKey::Entity(TREE),
        ] {
            assert_eq!(key.uri().parse::<ResourceKey>().unwrap(), key);
        }
    }
}
