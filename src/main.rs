use std::io;

use app::App;

pub mod ui;
pub mod helpers;
mod app;
pub mod constants;
pub mod models;
#[cfg(test)]
mod tests;
mod event_handler;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut app = App::default();   
    app.init()?; 
    app.run().await?;

    App::restore()
}
