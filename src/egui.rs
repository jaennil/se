use eframe::egui;

use crate::{home::Home, welcome};

#[derive(Default)]
pub struct Egui {
    native_options: eframe::NativeOptions,
    state: State,
    home: Home,
}

impl Egui {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl Egui {
    pub fn run(self) -> eframe::Result {
        eframe::run_native(
            "Money Manager Ex",
            self.native_options,
            Box::new(|cc| {
                egui_extras::install_image_loaders(&cc.egui_ctx);
                Ok(Box::new(Self::new(cc)))
            }),
        )
    }
}

impl eframe::App for Egui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.state {
            State::Welcome => welcome::render(ctx),
            State::Home => self.home.render(ctx),
        };
    }
}

#[derive(Default)]
enum State {
    Welcome,
    #[default]
    Home,
}
