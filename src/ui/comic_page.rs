use crate::{app::AppState, models::comic::Comic};

use ratatui::{
    prelude::*, symbols::border, widgets::{block::*, Borders, List, ListItem, Paragraph}
};

pub struct ComicPage;

impl ComicPage {
    pub fn render_page(
        area: Rect,
        frame: &mut Frame,
        app_state: &mut AppState,
        comic: Comic,
    ) {

        let block = Block::default()
            .borders(Borders::ALL)
            .border_set(border::ROUNDED)
            .style(Style::new().fg(Color::Green));


        let text = Paragraph::new(Text::from("Not implemented"))
            .bold()
            .centered()
            .block(block);


        frame.render_widget(text, area);
    }
}
