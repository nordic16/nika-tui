use crate::{
    event_handler::{EventHandler, NikaMessage},
    helpers::{self, search_manga},
    models::comic::{Comic, ComicInfo},
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
    backend::CrosstermBackend,
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, ListState, Paragraph},
    Frame, Terminal,
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
    UpdateSearchQuery,
    Render,
    Error,
    Key(KeyEvent),
    LoadSearchResults(Vec<Comic>),
    LoadMangaByName(String),
    SelectComic(Comic),
    RenderComicPage(ComicInfo),
    LiftLoadingScreen,
}

pub struct App {
    state: AppState,

    // textarea should be an option in order to invalidate it as soon as the user switches to another page.
    textarea: Option<TextArea<'static>>,
    action_s: UnboundedSender<NikaAction>,
    action_r: UnboundedReceiver<NikaAction>,

    // Action to run when needed.
    action: Option<NikaAction>,

    // APP DATA, might be refactored in the future:
    search_results: Vec<Comic>,
    selected_comic: Option<Comic>,
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
        let (s, r) = unbounded_channel::<NikaAction>();

        Self {
            state: AppState::default(),
            textarea: Default::default(),

            action_s: s,
            action_r: r,
            action: None,
            search_results: Vec::new(),
            selected_comic: None,
        }
    }
}

use std::io::stdout;

