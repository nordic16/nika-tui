use ratatui::{
    prelude::*,
    widgets::{block::*, Borders, List, ListItem},
};

use tui_textarea::TextArea;

use crate::models::comic::Comic;

#[derive(Default)]
pub struct SearchPage;

impl SearchPage {
    pub fn render_page(area: Rect, frame: &mut Frame, input: &mut TextArea, results: &Vec<Comic>) {
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

        let items = results
            .iter()
            .map(|f| ListItem::new(f.name.as_str()))
            .collect::<Vec<ListItem>>();

        let results = List::new(items).block(block2);

        frame.render_widget(input.widget(), layout[0]);
        frame.render_widget(results, layout[1]);
    }
}
