use app::App;
mod models;
mod app;
mod home;
mod egui;
mod welcome;

fn main() -> anyhow::Result<()> {
    let db = rusqlite::Connection::open_in_memory()?;
    let app = App::new(db)?;
    app.run();
    Ok(())
}
