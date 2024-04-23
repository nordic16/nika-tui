use std::io;

use crossterm::event::KeyCode;
use ratatui::prelude::*;
use ratatui::widgets::block::*;
use ratatui::widgets::{Borders, Paragraph};
use tokio::sync::mpsc::UnboundedSender;

use crate::app::{NikaAction, Page};
use crate::traits::Component;

#[derive(Default)]
pub struct HomePage {
    action_handler: Option<UnboundedSender<NikaAction>>,
}

impl Component for HomePage {
    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) {
        let block = Block::default()
            .title("Nika-tui".bold().light_red())
            .title_alignment(Alignment::Center)
            .border_style(Style::new().fg(Color::Red))
            .borders(Borders::ALL)
            .title_bottom(
                "<q> to quit, <s> for search, <o> for options, <m> for main page."
                    .bold()
                    .light_red(),
            );

        let text = Text::from("Welcome to Nika!".light_red()).centered();
        let paragraph = Paragraph::new(text).centered().block(block);

        f.render_widget(paragraph, rect);
    }

    fn init(&mut self, tx: UnboundedSender<NikaAction>) -> std::io::Result<()> {
        self.action_handler = Some(tx);
        Ok(())
    }

    fn handle_key_events(
        &mut self,
        key: crossterm::event::KeyEvent,
    ) -> io::Result<Option<NikaAction>> {
        match key.code {
            KeyCode::Char('q') => Ok(Some(NikaAction::Quit)),
            KeyCode::Char('s') => Ok(Some(NikaAction::ChangePage(Page::Search))),
            _ => Ok(None),
        }
    }

    fn update(&mut self, action: NikaAction) -> anyhow::Result<()> {
        Ok(())
    }
}
