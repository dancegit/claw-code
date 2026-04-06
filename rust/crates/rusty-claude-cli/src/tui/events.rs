//! Async event handling for the TUI.
//!
//! Provides an event loop that handles keyboard, mouse, resize, and custom events.

use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Events that can be handled by the TUI.
#[derive(Debug, Clone)]
pub enum Event {
    /// Key event.
    Key(KeyEvent),
    /// Mouse event.
    Mouse(MouseEvent),
    /// Terminal resize event.
    Resize(u16, u16),
    /// Tick event (periodic).
    Tick,
    /// Custom event.
    Custom(String),
}

/// Async event handler.
pub struct EventHandler {
    sender: mpsc::Sender<Event>,
    receiver: mpsc::Receiver<Event>,
    _handle: thread::JoinHandle<()>,
}

impl EventHandler {
    /// Create a new event handler.
    pub fn new(tick_rate_ms: u64) -> Self {
        let (sender, receiver) = mpsc::channel();
        let tick_rate = Duration::from_millis(tick_rate_ms);

        let thread_sender = sender.clone();
        let handle = thread::spawn(move || {
            let mut last_tick = std::time::Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));

                if event::poll(timeout).is_ok() {
                    if let Ok(evt) = event::read() {
                        let event = match evt {
                            CrosstermEvent::Key(key) => Event::Key(key),
                            CrosstermEvent::Mouse(mouse) => Event::Mouse(mouse),
                            CrosstermEvent::Resize(width, height) => Event::Resize(width, height),
                            _ => continue,
                        };
                        if thread_sender.send(event).is_err() {
                            return;
                        }
                    }
                }

                if last_tick.elapsed() >= tick_rate {
                    if thread_sender.send(Event::Tick).is_err() {
                        return;
                    }
                    last_tick = std::time::Instant::now();
                }
            }
        });

        Self {
            sender,
            receiver,
            _handle: handle,
        }
    }

    /// Get the next event.
    pub fn next(&self) -> Result<Event, mpsc::RecvError> {
        self.receiver.recv()
    }

    /// Send a custom event.
    pub fn send_custom(&self, message: String) -> Result<(), mpsc::SendError<Event>> {
        self.sender.send(Event::Custom(message))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_creation() {
        let event = Event::Tick;
        assert!(matches!(event, Event::Tick));
    }

    #[test]
    fn test_event_handler_creation() {
        let handler = EventHandler::new(250);
        // Event handler is created successfully
        drop(handler);
    }
}
