use std::io;
use std::sync::Arc;

use crossterm::event::KeyEvent;
use lazy_static::lazy_static;
use reqwest::{Client, ClientBuilder};
use tokio::sync::mpsc::unbounded_channel;

use crate::components::comic_page::ComicPage;
use crate::components::loading_screen::LoadingScreen;
use crate::components::main_page::HomePage;
use crate::components::search_page::SearchPage;
use crate::config::Config;
use crate::models::comic::{Chapter, Comic, ComicInfo};
use crate::traits::{Component, Source};
use crate::tui::Tui;

lazy_static! {
    pub static ref CLIENT: Client = ClientBuilder::new().gzip(true).build().unwrap();
}

#[derive(Default, Clone)]
pub enum Page {
    #[default]
    Home,
    Search,
    Options,
    Comic(Comic, Arc<dyn Source>, ComicInfo),
    /// string: text shown to the user.
    /// u16: progress of a given operation.
    LoadingScreen(&'static str, Option<f64>),
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
    SetSearchResults(Vec<Comic>),
    SelectComic(Comic),
    FetchNewChapters(bool), // true if right, false if left.
    SetChapters(Vec<Chapter>),
    FetchChapter(Chapter),
    UpdateLoadingScreen(String, f64),
}

pub struct App {
    component: Box<dyn Component>,
    quit: bool,
    config: Config,
}

impl App {
    pub fn new(config: Config) -> Self {
        Self {
            component: Box::<HomePage>::default(),
            quit: false,
            config,
        }
    }
}

impl App {
    /// runs the application main loop until the user quits
    pub async fn run(&mut self) -> io::Result<()> {
        let mut tui = Tui::new()?;
        tui.init_panic_hook();

        let (tx, mut rx) = unbounded_channel::<NikaAction>();
        self.component.init(tx.clone())?;

        tui.run()?;

        loop {
            let event = tui.next().await;

            if let Some(e) = event {
                // If there was an event in a given component.
                if let Ok(Some(action)) = self.component.handle_events(Some(e)) {
                    // ChangePage should be handled in the main loop
                    tx.send(action).unwrap();
                }
            }

            // Action handler.
            while let Ok(act) = rx.try_recv() {
                match act {
                    NikaAction::Quit => self.quit = true,
                    NikaAction::Render => {
                        // Receiving a render request causes the app to draw the widget on screen.
                        tui.terminal.draw(|f| self.component.draw(f, f.size()))?;
                    }

                    NikaAction::ChangePage(page) => {
                        let page = self.get_component(page);
                        self.component = page;

                        // Needs to be registered again after assigning a new component.
                        self.component.init(tx.clone())?;
                    }
                    _ => {
                        self.component.update(act).unwrap();
                    }
                }
            }

            if self.quit {
                break;
            }
        }

        Ok(())
    }

    fn get_component(&self, page: Page) -> Box<dyn Component> {
        match page {
            Page::Home => Box::<HomePage>::default(),
            Page::Search => Box::<SearchPage>::default(),
            Page::Options => todo!(),
            Page::Comic(c, s, i) => Box::new(ComicPage::new(c, s, i, self.config.clone())),
            Page::LoadingScreen(t, p) => Box::new(LoadingScreen::new(p, t)),
        }
    }
}
