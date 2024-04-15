use crate::{
    components::{
        comic_page::ComicPage, main_page::HomePage, options_page::OptionsPage,
        search_page::SearchPage, Component,
    },
    helpers::{self, get_selection_index, search_manga},
    models::comic::{Chapter, Comic, ComicInfo},
    tui::{NikaEvent, Tui},
};
use std::io;

use crossterm::{
    cursor,
    event::{KeyCode, KeyEvent},
    execute,
    terminal::{self, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    style::{Color, Style, Stylize},
    text::Text,
    widgets::{Block, Borders, ListDirection, ListState, Paragraph},
    Frame, Terminal,
};
use reqwest::redirect::Action;
use tokio::sync::mpsc::{error::SendError, unbounded_channel, UnboundedReceiver, UnboundedSender};
use tui_textarea::TextArea;

#[derive(Debug, Default, Clone)]
pub enum Page {
    #[default]
    Main,
    Search,
    Options,
    ViewComic(Comic),
}

#[derive(Default, Clone)]
pub enum InputMode {
    #[default]
    Normal,
    Editing,
}

#[derive(Clone)]
pub enum NikaAction {
    Error,
    Key(KeyEvent),
    Quit,
    Render,
}

pub struct App {
    state: AppState,
    pub component: Box<dyn Component>,
}

#[derive(Default, Clone)]
pub struct AppState {
    exit: bool,
    loading: bool,

    pub input_mode: InputMode,
    page: Page,
    // Should probably be an option.
    pub list_state: ListState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            component: Box::new(HomePage::default()),
            state: AppState::default(),
        }
    }
}

impl App {
    /// runs the application main loop until the user quits
    pub async fn run(&mut self) -> io::Result<()> {
        let mut tui = Tui::new()?;

        let (tx, mut rx) = unbounded_channel::<NikaAction>();

        self.component.register_action_handler(tx.clone())?;

        tui.run()?;

        loop {
            let event = tui.next().await;

            if let Some(e) = event {
                // If there was an event in a given component.
                if let Ok(Some(action)) = self.component.handle_events(Some(e)) {
                    tx.send(action).unwrap();
                }
            }

            while let Ok(act) = rx.try_recv() {
                match act {
                    NikaAction::Quit => self.state.exit = true,
                    NikaAction::Render => {
                        // Receiving a render request causes the app to draw the widget on screen.
                        tui.terminal.draw(|f| self.component.draw(f, f.size()))?;
                    }
                    _ => {
                        self.component.update(act)?;
                    }
                }
            }

            if self.state.exit {
                break;
            }
        }

        Ok(())
    }
}
