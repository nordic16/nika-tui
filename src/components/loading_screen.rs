use std::io;

use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, style::{Style, Stylize}, text::Text, widgets::{Block, BorderType, Borders, Paragraph}, Frame};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::{app::NikaAction, traits::Component};

#[derive(Default, Clone)]
 pub struct LoadingScreen {
     progress: Option<u16>,
     text: String,
     sender: Option<UnboundedSender<NikaAction>>
 }

impl LoadingScreen {
    pub fn new(progress: Option<u16>, text: &str) -> Self {
        Self { progress,  text: text.to_owned(), sender: None}
    }
}

impl Component for LoadingScreen {
    fn init(&mut self, tx: UnboundedSender<NikaAction>) -> io::Result<()> {
        self.sender = Some(tx);
        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> io::Result<Option<NikaAction>> {
        Ok(None)
    }

    fn update(&mut self, action: crate::app::NikaAction) -> anyhow::Result<()> {
        Ok(())
    }

    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) {
        let block = Block::default()
            .border_style(Style::new().light_blue())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let p = Paragraph::new(self.text.to_owned())
            .centered()
            .bold()
            .block(block);

        f.render_widget(p, rect)
    }
}
