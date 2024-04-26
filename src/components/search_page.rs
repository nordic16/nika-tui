use std::io;
use std::sync::Arc;

use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::prelude::*;
use ratatui::widgets::block::*;
use ratatui::widgets::{Borders, List, ListDirection, ListItem, ListState};
use tokio::sync::mpsc::UnboundedSender;
use tui_textarea::TextArea;

use crate::app::{InputMode, NikaAction, Page};
use crate::helpers;
use crate::models::comic::Comic;
use crate::models::sources::mangapill::MangapillSource;
use crate::traits::{Component, Source};

#[derive(Default)]
pub struct SearchPage {
    action_tx: Option<UnboundedSender<NikaAction>>,
    search_results: Vec<Comic>,
    text_area: TextArea<'static>,
    mode: InputMode,
    list_state: ListState,
    sources: Vec<Arc<dyn Source>>,
    selected_source_index: usize,
}

impl Component for SearchPage {
    fn init(&mut self, tx: UnboundedSender<NikaAction>) -> io::Result<()> {
        self.action_tx = Some(tx);
        let mut vec: Vec<Arc<dyn Source>> = vec![
            Arc::new(MangapillSource::new()),
            // Arc::new(MangaseeSource::new()),
        ];
        self.sources.append(&mut vec);

        Ok(())
    }

    fn handle_key_events(&mut self, key: event::KeyEvent) -> io::Result<Option<NikaAction>> {
        match self.mode {
            InputMode::Normal => {
                match key.code {
                    KeyCode::Char('h') => Ok(Some(NikaAction::ChangePage(Page::Home))),
                    KeyCode::Char('q') => Ok(Some(NikaAction::Quit)),
                    KeyCode::Char('/') => {
                        self.mode = InputMode::Editing;
                        self.list_state.select(None);
                        Ok(None)
                    }

                    KeyCode::Char('s') => {
                        self.selected_source_index += 1;

                        if self.selected_source_index == self.sources.len() {
                            self.selected_source_index = 0;
                        }
                        Ok(None)
                    }

                    KeyCode::Up => {
                        let selected = self.list_state.selected().unwrap_or_default();
                        let index = helpers::get_new_selection_index(
                            selected,
                            self.search_results.len(),
                            ListDirection::BottomToTop,
                        );

                        self.list_state.select(Some(index));

                        Ok(None) // temporary
                    }

                    KeyCode::Down => {
                        let selected = self.list_state.selected().unwrap_or_default();
                        let index = helpers::get_new_selection_index(
                            selected,
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
                        KeyCode::Enter => {
                            self.mode = InputMode::Normal;
                            self.list_state.select(Some(0));
                            return Ok(None);
                        }

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

    fn update(&mut self, action: NikaAction) -> anyhow::Result<()> {
        match action {
            NikaAction::SearchComic(query) => {
                let sender = self.action_tx.clone().unwrap();
                let s = self.sources[self.selected_source_index].clone();

                tokio::spawn(async move {
                    let results = s.search(&query).await;

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
                sender.send(NikaAction::ChangePage(Page::LoadingScreen(
                    "Loading Comics...",
                    None,
                )))?;
                let source = self.sources[self.selected_source_index].clone();

                tokio::spawn(async move {
                    let chapters = source.get_chapters(&c).await.unwrap();
                    let info = source.get_info(&c).await.unwrap();

                    c.chapters = chapters;

                    sender
                        .send(NikaAction::ChangePage(Page::Comic(
                            c,
                            source,
                            info.unwrap(),
                        )))
                        .unwrap();
                });
            }
            _ => {}
        }

        Ok(())
    }

    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) {
        let (scolor, rcolor) = match self.mode {
            InputMode::Normal => (Color::default(), Color::Yellow),
            InputMode::Editing => (Color::Yellow, Color::default()),
        };

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(3), Constraint::Fill(2)])
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
            .title_alignment(Alignment::Center)
            .title_bottom("</> to edit and <Enter> to stop editing");

        let items = self
            .search_results
            .iter()
            .map(|f| ListItem::new(f.name.as_str()))
            .collect::<Vec<ListItem>>();

        let source = Text::from(format!(
            "Source: {}",
            self.sources[self.selected_source_index].name()
        ))
        .centered();

        let results = List::new(items)
            .block(block2)
            .highlight_style(Style::new().fg(Color::Yellow));

        f.render_widget(self.text_area.widget(), layout[0]);
        f.render_widget(source, layout[0]);
        f.render_stateful_widget(results, layout[1], &mut self.list_state);
    }
}
