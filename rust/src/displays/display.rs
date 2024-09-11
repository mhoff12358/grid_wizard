use crate::{game::Game, states::stack::GameStateStack};

pub fn display_game(game: &Game, state_stack: &GameStateStack) {
    for state in state_stack.top_to_bottom_iter() {}
}
