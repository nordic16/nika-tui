use std::io::Stdout;

use ratatui::{backend::CrosstermBackend, Terminal};

pub mod main_page;
pub mod settings_page;

pub type Tui = Terminal<CrosstermBackend<Stdout>>;
