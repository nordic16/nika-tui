use crate::{app::AppState, models::comic::Comic};

use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, Borders, Paragraph},
};

pub struct ComicPage;

impl ComicPage {
    pub fn render_page(area: Rect, frame: &mut Frame, app_state: &mut AppState, comic: Comic) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)])
            .split(area);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_set(border::ROUNDED)
            .style(Style::new().fg(Color::Green));

        let text = Paragraph::new(Text::from(comic.name))
            .bold()
            .centered()
            .block(block.clone());

        let chap = Paragraph::new(Text::from("Chapters"))
            .bold()
            .centered()
            .block(block);

        frame.render_widget(text, layout[0]);
        frame.render_widget(chap, layout[1])
    }
}