impl App {
    fn update(&mut self, action: NikaAction) -> io::Result<()> {
        match action {
            NikaAction::UpdateSearchQuery => {
                if let Some(text) = &mut self.textarea {
                    let content = &text.lines()[0];
                    self.action_s
                        .send(NikaAction::LoadMangaByName(content.to_owned()))
                        .unwrap();
                }
            }
            NikaAction::Render => {}
            NikaAction::Error => todo!(),
            NikaAction::Key(key) => match self.state.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => self.state.exit = true,
                    KeyCode::Char('s') => {
                        self.state.page = Page::Search;
                        self.state.list_state = ListState::default().with_selected(Some(0));
                        self.textarea = Some(TextArea::default());
                    }
                    KeyCode::Char('o') => {
                        self.state.page = Page::Options;
                        self.textarea = None;
                    }
                    KeyCode::Char('m') => {
                        self.state.page = Page::Main;
                        self.textarea = None;
                    }
                    KeyCode::Char('e') => {
                        // Only goes into editing mode if there's something to edit lol.
                        if let Some(txt) = &mut self.textarea {
                            self.state.input_mode = InputMode::Editing;
                            txt.set_cursor_style(Style::new().rapid_blink());
                        }
                    }

                    KeyCode::Down | KeyCode::Left => {
                        let index = match self.state.list_state.selected() {
                            Some(i) => {
                                if i == self.search_results.len() - 1 {
                                    // Prevent user from selecting elements below the list
                                    i
                                } else {
                                    i + 1
                                }
                            }
                            None => 0,
                        };

                        self.selected_comic = Some(self.search_results[index].clone());
                        self.state.list_state.select(Some(index));
                    }

                    KeyCode::Up | KeyCode::Right => {
                        let index = match self.state.list_state.selected() {
                            Some(i) => {
                                // There's no element -1, duhhh
                                if i > 0 {
                                    i - 1
                                } else {
                                    i
                                }
                            }
                            None => 1,
                        };

                        self.selected_comic = Some(self.search_results[index].clone());
                        self.state.list_state.select(Some(index));
                    }

                    KeyCode::Enter => {
                        if let Some(action) = &self.action {
                            self.action_s.send(action.to_owned()).unwrap();
                        }
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
                        if let Some(textarea) = &mut self.textarea {
                            textarea.input(key);

                            if let Some(action) = &self.action {
                                self.action_s.send(action.to_owned()).unwrap();
                            }
                        }
                    }
                },
            },
            NikaAction::LoadSearchResults(comics) => self.search_results = comics,
            NikaAction::LoadMangaByName(query) => {
                let sender = self.action_s.clone();

                tokio::spawn(async move {
                    let results = search_manga(&query).await;

                    if let Ok(results) = results {
                        sender.send(NikaAction::LoadSearchResults(results)).unwrap();
                    }
                });
            }
            NikaAction::SelectComic(comic) => {
                let sender = self.action_s.clone();
                self.state.loading = true;

                // Loads comic info.
                tokio::spawn(async move {
                    if let Ok(Some(info)) = helpers::get_comic_info(&comic).await {
                        sender.send(NikaAction::RenderComicPage(info)).unwrap();
                        sender.send(NikaAction::LiftLoadingScreen).unwrap();
                    }
                });
            }
            NikaAction::RenderComicPage(info) => {
                if let Some(comic) = &mut self.selected_comic {
                    comic.manga_info = Some(info);
                    self.state.page = Page::ViewComic(comic.to_owned());
                }
            }
            NikaAction::LiftLoadingScreen => self.state.loading = false,
        }
        Ok(())
    }

    /// runs the application main loop until the user quits
    pub async fn run(&mut self) -> io::Result<()> {
        let mut events = EventHandler::new();
        let mut terminal = Terminal::new(CrosstermBackend::new(io::stderr()))?;

        loop {
            let message = events.next().await.unwrap();
            match self.send_action(message.clone()) {
                Ok(_) => {}
                Err(e) => panic!("Failed to send message {}", e),
            }

            // "while there are new actions, update the app."
            // NOTE: Don't use the async version recv(). not a very bright idea given we're messing with UI here.
            while let Ok(action) = self.action_r.try_recv() {
                self.update(action.clone())?;

                if let NikaAction::Render = action {
                    terminal.draw(|f| {
                        self.render_page(f);
                    })?;
                }
            }

            if self.state.exit {
                break;
            }
        }

        Ok(())
    }

    /// Figures out which page is to be rendered based on self.page.
    fn render_page(&mut self, frame: &mut Frame) {
        if !self.state.loading {
            match self.state.page.clone() {
                Page::Main => MainPage::render_page(frame.size(), frame),
                Page::Search => {
                    if let Some(s) = &mut self.textarea {
                        SearchPage::render_page(
                            frame.size(),
                            frame,
                            s,
                            &self.search_results,
                            &mut self.state,
                        );
                    }

                    // If the user isn't editing anything, then the right action will be to load the comic view page.
                    self.action = match self.state.input_mode {
                        InputMode::Normal => match &self.selected_comic {
                            Some(comic) => Some(NikaAction::SelectComic(comic.clone())),
                            None => None,
                        },
                        InputMode::Editing => Some(NikaAction::UpdateSearchQuery),
                    }
                }
                Page::Options => OptionsPage::render_page(frame.size(), frame),
                Page::ViewComic(comic) => {
                    ComicPage::render_page(frame.size(), frame, &mut self.state, &comic)
                }
            };
        } else {
            let block = Block::default()
                .borders(Borders::ALL)
                .style(Style::new().fg(Color::Red));

            let paragraph = Paragraph::new("Loading...").centered().bold().block(block);

            frame.render_widget(paragraph, frame.size())
        }
    }

    fn send_action(&self, message: NikaMessage) -> Result<(), SendError<NikaAction>> {
        let message = match message {
            NikaMessage::Render => NikaAction::Render,
            NikaMessage::Error => NikaAction::Error,
            NikaMessage::Key(e) => NikaAction::Key(e),
        };
        self.action_s.send(message)
    }

    pub fn init(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(std::io::stderr(), EnterAlternateScreen, cursor::Hide)?;

        Ok(())
    }

    /// Restore the terminal to its original state
    pub fn restore() -> io::Result<()> {
        execute!(stdout(), LeaveAlternateScreen)?;
        disable_raw_mode()?;

        Ok(())
    }
}
