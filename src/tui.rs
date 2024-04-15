use std::{
    io::{self, stdout},
    panic::{set_hook, take_hook},
};

use crossterm::{
    cursor,
    event::{Event, EventStream, KeyEvent, KeyEventKind},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::{FutureExt, StreamExt};
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

#[derive(Clone)]
pub enum NikaEvent {
    Quit,
    Error,
    Closed,
    Render,
    FocusGained,
    FocusLost,
    Paste(String),
    Key(KeyEvent),
    Resize(u16, u16),
}

#[derive(Debug)]
pub struct Tui {
    pub terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    pub event_rx: UnboundedReceiver<NikaEvent>,
    pub event_tx: UnboundedSender<NikaEvent>,
    pub framerate: f64,
}

impl Tui {
    pub fn new() -> io::Result<Self> {
        let framerate = 60.0;

        let (event_tx, event_rx) = unbounded_channel::<NikaEvent>();
        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

        Ok(Self {
            terminal,
            event_rx,
            event_tx,
            framerate,
        })
    }

    // Panics properly.
    pub fn init_panic_hook(&self) {
        let original_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            // intentionally ignore errors here since we're already in a panic
            let _ = Self::restore();
            original_hook(panic_info);
        }));
    }

    pub fn run(&mut self) -> io::Result<()> {
        let render_delay = std::time::Duration::from_secs_f64(1.0 / 60.0);
        let _tx = self.event_tx.clone();

        Self::init()?;

        tokio::spawn(async move {
            let mut reader = EventStream::new();
            let mut render_interval = tokio::time::interval(render_delay);

            loop {
                let render_delay = render_interval.tick();
                // Gets the event.
                let crossterm_event = reader.next().fuse();
                tokio::select! { // Checks the type of some event and sends it through tx.
                    maybe_event = crossterm_event => {
                        match maybe_event {
                            Some(Ok(evt)) => {
                                if let Event::Key(key) = evt {
                                    if key.kind == KeyEventKind::Press {
                                        // Actually sends the event.
                                        _tx.send(NikaEvent::Key(key)).expect("Couldn't send input key");
                                    }
                                }
                            }
                            Some(Err(e)) => {
                                _tx.send(NikaEvent::Error).expect(format!("Error! {}", e).as_str());
                            }
                            None => {},
                        }
                  },
                    _ = render_delay => {
                        _tx.send(NikaEvent::Render).unwrap();
                    }
                }
            }
        });
        Ok(())
    }

    pub async fn next(&mut self) -> Option<NikaEvent> {
        self.event_rx.recv().await
    }

    pub fn init() -> io::Result<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(std::io::stderr(), EnterAlternateScreen, cursor::Hide)?;

        Ok(())
    }

    /// Restore the terminal to its original state
    pub fn restore() -> io::Result<()> {
        execute!(stdout(), LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;

        Ok(())
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        Self::restore().unwrap();
    }
}
