use crate::egui::Egui;

pub struct App {
    db: rusqlite::Connection,
    egui: Egui,
}

impl App {
    pub fn new(db: rusqlite::Connection) -> anyhow::Result<Self> {
        Ok(Self {
            db,
            egui: Default::default(),
        })
    }

    pub fn run(self) {
        self.egui.run();
    }
}
