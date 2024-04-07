use std::{default, io};
use crate::{event_handler::{EventHandler, NikaEvent}, ui::{self, main_page::MainPage, search_page::SearchPage, options_page::OptionsPage, Tui}};

#[derive(Debug, Default)]
pub enum Page {
    #[default]
    Main,
    Search,
    Options
}

#[derive(Default, Debug)]
pub enum InputMode {
    #[default]
    Normal,
    Editing,
}

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    page: Page,
    textarea: TextArea<'static>,
    input_mode: InputMode
}

use std::io::stdout;

use crossterm::{cursor, event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, execute, terminal::{self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use ratatui::{backend::CrosstermBackend, layout::Rect, Frame, Terminal};
use tui_textarea::TextArea;


impl App {
    fn update(&mut self, event: NikaEvent) -> io::Result<()> {
        if let NikaEvent::Key(key) = event {
            match self.input_mode {
                
                InputMode::Normal => {
                    match key.code {
                        KeyCode::Char('q') => self.exit = true,
                        KeyCode::Char('s') => self.page = Page::Search,
                        KeyCode::Char('o') => self.page = Page::Options, 
                        KeyCode::Char('m') => self.page = Page::Main,
                        KeyCode::Char('e') => self.input_mode = InputMode::Editing,
                        
                        _ => {},
                    }
                },
                InputMode::Editing => {
                    match key.code {
                        KeyCode::Esc => self.input_mode = InputMode::Normal,
                        KeyCode::Enter => {}
                        _ => {
                            
                            self.textarea.input(key);
                        }
                    }
                },
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
            Page::Search => {
            // to handle events and stuff.
                SearchPage::render_page(frame.size(), frame, &mut self.textarea);


            },
            Page::Options => frame.render_widget(OptionsPage::default(), frame.size()),
        };
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