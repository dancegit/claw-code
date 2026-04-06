//! Terminal lifecycle management.
//!
//! Handles terminal initialization, raw mode, alternate screen, and cleanup.

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use std::io::{self, Stdout};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Terminal wrapper with automatic cleanup on drop.
pub struct Terminal {
    inner: ratatui::Terminal<CrosstermBackend<Stdout>>,
    _restore_on_drop: Arc<RestoreOnDrop>,
}

impl Terminal {
    /// Initialize the terminal for TUI mode.
    ///
    /// This enables raw mode, enters alternate screen, and enables mouse capture.
    pub fn new(config: &super::TuiConfig) -> io::Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        if config.mouse_support {
            execute!(stdout, EnableMouseCapture)?;
        }

        let backend = CrosstermBackend::new(stdout);
        let inner = ratatui::Terminal::new(backend)?;

        // Set up panic hook to restore terminal on panic
        set_panic_hook();

        Ok(Self {
            inner,
            _restore_on_drop: Arc::new(RestoreOnDrop::new(config.mouse_support)),
        })
    }

    /// Get the underlying ratatui terminal.
    pub fn inner(&mut self) -> &mut ratatui::Terminal<CrosstermBackend<Stdout>> {
        &mut self.inner
    }

    /// Get the current terminal size.
    pub fn size(&self) -> io::Result<TerminalSize> {
        let size = self.inner.size()?;
        Ok(TerminalSize {
            width: size.width,
            height: size.height,
        })
    }

    /// Clear the terminal screen.
    pub fn clear(&mut self) -> io::Result<()> {
        self.inner.clear()
    }

    /// Flush the terminal buffer.
    pub fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }

    /// Hide the cursor.
    pub fn hide_cursor(&mut self) -> io::Result<()> {
        self.inner.hide_cursor()
    }

    /// Show the cursor.
    pub fn show_cursor(&mut self) -> io::Result<()> {
        self.inner.show_cursor()
    }

    pub fn set_cursor(&mut self, x: u16, y: u16) -> io::Result<()> {
        use ratatui::layout::Position;
        self.inner.set_cursor_position(Position::new(x, y))
    }
}

/// Terminal size in columns and rows.
#[derive(Debug, Clone, Copy)]
pub struct TerminalSize {
    pub width: u16,
    pub height: u16,
}

/// Restore terminal state when dropped.
struct RestoreOnDrop {
    mouse_enabled: bool,
    restored: Arc<AtomicBool>,
}

impl RestoreOnDrop {
    fn new(mouse_enabled: bool) -> Self {
        Self {
            mouse_enabled,
            restored: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl Drop for RestoreOnDrop {
    fn drop(&mut self) {
        if self.restored.swap(true, Ordering::SeqCst) {
            return;
        }

        let _ = disable_raw_mode();
        let mut stdout = io::stdout();
        let _ = execute!(stdout, LeaveAlternateScreen);
        if self.mouse_enabled {
            let _ = execute!(stdout, DisableMouseCapture);
        }
    }
}

/// Set up panic hook to restore terminal on panic.
fn set_panic_hook() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        // Restore terminal
        let _ = disable_raw_mode();
        let mut stdout = io::stdout();
        let _ = execute!(stdout, LeaveAlternateScreen, DisableMouseCapture);
        // Call original panic hook
        original_hook(panic_info);
    }));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal_size() {
        let size = TerminalSize {
            width: 80,
            height: 24,
        };
        assert_eq!(size.width, 80);
        assert_eq!(size.height, 24);
    }

    #[test]
    fn test_restore_on_drop() {
        let restore = RestoreOnDrop::new(true);
        assert!(!restore.restored.load(Ordering::SeqCst));
    }
}
