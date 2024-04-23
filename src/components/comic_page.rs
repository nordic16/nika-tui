use std::sync::Arc;

use crossterm::event::KeyCode;
use ratatui::prelude::*;
use ratatui::symbols::border;
use ratatui::widgets::block::*;
use ratatui::widgets::{Borders, List, ListDirection, ListState, Paragraph};
use tokio::sync::mpsc::UnboundedSender;

use crate::app::{NikaAction, Page};
use crate::helpers;
use crate::models::comic::{Chapter, Comic};
use crate::traits::{Component, Source};

pub struct ComicPage {
    action_tx: Option<UnboundedSender<NikaAction>>,
    comic: Comic,
    list_state: ListState,
    shown_chapters: Vec<Chapter>,
    page_number: u16,
    source: Arc<dyn Source>,
}

impl ComicPage {
    pub fn new(comic: Comic, source: Arc<dyn Source>) -> Self {
        let c = comic.clone();
        let chapters: Vec<Chapter> = c.chapters.into_iter().take(25).collect();

        Self {
            action_tx: None,
            comic,
            list_state: ListState::default().with_selected(Some(0)),
            shown_chapters: chapters,
            page_number: 1,
            source,
        }
    }
}

impl Component for ComicPage {
    fn init(&mut self, tx: UnboundedSender<NikaAction>) -> std::io::Result<()> {
        self.action_tx = Some(tx);

        Ok(())
    }

    fn handle_key_events(
        &mut self,
        key: crossterm::event::KeyEvent,
    ) -> std::io::Result<Option<NikaAction>> {
        match key.code {
            KeyCode::Char('q') => Ok(Some(NikaAction::Quit)),
            KeyCode::Char('s') => Ok(Some(NikaAction::ChangePage(Page::Search))),
            KeyCode::Char('h') => Ok(Some(NikaAction::ChangePage(Page::Home))),

            KeyCode::Up => {
                let selected = self.list_state.selected().unwrap_or_default();

                let index =
                    helpers::get_new_selection_index(selected, 25, ListDirection::BottomToTop);
                self.list_state.select(Some(index));

                Ok(None)
            }

            KeyCode::Down => {
                let selected = self.list_state.selected().unwrap_or_default();

                let index = helpers::get_new_selection_index(
                    selected,
                    self.shown_chapters.len(),
                    ListDirection::TopToBottom,
                );

                self.list_state.select(Some(index));

                Ok(None)
            }

            KeyCode::Right => Ok(Some(NikaAction::FetchNewChapters(true))),
            KeyCode::Left => Ok(Some(NikaAction::FetchNewChapters(false))),

            KeyCode::Enter => {
                let chapter = self.shown_chapters[self.list_state.selected().unwrap()].clone();

                Ok(Some(NikaAction::FetchChapter(chapter)))
            }

            _ => Ok(None),
        }
    }

    fn update(&mut self, action: NikaAction) -> anyhow::Result<()> {
        match action {
            NikaAction::FetchNewChapters(a) => {
                let final_page = (self.comic.chapters.len() as f32 / 25_f32).ceil() as u16;

                let new_page_number = match a {
                    true => {
                        if self.page_number == final_page {
                            self.page_number
                        } else {
                            self.page_number + 1
                        }
                    }
                    false => {
                        if self.page_number == 1 {
                            self.page_number
                        } else {
                            self.page_number - 1
                        }
                    }
                };

                let tmp = self.comic.chapters.clone();
                let chapters = tmp
                    .into_iter()
                    .skip(((new_page_number - 1) * 25) as usize)
                    .take(25)
                    .collect::<Vec<Chapter>>();

                if chapters.is_empty() {
                    return Ok(());
                }

                self.shown_chapters = chapters;
                self.list_state.select(Some(0));
                self.page_number = new_page_number;
            }

            NikaAction::SetChapters(chapters) => self.comic.chapters = chapters,

            NikaAction::FetchChapter(chap) => {
                let sender = self.action_tx.clone().unwrap();
                let source = self.source.clone();

                tokio::spawn(async move {
                    sender.send(NikaAction::ShowLoadingScreen).unwrap();

                    let _ = source.download_chapter(&chap).await.unwrap();
                });
            }

            _ => {}
        };

        Ok(())
    }

    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) {
        let info = self.comic.manga_info.as_ref().unwrap();

        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)])
            .split(rect);

        let inner_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(main_layout[0]);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_set(border::ROUNDED)
            .style(Style::new().fg(Color::Green))
            .title_alignment(Alignment::Center);

        let paragraph = Paragraph::new(Text::from(self.comic.name.to_owned().bold()))
            .centered()
            .block(block.clone());

        let more_info = Paragraph::new(vec![
            format!("Year: {}", info.date.to_string().bold()).into(),
            format!("Genres: {}", info.genres.join(", ").bold()).into(),
        ])
        .centered()
        .block(block.clone());

        let total_pages = (self.comic.chapters.len() as f32 / 25_f32).ceil();
        let tmp = format!("Chapters (Page {} of {})", self.page_number, total_pages);

        let list = self
            .shown_chapters
            .iter()
            .map(|f| Text::from(f.name.as_str()))
            .collect::<List>()
            .block(
                block
                    .title(tmp)
                    .title_bottom("◀ previous, ▲ up, ▼ down, ▶ next"),
            )
            .style(Style::new().fg(Color::White))
            .highlight_style(Style::new().fg(Color::LightGreen));

        f.render_widget(paragraph, inner_layout[0]);
        f.render_widget(more_info, inner_layout[1]);
        f.render_stateful_widget(list, main_layout[1], &mut self.list_state);
    }
}
