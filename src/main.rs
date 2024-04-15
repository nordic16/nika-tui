use std::io;

use app::App;

mod app;
pub mod components;
pub mod constants;
pub mod helpers;
pub mod models;
#[cfg(test)]
mod tests;
mod tui;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut app = App::default();

    app.run().await
}
