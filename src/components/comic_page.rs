use crate::{
    app::{NikaAction, Page},
    models::comic::Comic,
};

use crossterm::event::KeyCode;
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, Borders, List, ListState, Paragraph},
};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;

pub struct ComicPage {
    action_tx: Option<UnboundedSender<NikaAction>>,
    comic: Comic,
    list_state: ListState,
}

impl ComicPage {
    pub fn new(comic: Comic) -> Self {
        Self {
            action_tx: None,
            comic,
            list_state: ListState::default(),
        }
    }
}

impl Component for ComicPage {
    fn register_action_handler(&mut self, tx: UnboundedSender<NikaAction>) -> std::io::Result<()> {
        self.action_tx = Some(tx);

        Ok(())
    }

    fn handle_key_events(
        &mut self,
        key: crossterm::event::KeyEvent,
    ) -> std::io::Result<Option<NikaAction>> {
        match key.code {
            KeyCode::Char('q') => Ok(Some(NikaAction::Quit)),
            KeyCode::Char('s') => Ok(Some(NikaAction::ChangePage(Page::Search))),
            KeyCode::Char('h') => Ok(Some(NikaAction::ChangePage(Page::Main))),
            _ => Ok(None),
        }
    }

    fn update(&mut self, action: NikaAction) {}

    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) {
        let info = self.comic.manga_info.as_ref().unwrap();

        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)])
            .split(rect);

        let inner_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(main_layout[0]);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_set(border::ROUNDED)
            .style(Style::new().fg(Color::Green));

        let title = Paragraph::new(Text::from(self.comic.name.to_owned().bold()))
            .centered()
            .block(block.clone());

        let more_info = Paragraph::new(vec![
            format!("Year: {}", info.year.to_string().bold()).into(),
            format!("Genres: {}", info.genres.join(", ").bold()).into(),
        ])
        .centered()
        .block(block.clone());

        let list = self
            .comic
            .chapters
            .iter()
            .map(|f| Text::from(f.name.as_str()))
            .collect::<List>()
            .block(block)
            .highlight_style(Style::new().fg(Color::LightGreen));

        f.render_widget(title, inner_layout[0]);
        f.render_widget(more_info, inner_layout[1]);
        f.render_stateful_widget(list, main_layout[1], &mut self.list_state);
    }
}
