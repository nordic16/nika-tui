use crate::{
    tui::{Tui, NikaEvent},
    helpers::{self, get_selection_index, search_manga},
    models::comic::{Chapter, Comic, ComicInfo},
    ui::{
        comic_page::ComicPage, main_page::MainPage, options_page::OptionsPage,
        search_page::SearchPage,
    },
};
use std::io;

use crossterm::{
    cursor,
    event::{KeyCode, KeyEvent},
    execute,
    terminal::{self, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend, style::{Color, Style, Stylize}, text::Text, widgets::{Block, Borders, ListDirection, ListState, Paragraph}, Frame, Terminal
};
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
    Render,
    Error,
    Key(KeyEvent),
}

pub struct App {
    state: AppState,
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
            state: AppState::default(),
        }
    }
}

impl App {
    fn update(&mut self, action: NikaAction) -> io::Result<()> {
        match action {
            NikaAction::Render => {}
            NikaAction::Error => todo!(),
            NikaAction::Key(key) => match self.state.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => self.state.exit = true,
                    KeyCode::Char('s') => {
                        self.state.page = Page::Search;
                        self.state.list_state = ListState::default().with_selected(Some(0));
                    }
                    KeyCode::Char('o') => {
                        self.state.page = Page::Options;
                    }
                    KeyCode::Char('m') => {
                        self.state.page = Page::Main;
                    }
                    KeyCode::Char('e') => {
                        // Only goes into editing mode if there's something to edit lol.
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Esc => {
                        self.state.input_mode = InputMode::Normal;
                        self.state.list_state.select(Some(0));
                    }
                    KeyCode::Enter => {}
                    _ => {
                    }
                },
            },
        }
        Ok(())
    }

    /// runs the application main loop until the user quits
    pub async fn run(&mut self) -> io::Result<()> {
        let mut tui = Tui::new()?;

        tui.run()?;

        loop {
            let evt = tui.next().await;
            
            if let Some(NikaEvent::Render) = evt {
                tui.terminal.draw(|f| {
                    let widget = Text::from("Nika - rewritten lol");

                    f.render_widget(widget, f.size())
                })?;
            }
            

            if self.state.exit {
                break;
            }
        }

        Ok(())
    }
}
