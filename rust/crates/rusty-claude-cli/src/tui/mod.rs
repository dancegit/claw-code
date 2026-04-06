//! TUI module for ratatui-based terminal interface.
//!
//! This module provides a modern, full-screen TUI experience with:
//! - Split-pane layout (sidebar | chat view | input area)
//! - Rich message rendering with streaming markdown
//! - Persistent sidebar for tasks, files, and agents
//! - Live status bar with model, tokens, cost, session info
//! - Multi-line input with slash command support

mod app;
mod components;
mod events;
mod layout;
mod terminal;

pub use app::TuiApp;
pub use components::Component;
pub use events::{Event, EventHandler};
pub use layout::TuiLayout;
pub use terminal::{Terminal, TerminalSize};

/// TUI configuration options.
#[derive(Debug, Clone)]
pub struct TuiConfig {
    /// Enable mouse support.
    pub mouse_support: bool,
    /// Enable bracketed paste.
    pub bracketed_paste: bool,
    /// Tick rate for event loop (ms).
    pub tick_rate_ms: u64,
}

impl Default for TuiConfig {
    fn default() -> Self {
        Self {
            mouse_support: true,
            bracketed_paste: true,
            tick_rate_ms: 250,
        }
    }
}

/// Result type for TUI operations.
pub type TuiResult<T> = std::result::Result<T, TuiError>;

/// Errors that can occur in the TUI.
#[derive(Debug, thiserror::Error)]
pub enum TuiError {
    #[error("Terminal error: {0}")]
    Terminal(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Event loop error: {0}")]
    EventLoop(String),

    #[error("Render error: {0}")]
    Render(String),
}
