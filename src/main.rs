use app::App;
mod entity;
mod app;
mod repository;
mod domain;
mod ui;

fn main() -> anyhow::Result<()> {
    // config

    let app = App::default();
    app.run()?;
    Ok(())
}
