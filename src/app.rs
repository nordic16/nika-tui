use std::io;
use std::sync::Arc;

use crossterm::event::KeyEvent;
use ratatui::style::{Style, Stylize};
use ratatui::text::Text;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use tokio::sync::mpsc::unbounded_channel;

use crate::components::comic_page::ComicPage;
use crate::components::main_page::HomePage;
use crate::components::search_page::SearchPage;
use crate::config::Config;
use crate::models::comic::{Chapter, Comic};
use crate::traits::{Component, Source};
use crate::tui::Tui;

#[derive(Default, Clone)]
pub enum Page {
    #[default]
    Home,
    Search,
    Options,
    Comic(Comic, Arc<dyn Source>),
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
    ShowLoadingScreen,
    LiftLoadingScreen,
    SelectComic(Comic),
    FetchNewChapters(bool), // true if right, false if left.
    SetChapters(Vec<Chapter>),
    FetchChapter(Chapter),
}

pub struct App {
    pub component: Box<dyn Component>,
    quit: bool,
    loading: bool,
    pub config: Config,
}

impl Default for App {
    fn default() -> Self {
        Self {
            component: Box::<HomePage>::default(),
            quit: false,
            loading: false,
            config: Config::get_or_default(),
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
                    NikaAction::ShowLoadingScreen => self.loading = true,
                    NikaAction::LiftLoadingScreen => self.loading = false,
                    NikaAction::Render => {
                        // Receiving a render request causes the app to draw the widget on screen.
                        if !self.loading {
                            tui.terminal.draw(|f| self.component.draw(f, f.size()))?;
                        } else {
                            let widget = self.get_loading_screen();

                            tui.terminal.draw(|f| f.render_widget(widget, f.size()))?;
                        }
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
            Page::Comic(c, s) => Box::new(ComicPage::new(c, s)),
        }
    }

    fn get_loading_screen(&self) -> Paragraph<'static> {
        let block = Block::default()
            .border_style(Style::new().light_blue())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        Paragraph::new(Text::from("Loading..."))
            .centered()
            .bold()
            .block(block)
    }
}
