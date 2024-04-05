use std::io;

use app::App;

pub mod ui;
pub mod helpers;
mod app;
pub mod constants;
pub mod models;
#[cfg(test)]
mod tests;

fn main() -> io::Result<()> {
    let mut term = App::init().unwrap();
    let mut app = App::default();    

    app.run(&mut term).unwrap();

    App::restore()
}
