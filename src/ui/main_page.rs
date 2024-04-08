use ratatui::{
    prelude::*,
    widgets::{block::*, Borders, Paragraph},
};

#[derive(Default)]
pub struct MainPage;

impl MainPage {
    pub fn render_page(area: Rect, frame: &mut Frame)
    where
        Self: Sized,
    {
        let block = Block::default()
            .title("Nika-tui".bold().light_red())
            .title_alignment(Alignment::Center)
            .border_style(Style::new().on_red())
            .borders(Borders::ALL)
            .title_bottom(
                "<q> to quit, <s> for search, <o> for options, <m> for main page."
                    .bold()
                    .light_red(),
            );

        let text = Text::from("Welcome to Nika!".light_red()).centered();

        let paragraph = Paragraph::new(text).centered().block(block);

        frame.render_widget(paragraph, area);
    }
}
