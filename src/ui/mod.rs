use std::io::Stdout;

use ratatui::{backend::CrosstermBackend, Terminal};

pub mod main_page;
pub mod options_page;
pub mod search_page;

pub type Tui = Terminal<CrosstermBackend<Stdout>>;
