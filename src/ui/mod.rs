mod state;
mod home;

use eframe::egui;
use home::Home;
use state::State;

use crate::domain::Domain;


pub struct Ui {
    domain: Domain,
    state: State,
    home: Home,
}

impl Ui {
    pub fn new(domain: Domain) -> Self {
        Self {
            domain,
            state: Default::default(),
            home: Default::default(),
        }
    }

    pub fn run(self) {
        let native_options = eframe::NativeOptions::default();

        eframe::run_native(
            "Money Manager Ex",
            native_options,
            Box::new(|cc| {
                egui_extras::install_image_loaders(&cc.egui_ctx);
                Ok(Box::new(Self::app_creator(cc, self.domain)))
            }),
        )
        .expect("failed to eframe::run_native");
    }

    fn app_creator(_cc: &eframe::CreationContext<'_>, domain: Domain) -> Self {
        Self::new(domain)
    }
}

impl eframe::App for Ui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.state {
            //State::Welcome => welcome::render(ctx),
            State::Home => self.home.render(ctx),
        };
    }
}
