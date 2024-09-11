use std::{cell::RefCell, rc::Rc};

use crate::{game::Game, menus::top_menu::TopMenu, states::stack::GameStateStack};

pub struct World {
    pub game: Rc<RefCell<Game>>,
    pub state_stack: Rc<RefCell<GameStateStack>>,
}

impl World {
    pub fn new() -> Self {
        let state_stack = Rc::new(RefCell::new(GameStateStack::new()));
        let game = Rc::new(RefCell::new(Game::new()));
        GameStateStack::push_state_outside_update(
            state_stack.clone(),
            game.clone(),
            Rc::new(RefCell::new(TopMenu::new())),
        );

        World { game, state_stack }
    }
}
