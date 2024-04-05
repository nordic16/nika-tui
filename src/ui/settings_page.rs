use ratatui::{
    prelude::*,
    widgets::{block::*, Borders, Paragraph},
};
#[derive(Debug, Default)]
pub struct SettingsPage;

impl Widget for SettingsPage {
    fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {
        Block::default()
            .title("Settings".bold().light_red())
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(Style::new().on_dark_gray())
            .render(area, buf);
    }
}