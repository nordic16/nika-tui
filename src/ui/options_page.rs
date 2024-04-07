use ratatui::{
    prelude::*,
    widgets::{block::*, Borders},
};
#[derive(Debug, Default)]
pub struct OptionsPage;

impl Widget for OptionsPage {
    fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {
        Block::default()
            .title("Options".bold().light_red())
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(Style::new().on_dark_gray())
            .render(area, buf);
    }
}