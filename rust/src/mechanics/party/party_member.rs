use crate::{
    ids::{Id, IdGenerator},
    mechanics::combat::actions::action::Action,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PartyMemberId {
    id: u64,
}

impl Id for PartyMemberId {
    fn new(value: u64) -> Self {
        PartyMemberId { id: value }
    }
}

pub struct PartyMember {
    id: PartyMemberId,
    name: String,
    actions: Vec<Action>,
}

impl PartyMember {
    pub fn new(id_generator: &mut IdGenerator, name: String) -> Self {
        Self {
            id: id_generator.generate(),
            name,
            actions: Default::default(),
        }
    }

    pub fn id(&self) -> &PartyMemberId {
        return &self.id;
    }

    pub fn add_action(&mut self, action: Action) {
        self.actions.push(action);
    }
}
