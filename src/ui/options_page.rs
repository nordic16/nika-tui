use ratatui::{
    prelude::*,
    widgets::{block::*, Borders, Paragraph},
};
#[derive(Debug, Default)]
pub struct OptionsPage;

impl OptionsPage {
    pub fn render_page(area: Rect, frame: &mut Frame)
    where
        Self: Sized,
    {
        let block = Block::default()
            .title("Options".bold().light_red())
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(Style::new().on_dark_gray());

        let text = Paragraph::new("Not implemented")
            .centered()
            .bold()
            .block(block);

        frame.render_widget(text, area)
    }
}
