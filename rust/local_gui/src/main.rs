use eframe::egui;
use gridwizard::{fakes::fake_world, states::stack::GameStateStack, world::World};

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 400.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Grid Wizard",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(LocalGui::new()))
        }),
    )
}

struct LocalGui {
    world: World,
}

impl LocalGui {
    pub fn new() -> Self {
        let mut world = World::new();
        fake_world(&mut world);
        Self { world: world }
    }
}

impl eframe::App for LocalGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        GameStateStack::update_gui(self.world.state_stack.clone(), ctx, self.world.game.clone());
    }
}
