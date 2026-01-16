use std::time::Duration;

use crossterm::event::{Event as CrosstermEvent, EventStream, KeyEvent, KeyEventKind, MouseEvent};
use futures::StreamExt;
use tokio::sync::mpsc;

use crate::executor::CommandResult;
use crate::state::ProviderInfo;

/// Application-level events produced by the terminal or background tasks.
#[derive(Debug)]
pub enum AppEvent {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
    Tick,
    ProviderDetected(ProviderInfo),
    PromptResult(CommandResult),
    Error(String),
}

/// Asynchronous event handler built on Crossterm's EventStream.
pub struct EventHandler {
    rx: mpsc::UnboundedReceiver<AppEvent>,
    task_tx: mpsc::UnboundedSender<AppEvent>,
}

impl EventHandler {
    /// Spawn a background task that forwards terminal and tick events into a channel.
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::unbounded_channel::<AppEvent>();
        let task_tx = tx.clone();

        tokio::spawn(async move {
            let mut reader = EventStream::new();
            let mut ticker = tokio::time::interval(tick_rate);

            loop {
                let event = reader.next();
                tokio::select! {
                    maybe_event = event => {
                        match maybe_event {
                            Some(Ok(evt)) => match evt {
                                CrosstermEvent::Key(key) if key.kind == KeyEventKind::Press => {
                                    let _ = tx.send(AppEvent::Key(key));
                                }
                                CrosstermEvent::Mouse(mouse) => {
                                    let _ = tx.send(AppEvent::Mouse(mouse));
                                }
                                CrosstermEvent::Resize(w, h) => {
                                    let _ = tx.send(AppEvent::Resize(w, h));
                                }
                                _ => {}
                            },
                            Some(Err(e)) => {
                                let _ = tx.send(AppEvent::Error(format!("event error: {e}")));
                            }
                            None => break,
                        }
                    }
                    _ = ticker.tick() => {
                        let _ = tx.send(AppEvent::Tick);
                    }
                }
            }
        });

        Self { rx, task_tx }
    }

    /// Receive the next application event.
    pub async fn next(&mut self) -> Option<AppEvent> {
        self.rx.recv().await
    }

    /// Sender for background tasks to emit AppEvents.
    pub fn task_sender(&self) -> mpsc::UnboundedSender<AppEvent> {
        self.task_tx.clone()
    }
}
