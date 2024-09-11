use std::{cell::RefCell, rc::Rc};

use crate::{ids::IdGenerator, mechanics::party::party::Party};

pub struct Game {
    pub id_generator: IdGenerator,
    pub party: Rc<RefCell<Party>>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            id_generator: IdGenerator::new(),
            party: Rc::new(RefCell::new(Party::new())),
        }
    }
}
