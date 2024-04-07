use ratatui::{
    prelude::*,
    widgets::{block::*, Borders, List},
};

use tui_textarea::TextArea;

#[derive(Default)]
pub struct SearchPage;

impl SearchPage {
    pub fn render_page(area: Rect, frame: &mut Frame, input: &mut TextArea) {
        let layout = Layout::default()
            .spacing(2)
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(area);

        let block1 = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::new().on_yellow())
            .border_type(BorderType::Rounded)
            .title("Search")
            .title_alignment(Alignment::Center);

        input.set_block(block1);

        let block2 = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::new().on_yellow())
            .border_type(BorderType::Rounded)
            .title("Results")
            .title_alignment(Alignment::Center);

        let results =
            List::new([Text::from("ONE PIECE IS REAL"), Text::from("AYO WTF")]).block(block2);

        frame.render_widget(input.widget(), layout[0]);
        frame.render_widget(results, layout[1]);
    }
}
