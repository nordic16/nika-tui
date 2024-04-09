use ratatui::{
    prelude::*,
    widgets::{block::*, Borders, List, ListItem, ListState},
};

use tui_textarea::TextArea;

use crate::{
    app::{AppState, InputMode},
    models::comic::Comic,
};

#[derive(Default)]
pub struct SearchPage;

impl SearchPage {
    pub fn render_page(
        area: Rect,
        frame: &mut Frame,
        input: &mut TextArea,
        results: &Vec<Comic>,
        app_state: &mut AppState,
    ) {
        // decides the right color for the results and text bar
        let (scolor, rcolor) = match app_state.input_mode {
            InputMode::Normal => (Color::default(), Color::Yellow),
            InputMode::Editing => (Color::Yellow, Color::default()),
        };

        let layout = Layout::default()
            .spacing(2)
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(area);

        let block1 = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::new().fg(scolor))
            .border_type(BorderType::Rounded)
            .title("Search")
            .title_alignment(Alignment::Center);

        input.set_block(block1);

        let block2 = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::new().fg(rcolor))
            .border_type(BorderType::Rounded)
            .title("Results")
            .title_alignment(Alignment::Center);

        let items = results
            .iter()
            .map(|f| ListItem::new(f.name.as_str()))
            .collect::<Vec<ListItem>>();

        let results = List::new(items)
            .block(block2)
            .highlight_style(Style::new().fg(Color::Yellow));

        frame.render_widget(input.widget(), layout[0]);

        frame.render_stateful_widget(results, layout[1], &mut (app_state.list_state));
    }
}
