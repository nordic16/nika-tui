use std::io;

use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};
use tokio::sync::mpsc::UnboundedSender;

use crate::{app::NikaAction, tui::NikaEvent};

pub mod comic_page;
pub mod main_page;
pub mod options_page;
pub mod search_page;

pub trait Component {
    #[allow(unused_variables)]
    fn register_action_handler(&mut self, tx: UnboundedSender<NikaAction>) -> io::Result<()>;

    fn init(&mut self) -> io::Result<()> {
        Ok(())
    }

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
    fn update(&mut self, action: NikaAction) -> io::Result<Option<NikaAction>>;

    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect);
}
