use crate::{domain::Domain, repository::Repository, ui::Ui};

#[derive(Default)]
pub struct App {
}

impl App {
    pub fn run(self) -> anyhow::Result<()> {
        let db = rusqlite::Connection::open_in_memory()?;
        let repository = Repository::new(db);
        let domain = Domain::new(repository);
        let ui = Ui::new(domain);
        ui.run();
        Ok(())
    }
}
