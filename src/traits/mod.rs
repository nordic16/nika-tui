use std::io;

use async_trait::async_trait;
use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};
use tokio::sync::mpsc::UnboundedSender;

use crate::{app::NikaAction, models::{
    comic::{Chapter, Comic, ComicInfo},
    sources::mangapill::MangapillSource,
}, tui::NikaEvent};

#[async_trait]
pub trait Source: Send + Sync {

    /// Returns a list of search results based on query
    async fn search(&self, query: &str) -> reqwest::Result<Vec<Comic>>;

    fn base_url(&self) -> &'static str;

    /// Returns the chapters for a given comic
    async fn get_chapters(&self, comic: &Comic) -> reqwest::Result<Vec<Chapter>>;

    async fn get_info(&self, comic: &Comic) -> reqwest::Result<Option<ComicInfo>>;

    fn name(&self) -> &'static str;
}


impl Clone for Box<dyn Source> {
    fn clone(&self) -> Self {
        self.to_owned()
    }
}

pub trait Component {
    #[allow(unused_variables)]
    fn init(&mut self, tx: UnboundedSender<NikaAction>) -> io::Result<()>;

    fn handle_events(&mut self, event: Option<NikaEvent>) -> io::Result<Option<NikaAction>> {
        let r = match event {
            Some(NikaEvent::Key(key_event)) => self.handle_key_events(key_event)?,
            Some(NikaEvent::Render) => Some(NikaAction::Render),
            _ => None,
        };
        Ok(r)
    }

    #[allow(unused_variables)]
    fn handle_key_events(&mut self, key: KeyEvent) -> io::Result<Option<NikaAction>>;

    #[allow(unused_variables)]
    fn update(&mut self, action: NikaAction);

    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect);
}
