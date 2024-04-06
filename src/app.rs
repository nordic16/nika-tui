use std::io;
use crate::{event_handler::{EventHandler, NikaEvent}, ui::{self, main_page::MainPage, settings_page::SettingsPage, Tui}};

#[derive(Debug, Default)]
pub enum Page {
    #[default]
    Main,
    Search,
    Options
}


#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    page: Page
}

use std::io::stdout;

use crossterm::{cursor, event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, execute, terminal::{self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use ratatui::{backend::CrosstermBackend, Frame, Terminal};


impl App {
    fn update(&mut self, event: NikaEvent) -> io::Result<()> {
        if let NikaEvent::Key(key) = event {
          match key.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('s') => self.page = Page::Search,
            KeyCode::Char('o') => self.page = Page::Options, 
            KeyCode::Char('m') => self.page = Page::Main,
            _ => {},
          }
        }
        Ok(())
      }

    /// runs the application's main loop until the user quits
    pub async fn run(&mut self) -> io::Result<()> {
        let mut events = EventHandler::new();
        let mut terminal = Terminal::new(CrosstermBackend::new(io::stderr()))?;

        loop {
            let event = events.next().await.unwrap();
            self.update(event.clone())?;

            if let NikaEvent::Render = event {
                // application render
                terminal.draw(|f| {
                  self.render_page(f);
                })?;
              }
            
            if self.exit {
                break;
            }
        };

        Ok(())
    }

    /// Figures out which page is to be rendered based on self.page.
    fn render_page(&mut self, frame: &mut Frame) {
        match self.page {
            Page::Main => frame.render_widget(MainPage::default(), frame.size()),
            Page::Search => todo!(),
            Page::Options => frame.render_widget(SettingsPage::default(), frame.size()),
        };
    }


    pub fn init(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(std::io::stderr(), EnterAlternateScreen, cursor::Hide)?;
        
        Ok(())
    }


    /// Code ran before the app exits.
    fn exit(&mut self) {
        self.exit = true;
    }

    /// Restore the terminal to its original state
    pub fn restore() -> io::Result<()> {
        execute!(stdout(), LeaveAlternateScreen)?;
        disable_raw_mode()?;
        
        Ok(())
    }
}