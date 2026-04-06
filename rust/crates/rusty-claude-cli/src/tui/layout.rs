//! Layout definitions for the TUI.
//!
//! Defines the split-pane layout with status bar, sidebar, chat view, and input area.

use ratatui::layout::{Constraint, Direction, Layout as RatatuiLayout, Rect};

/// Main TUI layout.
#[derive(Debug, Clone)]
pub struct TuiLayout {
    /// Status bar area (top).
    pub status_bar: Rect,
    /// Sidebar area (left).
    pub sidebar: Rect,
    /// Chat view area (right).
    pub chat_view: Rect,
    /// Input area (bottom).
    pub input_area: Rect,
}

impl TuiLayout {
    /// Create a new layout from the terminal size.
    pub fn new(size: Rect) -> Self {
        // Split into status bar (top) and body (bottom)
        let chunks = RatatuiLayout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(0)])
            .split(size);

        let status_bar = chunks[0];

        // Split body into sidebar (left) and main (right)
        let body_chunks = RatatuiLayout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
            .split(chunks[1]);

        let sidebar = body_chunks[0];
        let main = body_chunks[1];

        // Split main into chat view (top) and input area (bottom)
        let main_chunks = RatatuiLayout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(main);

        let chat_view = main_chunks[0];
        let input_area = main_chunks[1];

        Self {
            status_bar,
            sidebar,
            chat_view,
            input_area,
        }
    }

    /// Create a layout with a hidden sidebar.
    pub fn without_sidebar(size: Rect) -> Self {
        // Split into status bar (top) and body (bottom)
        let chunks = RatatuiLayout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(0)])
            .split(size);

        let status_bar = chunks[0];

        // Use entire body for chat view and input
        let main_chunks = RatatuiLayout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(chunks[1]);

        let chat_view = main_chunks[0];
        let input_area = main_chunks[1];

        // Empty sidebar
        let sidebar = Rect::default();

        Self {
            status_bar,
            sidebar,
            chat_view,
            input_area,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_creation() {
        let size = Rect::new(0, 0, 80, 24);
        let layout = TuiLayout::new(size);

        assert_eq!(layout.status_bar.height, 1);
        assert!(layout.sidebar.width > 0);
        assert!(layout.chat_view.width > 0);
        assert_eq!(layout.input_area.height, 3);
    }

    #[test]
    fn test_layout_without_sidebar() {
        let size = Rect::new(0, 0, 80, 24);
        let layout = TuiLayout::without_sidebar(size);

        assert_eq!(layout.status_bar.height, 1);
        assert_eq!(layout.sidebar.width, 0);
        assert!(layout.chat_view.width > 0);
        assert_eq!(layout.input_area.height, 3);
    }
}
