use std::time::Duration;

use crossterm::event::{self, Event, EventStream, KeyEvent, KeyEventKind};
use futures::{FutureExt, StreamExt};
use tokio::{sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender}, task::JoinHandle};

use crate::app::Page;

#[derive(Clone)]
pub enum NikaEvent {
    Render,
    Error,
    Tick,
    Key(KeyEvent),
}

#[derive(Debug)]
pub struct EventHandler {
  rx: UnboundedReceiver<NikaEvent>,
}

impl EventHandler {
    pub fn new() -> Self {
      let tick_delay = std::time::Duration::from_secs_f64(1.0 / 60.0);
      let render_delay = std::time::Duration::from_secs_f64(1.0 / 60.0);
        let (tx, rx) = unbounded_channel::<NikaEvent>();
        let _tx = tx.clone();
    
        tokio::spawn(async move {
          let mut reader = EventStream::new();
          let mut tick_interval = tokio::time::interval(tick_delay);
          let mut render_interval = tokio::time::interval(render_delay);
          
          loop {      
            let tick_delay = tick_interval.tick();
            let render_delay = render_interval.tick();
                      // Gets the event.
            let crossterm_event = reader.next().fuse();
            tokio::select! { // Checks the type of some event and sends it through tx.
              maybe_event = crossterm_event => {
                match maybe_event {
                  Some(Ok(evt)) => {
                    match evt {
                      Event::Key(key) => {
                        if key.kind == KeyEventKind::Press {
                          // Actually sends the event.
                          _tx.send(NikaEvent::Key(key)).unwrap();
                        }
                      },
                      _ => {},
                    }
                  }
                  Some(Err(_)) => {
                    _tx.send(NikaEvent::Error).unwrap();
                  }
                  None => {},
                }
              },

              // The following statements happen every tick_delay and render_delay 
              // seconds respectively.
              _ = tick_delay => {
                  _tx.send(NikaEvent::Tick).unwrap();
              },

              _ = render_delay => {
                _tx.send(NikaEvent::Render).unwrap();
              }
              
            }
          }
        });
    
        Self { rx }
      }
    
    
    pub async fn next(&mut self) -> Option<NikaEvent> {
        self.rx.recv().await
    }
    
}