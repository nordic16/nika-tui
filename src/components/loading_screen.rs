use std::io;

use crossterm::event::KeyEvent;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, BorderType, Borders, LineGauge, Paragraph};
use ratatui::{symbols, Frame};
use tokio::sync::mpsc::UnboundedSender;

use crate::app::NikaAction;
use crate::traits::Component;

#[derive(Default, Clone)]
pub struct LoadingScreen {
    percentage: Option<f64>,
    text: String,
    sender: Option<UnboundedSender<NikaAction>>,
    operation: String,
    show_gauge: bool,
}

impl LoadingScreen {
    pub fn new(progress: Option<f64>, text: &str, gauge: bool) -> Self {
        Self {
            percentage: progress,
            text: text.to_owned(),
            sender: None,
            operation: String::from(""),
            show_gauge: gauge
        }
    }
}

impl Component for LoadingScreen {
    fn init(&mut self, tx: UnboundedSender<NikaAction>) -> io::Result<()> {
        self.sender = Some(tx);
        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> io::Result<Option<NikaAction>> {
        Ok(None)
    }

    fn update(&mut self, action: crate::app::NikaAction) -> anyhow::Result<()> {
        match action {
            NikaAction::UpdateLoadingScreen(operation, pr) => {
                let mut percentage = self.percentage.unwrap_or_default();
                percentage += pr;

                self.percentage = Some(percentage);
                self.operation = operation;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) {
        let block = Block::default()
            .border_style(Style::new().light_blue())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let p = Paragraph::new(self.text.to_owned())
            .centered()
            .bold()
            .block(block);

        f.render_widget(p, rect);

        let size = rect.as_size();
        let pos = rect.as_position();
        let rect2 = Rect::new(pos.x + 2, pos.y + 2, size.width - 4, 2);

        if self.show_gauge {
            let percentage = self.percentage.unwrap_or_default();
            let gauge = LineGauge::default()
                .block(Block::default().title(self.operation.as_str()))
                .ratio(percentage)
                .gauge_style(Style::default().fg(Color::Green))
                .line_set(symbols::line::ROUNDED);

            f.render_widget(gauge, rect2);
        }
    }
}
