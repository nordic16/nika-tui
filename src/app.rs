use crate::{
    event_handler::{EventHandler, NikaMessage},
    ui::{main_page::MainPage, options_page::OptionsPage, search_page::SearchPage},
};
use std::io;

use crossterm::{
    cursor,
    event::{KeyCode, KeyEvent},
    execute,
    terminal::{self, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Frame, Terminal};
use tokio::sync::mpsc::{error::SendError, unbounded_channel, UnboundedReceiver, UnboundedSender};
use tui_textarea::TextArea;

#[derive(Debug, Default)]
pub enum Page {
    #[default]
    Main,
    Search,
    Options,
}

#[derive(Default, Debug)]
pub enum InputMode {
    #[default]
    Normal,
    Editing,
}

#[derive(Debug, Clone)]
pub enum NikaAction<'a> {
    UpdateQuery(&'a str),
    Render,
    Error,
    Key(KeyEvent),
}

#[derive(Debug)]
pub struct App {
    exit: bool,
    page: Page,
    textarea: TextArea<'static>,
    input_mode: InputMode,
    action_s: UnboundedSender<NikaAction<'static>>,
    action_r: UnboundedReceiver<NikaAction<'static>>,
}

impl Default for App {
    fn default() -> Self {
        let (s, r) = unbounded_channel::<NikaAction<'static>>();

        Self {
            exit: Default::default(),
            page: Default::default(),
            textarea: Default::default(),
            input_mode: Default::default(),
            action_s: s,
            action_r: r,
        }
    }
}

use std::io::stdout;

impl App {
    fn update(&mut self, action: NikaAction) -> io::Result<()> {
        match action {
            NikaAction::UpdateQuery(_) => todo!(),
            NikaAction::Render => {}
            NikaAction::Error => todo!(),
            NikaAction::Key(key) => match self.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => self.exit = true,
                    KeyCode::Char('s') => self.page = Page::Search,
                    KeyCode::Char('o') => self.page = Page::Options,
                    KeyCode::Char('m') => self.page = Page::Main,
                    KeyCode::Char('e') => self.input_mode = InputMode::Editing,

                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Esc => self.input_mode = InputMode::Normal,
                    KeyCode::Enter => {}
                    _ => {
                        self.textarea.input(key);
                    }
                },
            },
        }

        Ok(())
    }

    /// runs the application main loop until the user quits
    pub async fn run(&mut self) -> io::Result<()> {
        let mut events = EventHandler::new();
        let mut terminal = Terminal::new(CrosstermBackend::new(io::stderr())).unwrap();

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

            if self.exit {
                break;
            }
        }

        Ok(())
    }

    /// Figures out which page is to be rendered based on self.page.
    fn render_page(&mut self, frame: &mut Frame) {
        match self.page {
            Page::Main => MainPage::render_page(frame.size(), frame),
            Page::Search => SearchPage::render_page(frame.size(), frame, &mut self.textarea),
            Page::Options => OptionsPage::render_page(frame.size(), frame),
        };
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
