pub mod displays;
pub mod fakes;
pub mod game;
pub mod ids;
pub mod mechanics;
pub mod menus;
pub mod states;
pub mod world;

use godot::prelude::*;

struct GridWizardExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GridWizardExtension {}
