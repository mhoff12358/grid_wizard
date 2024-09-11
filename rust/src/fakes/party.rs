use crate::{game::Game, mechanics::party::party_member::PartyMember};

pub fn fake_party(game: &mut Game) {
    let mut party = game.party.borrow_mut();
    party.add_member(PartyMember::new(&mut game.id_generator, "PM A".into()));
    party.add_member(PartyMember::new(&mut game.id_generator, "PM B".into()));
    party.add_member(PartyMember::new(&mut game.id_generator, "PM C".into()));
}
