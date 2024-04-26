use std::io;

use app::App;
use config::Config;

mod app;
pub mod components;
pub mod config;
pub mod constants;
pub mod helpers;
pub mod models;
pub mod traits;
mod tui;

#[tokio::main]
async fn main() -> io::Result<()> {
    let config = Config::get_or_default();
    let mut app = App::new(config);
    app.run().await
}