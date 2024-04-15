use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{block::*, Borders, List, ListDirection, ListItem, ListState},
};

use tokio::sync::mpsc::UnboundedSender;
use tui_textarea::TextArea;

use crate::{
    app::{AppState, InputMode, NikaAction, Page}, helpers, models::comic::Comic
};

use super::Component;

#[derive(Default)]
pub struct SearchPage {
    action_tx: Option<UnboundedSender<NikaAction>>,
    search_results: Vec<Comic>,
    text_area: TextArea<'static>,
    mode: InputMode,
    list_state: ListState,
}


impl Component for SearchPage {
    fn register_action_handler(&mut self, tx: tokio::sync::mpsc::UnboundedSender<crate::app::NikaAction>) -> std::io::Result<()> {
        self.action_tx = Some(tx);
        Ok(())

    }

    fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) -> std::io::Result<Option<crate::app::NikaAction>> {
        match self.mode {
            InputMode::Normal => {
                match key.code {
                    KeyCode::Char('h') => Ok(Some(NikaAction::ChangePage(Page::Main))),
                    KeyCode::Char('q') => Ok(Some(NikaAction::Quit)),
                    KeyCode::Char('e') => {
                        self.mode = InputMode::Editing;
                        Ok(None)   
                    },

                    KeyCode::Up => {
                        let index = helpers::get_selection_index(self.list_state.selected(), 
                            self.search_results.len(), ListDirection::BottomToTop);

                        self.list_state.select(Some(index));

                        Ok(None) // temporary
                    },

                    KeyCode::Down => {
                        let index = helpers::get_selection_index(self.list_state.selected(), 
                            self.search_results.len(), ListDirection::TopToBottom);

                        self.list_state.select(Some(index));

                        Ok(None) // temporary
                    },

                    _ => Ok(None)
                }
            },
            InputMode::Editing => {
                if let KeyEventKind::Press = key.kind {
                    match key.code {
                        KeyCode::Esc => {
                            self.mode = InputMode::Normal;
                            return Ok(None);
                        }

                        KeyCode::Enter => {}, // No newline lil bro

                        _ => {
                            self.text_area.input(key);
                            let query = &self.text_area.lines()[0];



                            return Ok(Some(NikaAction::SearchComic(query.to_owned())))
                        }
                    }
                }

                Ok(None)
            },
        }
    }

    fn update(&mut self, action: crate::app::NikaAction) {
        match action {
            NikaAction::SearchComic(query) => {
                let sender = self.action_tx.clone().unwrap();
                tokio::spawn(async move {
                    let results = helpers::search_manga(&query).await;

                    let message = match results {
                        Ok(val) => NikaAction::SetSearchResults(val),
                        Err(e) => NikaAction::Error,
                    };

                    sender.send(message).unwrap();
                });

                
            }

            NikaAction::SetSearchResults(r) => self.search_results = r,

            _ => {},
        }
    }

    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) {
        let (scolor, rcolor) = match self.mode {
            InputMode::Normal => (Color::default(), Color::Yellow),
            InputMode::Editing => (Color::Yellow, Color::default()),
        };

        let layout = Layout::default()
            .spacing(2)
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(f.size());
        

        let block1 = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::new().fg(scolor))
            .border_type(BorderType::Rounded)
            .title("Search")
            .title_alignment(Alignment::Center);

        self.text_area.set_block(block1);

        let block2 = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::new().fg(rcolor))
            .border_type(BorderType::Rounded)
            .title("Results")
            .title_alignment(Alignment::Center);

        let items = self.search_results
            .iter()
            .map(|f| ListItem::new(f.name.as_str()))
            .collect::<Vec<ListItem>>();

        let results = List::new(items)
            .block(block2)
            .highlight_style(Style::new().fg(Color::Yellow));

        f.render_widget(self.text_area.widget(), layout[0]);
        f.render_stateful_widget(results, layout[1], &mut self.list_state);    }
}


impl SearchPage {
    pub fn render_page(
        area: Rect,
        frame: &mut Frame,
        input: &mut TextArea,
        results: &[Comic],
        app_state: &mut AppState,
    ) {
        // decides the right color for the results and text bar
        
        /* 
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
        frame.render_stateful_widget(results, layout[1], &mut app_state.list_state);
        */
    }
}
