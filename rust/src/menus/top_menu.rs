use std::{cell::RefCell, rc::Rc};

use crate::{
    mechanics::combat::states::CombatStateRoot,
    states::{
        contextual_input::ContextualInput,
        state::{DisplayContext, DisplayInstructions, GameState, GameStateContext},
    },
};

use super::linear_menu::LinearMenu;

#[derive(Debug, Clone, Copy)]
enum TopMenuOption {
    Start,
    Quit,
}

impl TopMenuOption {
    pub fn text(&self) -> &'static str {
        match self {
            TopMenuOption::Start => "Start",
            TopMenuOption::Quit => "Quit",
        }
    }

    pub fn pressed(&self) {
        println!("Pressed {}", self.text());
    }
}

pub struct TopMenu {
    menu: LinearMenu<TopMenuOption>,
}

impl TopMenu {
    pub fn new() -> Self {
        Self {
            menu: LinearMenu::new(vec![TopMenuOption::Start, TopMenuOption::Quit]),
        }
    }
}

impl GameState for TopMenu {
    fn handle_input(&mut self, context: &GameStateContext, contextual_input: &ContextualInput) {
        match contextual_input {
            ContextualInput::MenuMove(direction) => {
                self.menu.move_cursor(*direction);
            }
            ContextualInput::MenuSelect => match self.menu.selected_option() {
                TopMenuOption::Start => {
                    context
                        .state_stack
                        .borrow_mut()
                        .push_state(Rc::new(RefCell::new(CombatStateRoot::new())));
                }
                TopMenuOption::Quit => {
                    std::process::exit(0);
                }
            },
            _ => {}
        }
    }

    fn get_display_instructions(&self) -> DisplayInstructions {
        DisplayInstructions {
            draw_lower_layers: false,
        }
    }

    fn display(&self, context: DisplayContext) {
        egui::CentralPanel::default().show(context.gui_context, |ui| {
            for option in self.menu.iter() {
                if ui
                    .button(format!(
                        "{}{}",
                        if option.selected() { "> " } else { "" },
                        option.item().text()
                    ))
                    .clicked()
                {
                    option.item().pressed();
                }
            }
        });
    }
}
