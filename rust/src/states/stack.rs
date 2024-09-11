use egui;
use std::{cell::RefCell, rc::Rc};

use crate::game::Game;

use super::{
    contextual_input::{self, ContextualInput, Direction2D},
    raw_input::RawInput,
    state::{DisplayContext, GameState, GameStateContext},
};

pub enum AnyInput {
    Raw(RawInput),
    Contextual(ContextualInput),
}

pub struct GameStateStack {
    stack: Vec<Rc<RefCell<dyn GameState>>>,

    state_to_add: Option<Rc<RefCell<dyn GameState>>>,
}

impl GameStateStack {
    pub fn new() -> Self {
        return Self {
            stack: Vec::new(),
            state_to_add: None,
        };
    }

    pub fn push_state_outside_update(
        this: Rc<RefCell<Self>>,
        game: Rc<RefCell<Game>>,
        state: Rc<RefCell<dyn GameState>>,
    ) {
        this.borrow_mut().push_state(state);
        Self::check_for_state_to_add(this, game);
    }

    pub fn push_state(&mut self, state: Rc<RefCell<dyn GameState>>) {
        assert!(self.state_to_add.is_none());
        self.state_to_add = Some(state);
    }

    pub fn pop_state(&mut self) {
        self.stack.pop();
    }

    pub fn bottom_to_top_iter(&self) -> impl Iterator<Item = &Rc<RefCell<dyn GameState>>> {
        self.stack.iter()
    }

    pub fn top_to_bottom_iter(&self) -> impl Iterator<Item = &Rc<RefCell<dyn GameState>>> {
        self.stack.iter().rev()
    }

    fn top_state(&self) -> Rc<RefCell<dyn GameState>> {
        self.stack[self.stack.len() - 1].clone()
    }

    pub fn handle_input(this: Rc<RefCell<Self>>, game: Rc<RefCell<Game>>, input: &AnyInput) {
        let top_state = this.borrow().top_state();

        let game_state_context = GameStateContext {
            game_state: game.clone(),
            state_stack: this.clone(),
        };

        let mapped_input;
        let contextual_input;
        match input {
            AnyInput::Raw(input) => {
                mapped_input = top_state.borrow().map_raw_input(&game_state_context, input);
                contextual_input = mapped_input.as_ref();
            }
            AnyInput::Contextual(input) => {
                contextual_input = Some(input);
            }
        }

        if let Some(contextual_input) = contextual_input {
            top_state
                .borrow_mut()
                .handle_input(&game_state_context, contextual_input);
        }

        Self::check_for_state_to_add(this, game);
    }

    fn check_for_state_to_add(this: Rc<RefCell<Self>>, game: Rc<RefCell<Game>>) {
        let mut borrowed_this = this.borrow_mut();
        let state_to_add = borrowed_this.state_to_add.take();
        if let Some(state_to_add) = state_to_add {
            borrowed_this.stack.push(state_to_add);
            drop(borrowed_this);
            Self::handle_input(
                this,
                game,
                &AnyInput::Contextual(ContextualInput::StateAdded),
            );
        }
    }

    pub fn update_gui(
        this: Rc<RefCell<Self>>,
        gui_context: &egui::Context,
        game: Rc<RefCell<Game>>,
    ) {
        gui_context.input(|input| {
            for event in input.events.iter() {
                match event {
                    egui::Event::Key {
                        key,
                        physical_key,
                        pressed,
                        repeat,
                        modifiers,
                    } => {
                        if *pressed && !*repeat {
                            if *key == egui::Key::W {
                                Self::handle_input(
                                    this.clone(),
                                    game.clone(),
                                    &AnyInput::Contextual(ContextualInput::MenuMove(
                                        Direction2D::Up,
                                    )),
                                );
                            }
                            if *key == egui::Key::A {
                                Self::handle_input(
                                    this.clone(),
                                    game.clone(),
                                    &AnyInput::Contextual(ContextualInput::MenuMove(
                                        Direction2D::Left,
                                    )),
                                );
                            }
                            if *key == egui::Key::S {
                                Self::handle_input(
                                    this.clone(),
                                    game.clone(),
                                    &AnyInput::Contextual(ContextualInput::MenuMove(
                                        Direction2D::Down,
                                    )),
                                );
                            }
                            if *key == egui::Key::D {
                                Self::handle_input(
                                    this.clone(),
                                    game.clone(),
                                    &AnyInput::Contextual(ContextualInput::MenuMove(
                                        Direction2D::Right,
                                    )),
                                );
                            }
                        }
                    }
                    _ => {}
                }
            }
        });

        this.borrow().display(gui_context, game);
    }

    pub fn display(&self, gui_context: &egui::Context, game: Rc<RefCell<Game>>) {
        let mut states_to_display = Vec::new();
        for state in self.top_to_bottom_iter() {
            states_to_display.push(state.clone());
            let display_instructions = state.borrow().get_display_instructions();
            if !display_instructions.draw_lower_layers {
                break;
            }
        }
        let game = game.borrow();
        for state in states_to_display.iter().rev() {
            state.borrow_mut().display(DisplayContext {
                gui_context,
                game_state: &game,
            });
        }
    }
}
