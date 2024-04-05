use std::io;
use crate::ui::{self, main_page::MainPage, settings_page::SettingsPage, Tui};

#[derive(Debug, Default)]
enum Page {
    #[default]
    Main,
    Search,
    Settings
}


#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    page: Page
}

use std::io::stdout;

use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use ratatui::{backend::CrosstermBackend, Frame, Terminal};


impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut ui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_page(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    /// Figures out which page is to be rendered based on self.page.
    fn render_page(&mut self, frame: &mut Frame) {
        match self.page {
            Page::Main => frame.render_widget(MainPage::default(), frame.size()),
            Page::Search => todo!(),
            Page::Settings => frame.render_widget(SettingsPage::default(), frame.size()),
        };
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('s') => self.page = Page::Settings,
            KeyCode::Char('m') => self.page = Page::Main,
            _ => {}
        }
    }


    /// Code ran before the app exits.
    fn exit(&mut self) {
        self.exit = true;
    }

    /// Initialize the terminal
    pub fn init() -> io::Result<Tui> {
        execute!(stdout(), EnterAlternateScreen)?;
        enable_raw_mode()?;

        Terminal::new(CrosstermBackend::new(stdout()))
    }

    /// Restore the terminal to its original state
    pub fn restore() -> io::Result<()> {
        execute!(stdout(), LeaveAlternateScreen)?;
        disable_raw_mode()?;
        
        Ok(())
    }
}