use egui;
use std::{cell::RefCell, rc::Rc};

use crate::game::Game;

use super::{contextual_input::ContextualInput, raw_input::RawInput, stack::GameStateStack};

pub struct GameStateContext {
    pub game_state: Rc<RefCell<Game>>,
    pub state_stack: Rc<RefCell<GameStateStack>>,
}

pub struct DisplayContext<'a> {
    pub gui_context: &'a egui::Context,
    pub game_state: &'a Game,
}

#[derive(Debug, Default)]
pub struct DisplayInstructions {
    pub draw_lower_layers: bool,
}

pub trait GameState {
    fn map_raw_input(
        &self,
        context: &GameStateContext,
        raw_input: &RawInput,
    ) -> Option<ContextualInput> {
        return None;
    }

    fn handle_input(&mut self, context: &GameStateContext, contextual_input: &ContextualInput);

    fn get_display_instructions(&self) -> DisplayInstructions {
        DisplayInstructions {
            draw_lower_layers: true,
        }
    }
    fn display(&self, context: DisplayContext);
}
