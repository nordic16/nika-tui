use crossterm::event::{Event, EventStream, KeyEvent, KeyEventKind};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};

#[derive(Clone)]
pub enum NikaMessage {
    Render,
    Error,
    Key(KeyEvent),
}

#[derive(Debug)]
pub struct EventHandler {
    rx: UnboundedReceiver<NikaMessage>,
}

impl EventHandler {
    pub fn new() -> Self {
        let render_delay = std::time::Duration::from_secs_f64(1.0 / 60.0);
        let (tx, rx) = unbounded_channel::<NikaMessage>();
        let _tx = tx.clone();

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
                                        _tx.send(NikaMessage::Key(key)).unwrap();
                                    }
                                }
                            }
                            Some(Err(_)) => {
                                _tx.send(NikaMessage::Error).unwrap();
                            }
                            None => {},
                        }
                  },
                    _ = render_delay => {
                        _tx.send(NikaMessage::Render).unwrap();
                    }
                }
            }
        });

        Self { rx }
    }

    pub async fn next(&mut self) -> Option<NikaMessage> {
        self.rx.recv().await
    }
}
