use crate::{app::AppState, models::comic::Comic};

use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, Borders, Paragraph},
};

pub struct ComicPage;

impl ComicPage {
    pub fn render_page(area: Rect, frame: &mut Frame, app_state: &mut AppState, comic: &Comic) {
        let info = comic.manga_info.as_ref().unwrap();

        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)])
            .split(area);

        let inner_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(15), Constraint::Percentage(85)])
            .split(main_layout[0]);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_set(border::ROUNDED)
            .style(Style::new().fg(Color::Green));

        let title = Paragraph::new(Text::from(comic.name.to_owned().bold()))
            .centered()
            .block(block.clone());

        let more_info = Paragraph::new(vec![
            format!("Year: {}", info.year.to_string().bold()).into(),
            format!("Genres: {}", info.genres.join(", ").bold()).into(),
        ])
        .centered()
        .block(block.clone());


        let lines: Vec<Text> = comic.chapters.iter().map(|f| Text::from(f.name.as_str())).collect();

        
        let chap = Paragraph::new(Text::from("Chapters"))
            .bold()
            .centered()
            .block(block);

        frame.render_widget(title, inner_layout[0]);
        frame.render_widget(more_info, inner_layout[1]);
        frame.render_widget(chap, main_layout[1])
    }
}
