use super::party_member::{PartyMember, PartyMemberId};

pub struct Party {
    members: Vec<PartyMember>,
}

impl Party {
    pub fn new() -> Self {
        Self {
            members: Default::default(),
        }
    }

    pub fn add_member(&mut self, member: PartyMember) {
        self.members.push(member);
    }

    pub fn ids(&self) -> Vec<PartyMemberId> {
        self.members
            .iter()
            .map(|member| member.id().clone())
            .collect()
    }
}
