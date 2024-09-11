use std::{cell::RefCell, collections::HashSet, rc::Rc};

use crate::{
    game::Game,
    mechanics::party::party_member::PartyMemberId,
    states::{
        contextual_input::ContextualInput,
        raw_input::RawInput,
        state::{DisplayContext, DisplayInstructions, GameState, GameStateContext},
    },
};

pub struct CombatStateRoot {}

impl CombatStateRoot {
    pub fn new() -> Self {
        Self {}
    }
}

impl GameState for CombatStateRoot {
    fn handle_input(&mut self, context: &GameStateContext, contextual_input: &ContextualInput) {}

    fn get_display_instructions(&self) -> DisplayInstructions {
        DisplayInstructions {
            draw_lower_layers: false,
        }
    }

    fn display(&self, context: DisplayContext) {}
}

pub struct PickMovesState {
    characters_to_pick: Vec<PartyMemberId>,
    current_picking_character: Option<usize>,

    picked_characters: HashSet<PartyMemberId>,
}

impl PickMovesState {
    pub fn new(game_state: Rc<Game>) -> Self {
        return Self {
            characters_to_pick: game_state.party.borrow().ids(),
            current_picking_character: None,
            picked_characters: Default::default(),
        };
    }

    pub fn make_character_move_picker(
        &mut self,
        game_state: Rc<RefCell<Game>>,
    ) -> PickCharacterMoveState {
        let next_picking_character;
        if let Some(current) = self.current_picking_character.clone() {
            next_picking_character = (current + 1) % self.characters_to_pick.len();
        } else {
            next_picking_character = 0;
        }
        self.current_picking_character = Some(next_picking_character);
        return PickCharacterMoveState::new(
            game_state,
            self.characters_to_pick[next_picking_character].clone(),
        );
    }
}

impl GameState for PickMovesState {
    fn handle_input(&mut self, context: &GameStateContext, contextual_input: &ContextualInput) {
        match contextual_input {
            ContextualInput::StateAdded => {
                let new_state = self.make_character_move_picker(context.game_state.clone());
                context
                    .state_stack
                    .borrow_mut()
                    .push_state(Rc::new(RefCell::new(new_state)));
            }
            _ => {}
        }
    }

    fn display(&self, context: DisplayContext) {}
}

pub struct PickCharacterMoveState {
    picking_character: PartyMemberId,
}

impl PickCharacterMoveState {
    pub fn new(game_state: Rc<RefCell<Game>>, party_member_id: PartyMemberId) -> Self {
        return Self {
            picking_character: party_member_id,
        };
    }
}

impl GameState for PickCharacterMoveState {
    fn handle_input(&mut self, context: &GameStateContext, contextual_input: &ContextualInput) {}

    fn display(&self, context: DisplayContext) {}
}
