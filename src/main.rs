use std::io;

use app::App;

mod app;
pub mod constants;
mod event_handler;
pub mod helpers;
pub mod models;
#[cfg(test)]
mod tests;
pub mod ui;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut app = App::default();
    app.init()?;
    app.run().await?;

    App::restore()
}
