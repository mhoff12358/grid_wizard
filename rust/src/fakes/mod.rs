use party::fake_party;

use crate::world::World;

pub mod party;

pub fn fake_world(world: &mut World) {
    fake_party(&mut world.game.borrow_mut());
}
