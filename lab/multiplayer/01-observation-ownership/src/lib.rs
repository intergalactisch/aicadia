//! Experimental, dependency-free state fixture for observation ownership.
//!
//! This crate deliberately simulates the World, host, delivery and Agent seams. It
//! is retained lab evidence, not production code or a production ordering design.

pub const BUFFER_LIMIT: usize = 3;
pub const KNOWLEDGE_LIMIT: usize = 6;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlaceId {
    OldQuarry,
    QuietGrove,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CharacterId {
    Mara,
    Ivo,
    Nia,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StoneState {
    Standing,
    Fallen,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Operation {
    SubmitAction,
    SubmitInteraction,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Access {
    PublicLocal,
    ParticipantsExactPlace,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HintMode {
    Normal,
    Duplicate,
    Lost,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContextSource {
    ActiveContext,
    PlaceHistory,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Command {
    FetchBaseline,
    Subscribe,
    Unsubscribe,
    Disconnect,
    Reconnect,
    SetHintMode(HintMode),
    SubmitPublicStoneAction,
    SubmitPrivateInteraction,
    Refetch,
    ReadPublicHistory,
    ExplicitUserTurn,
    SwitchCharacter,
    MoveActiveCharacter,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Character {
    pub id: CharacterId,
    pub place_id: PlaceId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Stone {
    pub place_id: PlaceId,
    pub state: StoneState,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Activity {
    pub id: u64,
    /// Fixture convenience only; not a production cursor or ordering proposal.
    pub fixture_sequence: u64,
    pub operation: Operation,
    pub actor_character_id: CharacterId,
    pub place_id: PlaceId,
    pub participant_character_ids: Vec<CharacterId>,
    pub summary: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct World {
    pub characters: [Character; 3],
    pub stone: Stone,
    pub activities: Vec<Activity>,
    pub next_fixture_sequence: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Baseline {
    pub character_id: CharacterId,
    pub place_id: PlaceId,
    pub after_fixture_sequence: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Subscription {
    pub character_id: CharacterId,
    pub place_id: PlaceId,
    pub after_fixture_sequence: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hint {
    pub place_id: PlaceId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContextItem {
    pub activity_id: u64,
    pub character_id: CharacterId,
    pub place_id: PlaceId,
    pub source: ContextSource,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Delivery {
    pub attempts: usize,
    pub coalesced: usize,
    pub lost: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Host {
    pub connected: bool,
    pub active_character_id: CharacterId,
    pub baseline: Option<Baseline>,
    pub subscription: Option<Subscription>,
    pub hints: Vec<Hint>,
    pub buffer: Vec<ContextItem>,
    pub next_hint_mode: HintMode,
    pub delivery: Delivery,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KnowledgeItem {
    pub activity_id: u64,
    pub character_id: CharacterId,
    pub source: ContextSource,
    pub presentation: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Agent {
    pub explicit_invocations: usize,
    pub knowledge: Vec<KnowledgeItem>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservationLab {
    pub world: World,
    pub host: Host,
    pub agent: Agent,
}

impl Default for ObservationLab {
    fn default() -> Self {
        Self {
            world: World {
                characters: [
                    Character {
                        id: CharacterId::Mara,
                        place_id: PlaceId::OldQuarry,
                    },
                    Character {
                        id: CharacterId::Ivo,
                        place_id: PlaceId::OldQuarry,
                    },
                    Character {
                        id: CharacterId::Nia,
                        place_id: PlaceId::OldQuarry,
                    },
                ],
                stone: Stone {
                    place_id: PlaceId::OldQuarry,
                    state: StoneState::Standing,
                },
                activities: Vec::new(),
                next_fixture_sequence: 1,
            },
            host: Host {
                connected: true,
                active_character_id: CharacterId::Ivo,
                baseline: None,
                subscription: None,
                hints: Vec::new(),
                buffer: Vec::new(),
                next_hint_mode: HintMode::Normal,
                delivery: Delivery::default(),
            },
            agent: Agent::default(),
        }
    }
}

impl ObservationLab {
    pub fn apply(&mut self, command: Command) {
        match command {
            Command::FetchBaseline => self.fetch_baseline(),
            Command::Subscribe => self.subscribe(),
            Command::Unsubscribe => self.end_attention(),
            Command::Disconnect => {
                self.host.connected = false;
                self.end_attention();
            }
            Command::Reconnect => self.host.connected = true,
            Command::SetHintMode(mode) => self.host.next_hint_mode = mode,
            Command::SubmitPublicStoneAction => self.submit_public_stone_action(),
            Command::SubmitPrivateInteraction => self.submit_private_interaction(),
            Command::Refetch => self.refetch(),
            Command::ReadPublicHistory => self.read_public_history(),
            Command::ExplicitUserTurn => self.explicit_user_turn(),
            Command::SwitchCharacter => self.switch_character(),
            Command::MoveActiveCharacter => self.move_active_character(),
        }
    }

    pub fn character(&self, id: CharacterId) -> &Character {
        self.world
            .characters
            .iter()
            .find(|character| character.id == id)
            .expect("fixed fixture character must exist")
    }

    fn character_mut(&mut self, id: CharacterId) -> &mut Character {
        self.world
            .characters
            .iter_mut()
            .find(|character| character.id == id)
            .expect("fixed fixture character must exist")
    }

    fn active_character(&self) -> &Character {
        self.character(self.host.active_character_id)
    }

    fn latest_fixture_sequence(&self) -> u64 {
        self.world.next_fixture_sequence - 1
    }

    fn fetch_baseline(&mut self) {
        if !self.host.connected {
            return;
        }

        let character = self.active_character();
        self.host.baseline = Some(Baseline {
            character_id: character.id,
            place_id: character.place_id,
            after_fixture_sequence: self.latest_fixture_sequence(),
        });
    }

    fn subscribe(&mut self) {
        if !self.host.connected {
            return;
        }

        let character = self.active_character();
        let Some(baseline) = &self.host.baseline else {
            return;
        };
        if baseline.character_id != character.id || baseline.place_id != character.place_id {
            return;
        }

        self.host.subscription = Some(Subscription {
            character_id: character.id,
            place_id: character.place_id,
            after_fixture_sequence: baseline.after_fixture_sequence,
        });
    }

    fn end_attention(&mut self) {
        self.host.subscription = None;
        self.host.baseline = None;
        self.host.hints.clear();
    }

    fn submit_public_stone_action(&mut self) {
        if self.world.stone.state != StoneState::Standing {
            return;
        }

        self.world.stone.state = StoneState::Fallen;
        self.record_activity(
            Operation::SubmitAction,
            CharacterId::Mara,
            PlaceId::OldQuarry,
            Vec::new(),
            "Mara dropped the Great Stone in the Old Quarry.",
        );
    }

    fn submit_private_interaction(&mut self) {
        if self
            .world
            .activities
            .iter()
            .any(|activity| activity.operation == Operation::SubmitInteraction)
        {
            return;
        }

        self.record_activity(
            Operation::SubmitInteraction,
            CharacterId::Mara,
            PlaceId::OldQuarry,
            vec![CharacterId::Mara, CharacterId::Ivo],
            "Mara quietly warned Ivo about loose rock.",
        );
    }

    fn record_activity(
        &mut self,
        operation: Operation,
        actor_character_id: CharacterId,
        place_id: PlaceId,
        participant_character_ids: Vec<CharacterId>,
        summary: &'static str,
    ) {
        let fixture_sequence = self.world.next_fixture_sequence;
        self.world.next_fixture_sequence += 1;
        self.world.activities.push(Activity {
            id: fixture_sequence,
            fixture_sequence,
            operation,
            actor_character_id,
            place_id,
            participant_character_ids,
            summary,
        });
        self.route_hint(fixture_sequence);
    }

    fn route_hint(&mut self, activity_id: u64) {
        let activity = self
            .world
            .activities
            .iter()
            .find(|activity| activity.id == activity_id)
            .expect("newly recorded fixture Activity must exist")
            .clone();
        if !self.host_is_attentive(&activity)
            || !self.world_allows(self.host.active_character_id, &activity)
        {
            return;
        }

        let attempts = match self.host.next_hint_mode {
            HintMode::Duplicate => 2,
            HintMode::Normal | HintMode::Lost => 1,
        };
        self.host.delivery.attempts += attempts;

        if self.host.next_hint_mode == HintMode::Lost {
            self.host.delivery.lost += 1;
            self.host.next_hint_mode = HintMode::Normal;
            return;
        }

        let already_dirty = self
            .host
            .hints
            .iter()
            .any(|hint| hint.place_id == activity.place_id);
        if !already_dirty {
            self.host.hints.push(Hint {
                place_id: activity.place_id,
            });
        }
        self.host.delivery.coalesced += attempts - usize::from(!already_dirty);
        self.host.next_hint_mode = HintMode::Normal;
    }

    fn host_is_attentive(&self, activity: &Activity) -> bool {
        let Some(subscription) = &self.host.subscription else {
            return false;
        };
        self.host.connected
            && subscription.character_id == self.host.active_character_id
            && subscription.place_id == activity.place_id
            && self.active_character().place_id == subscription.place_id
    }

    fn world_allows(&self, character_id: CharacterId, activity: &Activity) -> bool {
        if self.character(character_id).place_id != activity.place_id {
            return false;
        }

        match operation_access(activity.operation) {
            Access::PublicLocal => true,
            Access::ParticipantsExactPlace => {
                activity.participant_character_ids.contains(&character_id)
            }
        }
    }

    fn refetch(&mut self) {
        if !self.host.connected {
            return;
        }
        let Some(subscription) = self.host.subscription.clone() else {
            return;
        };

        let activities: Vec<_> = self
            .world
            .activities
            .iter()
            .filter(|activity| {
                activity.fixture_sequence > subscription.after_fixture_sequence
                    && activity.place_id == subscription.place_id
                    && self.world_allows(subscription.character_id, activity)
            })
            .cloned()
            .collect();
        for activity in &activities {
            self.append_buffer(activity, ContextSource::ActiveContext);
        }

        let latest = self.latest_fixture_sequence();
        if let Some(active_subscription) = &mut self.host.subscription {
            active_subscription.after_fixture_sequence = latest;
        }
        self.host
            .hints
            .retain(|hint| hint.place_id != subscription.place_id);
    }

    fn read_public_history(&mut self) {
        if !self.host.connected {
            return;
        }

        let character_id = self.host.active_character_id;
        let character_place = self.character(character_id).place_id;
        let mut activities: Vec<_> = self
            .world
            .activities
            .iter()
            .filter(|activity| {
                activity.place_id == character_place
                    && operation_access(activity.operation) == Access::PublicLocal
                    && self.world_allows(character_id, activity)
            })
            .cloned()
            .collect();
        let first = activities.len().saturating_sub(BUFFER_LIMIT);
        for activity in activities.drain(first..) {
            self.append_buffer(&activity, ContextSource::PlaceHistory);
        }
    }

    fn append_buffer(&mut self, activity: &Activity, source: ContextSource) {
        let character_id = self.host.active_character_id;
        if self
            .host
            .buffer
            .iter()
            .any(|item| item.activity_id == activity.id && item.character_id == character_id)
        {
            return;
        }

        self.host.buffer.push(ContextItem {
            activity_id: activity.id,
            character_id,
            place_id: activity.place_id,
            source,
        });
        let excess = self.host.buffer.len().saturating_sub(BUFFER_LIMIT);
        if excess > 0 {
            self.host.buffer.drain(..excess);
        }
    }

    fn explicit_user_turn(&mut self) {
        if !self.host.connected {
            return;
        }

        let character_id = self.host.active_character_id;
        let selected: Vec<_> = self
            .host
            .buffer
            .iter()
            .filter(|item| item.character_id == character_id)
            .cloned()
            .collect();
        self.agent.explicit_invocations += 1;
        for context in selected {
            let activity = self
                .world
                .activities
                .iter()
                .find(|activity| activity.id == context.activity_id)
                .expect("buffered fixture Activity must exist");
            self.agent.knowledge.push(KnowledgeItem {
                activity_id: activity.id,
                character_id,
                source: context.source,
                presentation: describe_for_agent(activity, context.source),
            });
        }
        let excess = self.agent.knowledge.len().saturating_sub(KNOWLEDGE_LIMIT);
        if excess > 0 {
            self.agent.knowledge.drain(..excess);
        }
        self.host
            .buffer
            .retain(|item| item.character_id != character_id);
    }

    fn switch_character(&mut self) {
        self.host.active_character_id = match self.host.active_character_id {
            CharacterId::Ivo => CharacterId::Nia,
            CharacterId::Nia | CharacterId::Mara => CharacterId::Ivo,
        };
        self.end_attention();
    }

    fn move_active_character(&mut self) {
        let character_id = self.host.active_character_id;
        let character = self.character_mut(character_id);
        character.place_id = match character.place_id {
            PlaceId::OldQuarry => PlaceId::QuietGrove,
            PlaceId::QuietGrove => PlaceId::OldQuarry,
        };
        self.end_attention();
    }
}

fn operation_access(operation: Operation) -> Access {
    match operation {
        Operation::SubmitAction => Access::PublicLocal,
        Operation::SubmitInteraction => Access::ParticipantsExactPlace,
    }
}

fn describe_for_agent(activity: &Activity, source: ContextSource) -> String {
    match (activity.operation, source) {
        (_, ContextSource::PlaceHistory) => format!(
            "Learned as public Place history: {} Do not claim personal sight or hearing.",
            activity.summary
        ),
        (Operation::SubmitAction, ContextSource::ActiveContext) => format!(
            "Available from active local context: {} Natural sensory wording creates no mechanic.",
            activity.summary
        ),
        (Operation::SubmitInteraction, ContextSource::ActiveContext) => format!(
            "Available because this Character is an Interaction participant: {}",
            activity.summary
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(commands: &[Command]) -> ObservationLab {
        let mut lab = ObservationLab::default();
        for command in commands {
            lab.apply(*command);
        }
        lab
    }

    #[test]
    fn live_public_occurrence_reaches_agent_only_on_explicit_user_turn() {
        let mut lab = run(&[
            Command::FetchBaseline,
            Command::Subscribe,
            Command::SubmitPublicStoneAction,
            Command::Refetch,
        ]);

        assert_eq!(lab.world.activities.len(), 1, "{lab:#?}");
        assert_eq!(lab.host.buffer.len(), 1, "{lab:#?}");
        assert_eq!(lab.agent.explicit_invocations, 0, "{lab:#?}");
        assert!(lab.agent.knowledge.is_empty(), "{lab:#?}");

        lab.apply(Command::ExplicitUserTurn);

        assert_eq!(lab.world.activities.len(), 1, "{lab:#?}");
        assert_eq!(lab.agent.explicit_invocations, 1, "{lab:#?}");
        assert!(
            lab.agent.knowledge.iter().any(|item| {
                item.activity_id == 1
                    && item.character_id == CharacterId::Ivo
                    && item.source == ContextSource::ActiveContext
            }),
            "{lab:#?}"
        );
    }

    #[test]
    fn duplicate_delivery_attempts_coalesce_before_authoritative_refetch() {
        let lab = run(&[
            Command::FetchBaseline,
            Command::Subscribe,
            Command::SetHintMode(HintMode::Duplicate),
            Command::SubmitPublicStoneAction,
            Command::Refetch,
            Command::ExplicitUserTurn,
        ]);

        assert_eq!(lab.host.delivery.attempts, 2, "{lab:#?}");
        assert_eq!(lab.host.delivery.coalesced, 1, "{lab:#?}");
        assert_eq!(lab.world.activities.len(), 1, "{lab:#?}");
        assert_eq!(lab.agent.knowledge.len(), 1, "{lab:#?}");
    }

    #[test]
    fn lost_hint_does_not_prevent_deliberate_authoritative_refetch() {
        let lab = run(&[
            Command::FetchBaseline,
            Command::Subscribe,
            Command::SetHintMode(HintMode::Lost),
            Command::SubmitPublicStoneAction,
            Command::Refetch,
            Command::ExplicitUserTurn,
        ]);

        assert_eq!(lab.host.delivery.lost, 1, "{lab:#?}");
        assert!(lab.host.hints.is_empty(), "{lab:#?}");
        assert!(
            lab.agent.knowledge.iter().any(|item| item.activity_id == 1),
            "{lab:#?}"
        );
    }

    #[test]
    fn disconnected_occurrence_returns_only_as_non_personal_public_history() {
        let lab = run(&[
            Command::FetchBaseline,
            Command::Subscribe,
            Command::Disconnect,
            Command::SubmitPublicStoneAction,
            Command::Reconnect,
            Command::FetchBaseline,
            Command::Subscribe,
            Command::Refetch,
            Command::ReadPublicHistory,
            Command::ExplicitUserTurn,
        ]);

        assert_eq!(lab.host.delivery.attempts, 0, "{lab:#?}");
        let knowledge = lab
            .agent
            .knowledge
            .iter()
            .find(|item| item.activity_id == 1)
            .expect("public history should reach the explicit User turn");
        assert_eq!(knowledge.source, ContextSource::PlaceHistory, "{lab:#?}");
        assert!(
            knowledge.presentation.contains("Do not claim personal"),
            "{lab:#?}"
        );
    }

    #[test]
    fn later_arrival_reads_public_history_without_realtime_delivery() {
        let lab = run(&[
            Command::MoveActiveCharacter,
            Command::SubmitPublicStoneAction,
            Command::MoveActiveCharacter,
            Command::ReadPublicHistory,
            Command::ExplicitUserTurn,
        ]);

        assert_eq!(lab.world.activities.len(), 1, "{lab:#?}");
        assert_eq!(lab.host.delivery.attempts, 0, "{lab:#?}");
        assert!(
            lab.agent.knowledge.iter().any(|item| {
                item.activity_id == 1 && item.source == ContextSource::PlaceHistory
            }),
            "{lab:#?}"
        );
    }

    #[test]
    fn same_place_bystander_cannot_receive_private_interaction() {
        let lab = run(&[
            Command::SwitchCharacter,
            Command::FetchBaseline,
            Command::Subscribe,
            Command::SubmitPrivateInteraction,
            Command::Refetch,
            Command::ExplicitUserTurn,
        ]);

        assert_eq!(lab.host.active_character_id, CharacterId::Nia, "{lab:#?}");
        assert_eq!(lab.world.activities.len(), 1, "{lab:#?}");
        assert_eq!(lab.host.delivery.attempts, 0, "{lab:#?}");
        assert!(lab.agent.knowledge.is_empty(), "{lab:#?}");
    }

    #[test]
    fn character_switch_ends_attention_even_at_the_same_place() {
        let lab = run(&[
            Command::FetchBaseline,
            Command::Subscribe,
            Command::SwitchCharacter,
            Command::SubmitPublicStoneAction,
        ]);

        assert_eq!(lab.host.active_character_id, CharacterId::Nia, "{lab:#?}");
        assert!(lab.host.subscription.is_none(), "{lab:#?}");
        assert_eq!(lab.world.activities.len(), 1, "{lab:#?}");
        assert_eq!(lab.host.delivery.attempts, 0, "{lab:#?}");
    }

    #[test]
    fn place_departure_ends_attention_before_later_local_occurrence() {
        let lab = run(&[
            Command::FetchBaseline,
            Command::Subscribe,
            Command::MoveActiveCharacter,
            Command::SubmitPublicStoneAction,
        ]);

        assert!(lab.host.subscription.is_none(), "{lab:#?}");
        assert_eq!(
            lab.character(CharacterId::Ivo).place_id,
            PlaceId::QuietGrove,
            "{lab:#?}"
        );
        assert_eq!(lab.host.delivery.attempts, 0, "{lab:#?}");
    }
}
