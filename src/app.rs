use crate::{
    components::{
        main_page::HomePage, search_page::SearchPage, Component
    },
    models::comic::Comic,
    tui::Tui,
};
use std::io;

use crossterm::event::KeyEvent;

use tokio::sync::mpsc::unbounded_channel;

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
    ChangePage(Page),
    SearchComic(String),
    SetSearchResults(Vec<Comic>)
}

pub struct App {
    state: AppState,
    pub component: Box<dyn Component>,
}

#[derive(Default, Clone)]
pub struct AppState {
    exit: bool,
    loading: bool,
    page: Page,
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
        tui.init_panic_hook();
        
        let (tx, mut rx) = unbounded_channel::<NikaAction>();
        self.component.register_action_handler(tx.clone())?;

        tui.run()?;

        loop {
            let event = tui.next().await;

            if let Some(e) = event {
                // If there was an event in a given component.
                if let Ok(Some(action)) = self.component.handle_events(Some(e)) {             
                    // ChangePage should be handled in the main loop.
                    if let NikaAction::ChangePage(page) = action {
                        let page = self.get_page(page);
                        self.component = page;

                        // Needs to be registered again after assigning a new component.
                        self.component.register_action_handler(tx.clone())?;
                        continue;
                    }
                    
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
                        self.component.update(act);
                    }
                }
            }

            if self.state.exit {
                break;
            }
        }

        Ok(())
    }


    fn get_page(&self, page: Page) -> Box<dyn Component> {
        match page {
            Page::Main => Box::new(HomePage::default()),
            Page::Search => Box::new(SearchPage::default()),
            Page::Options => todo!(),
            Page::ViewComic(_) => todo!(),
        }    
    }
}
