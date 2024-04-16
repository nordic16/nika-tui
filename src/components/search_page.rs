use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{block::*, Borders, List, ListDirection, ListItem, ListState},
};

use tokio::sync::mpsc::UnboundedSender;
use tui_textarea::TextArea;

use crate::{
    app::{InputMode, NikaAction, Page},
    helpers,
    models::comic::Comic,
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
    fn register_action_handler(
        &mut self,
        tx: tokio::sync::mpsc::UnboundedSender<crate::app::NikaAction>,
    ) -> std::io::Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }

    fn handle_key_events(
        &mut self,
        key: crossterm::event::KeyEvent,
    ) -> std::io::Result<Option<crate::app::NikaAction>> {
        match self.mode {
            InputMode::Normal => {
                match key.code {
                    KeyCode::Char('h') => Ok(Some(NikaAction::ChangePage(Page::Home))),
                    KeyCode::Char('q') => Ok(Some(NikaAction::Quit)),
                    KeyCode::Char('e') => {
                        self.mode = InputMode::Editing;
                        self.list_state.select(None);
                        Ok(None)
                    }

                    KeyCode::Up => {
                        let index = helpers::get_new_selection_index(
                            self.list_state.selected(),
                            self.search_results.len(),
                            ListDirection::BottomToTop,
                        );

                        self.list_state.select(Some(index));

                        Ok(None) // temporary
                    }

                    KeyCode::Down => {
                        let index = helpers::get_new_selection_index(
                            self.list_state.selected(),
                            self.search_results.len(),
                            ListDirection::TopToBottom,
                        );

                        self.list_state.select(Some(index));

                        Ok(None) // temporary
                    }

                    KeyCode::Enter => {
                        let comic = &self.search_results[self.list_state.selected().unwrap()];

                        Ok(Some(NikaAction::SelectComic(comic.to_owned())))
                    }

                    _ => Ok(None),
                }
            }
            InputMode::Editing => {
                if let KeyEventKind::Press = key.kind {
                    match key.code {
                        KeyCode::Esc => {
                            self.mode = InputMode::Normal;
                            self.list_state.select(Some(0));
                            return Ok(None);
                        }

                        KeyCode::Enter => {} // No newline lil bro

                        _ => {
                            self.text_area.input(key);
                            let query = &self.text_area.lines()[0];

                            return Ok(Some(NikaAction::SearchComic(query.to_owned())));
                        }
                    }
                }
                Ok(None)
            }
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
                        Err(_) => NikaAction::Error,
                    };
                    sender.send(message).unwrap();
                });
            }

            NikaAction::SetSearchResults(r) => self.search_results = r,

            NikaAction::SelectComic(mut c) => {
                let sender = self.action_tx.as_ref().unwrap().to_owned();
                sender.send(NikaAction::ShowLoadingScreen).unwrap();

                tokio::spawn(async move {
                    sender.send(NikaAction::ShowLoadingScreen).unwrap();

                    let chapters = helpers::get_chapters(&c).await.unwrap();
                    let info = helpers::get_comic_info(&c).await.unwrap();

                    c.manga_info = info;
                    c.chapters = chapters;

                    sender.send(NikaAction::LiftLoadingScreen).unwrap();
                    sender.send(NikaAction::ChangePage(Page::Comic(c))).unwrap();
                });
            }
            _ => {}
        }
    }

    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) {
        let (scolor, rcolor) = match self.mode {
            InputMode::Normal => (Color::default(), Color::Yellow),
            InputMode::Editing => (Color::Yellow, Color::default()),
        };

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(rect);

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

        let items = self
            .search_results
            .iter()
            .map(|f| ListItem::new(f.name.as_str()))
            .collect::<Vec<ListItem>>();

        let results = List::new(items)
            .block(block2)
            .highlight_style(Style::new().fg(Color::Yellow));

        f.render_widget(self.text_area.widget(), layout[0]);
        f.render_stateful_widget(results, layout[1], &mut self.list_state);
    }
}
