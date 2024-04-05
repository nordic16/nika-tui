use ratatui::{
    prelude::*,
    widgets::{block::*, Borders, Paragraph},
};

use crate::helpers::get_manga_from_name;

#[derive(Default)]
pub struct MainPage {
    text: String,
}

impl Widget for MainPage {
    fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {
        let block = Block::default()
            .title("Nika-tui".bold().light_red()).title_alignment(Alignment::Center)
            .border_style(Style::new().on_red())
            .borders(Borders::ALL)
            .title_bottom("<q> to quit, <s> for settings, <m> for main page.".bold().light_red());    
        
        let text = Text::from("Welcome to Nika!".light_red())
            .centered();

        Paragraph::new(text)
            .centered()
            .block(block)
            .render(area, buf);
        }
}
