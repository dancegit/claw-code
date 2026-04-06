use super::{
    components::Component, events::EventHandler, layout::TuiLayout, terminal::Terminal, TuiConfig,
    TuiError, TuiResult,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub struct TuiApp {
    terminal: Terminal,
    events: EventHandler,
    config: TuiConfig,
    should_quit: bool,
    layout: Option<TuiLayout>,
}

impl TuiApp {
    pub fn new(config: TuiConfig) -> TuiResult<Self> {
        let terminal = Terminal::new(&config).map_err(|e| TuiError::Terminal(e.to_string()))?;
        let events = EventHandler::new(config.tick_rate_ms);

        Ok(Self {
            terminal,
            events,
            config,
            should_quit: false,
            layout: None,
        })
    }

    pub fn run(&mut self) -> TuiResult<()> {
        while !self.should_quit {
            self.render()?;

            match self.events.next() {
                Ok(event) => self.handle_event(event)?,
                Err(_) => {
                    self.should_quit = true;
                }
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, event: super::events::Event) -> TuiResult<()> {
        match event {
            super::events::Event::Key(key_event) => self.handle_key_event(key_event)?,
            super::events::Event::Mouse(_) | super::events::Event::Resize(_, _) => {}
            super::events::Event::Tick | super::events::Event::Custom(_) => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, event: KeyEvent) -> TuiResult<()> {
        match event.code {
            KeyCode::Char('q') | KeyCode::Char('c')
                if event.modifiers.contains(KeyModifiers::CONTROL) =>
            {
                self.should_quit = true;
            }
            _ => {}
        }
        Ok(())
    }

    fn render(&mut self) -> TuiResult<()> {
        let size = self
            .terminal
            .size()
            .map_err(|e| TuiError::Render(e.to_string()))?;

        let layout = TuiLayout::new(ratatui::layout::Rect::new(0, 0, size.width, size.height));

        self.terminal
            .inner()
            .draw(|frame| draw_ui(frame, &layout))
            .map_err(|e| TuiError::Render(e.to_string()))?;

        self.layout = Some(layout);
        Ok(())
    }
}

fn draw_ui(frame: &mut ratatui::Frame, layout: &TuiLayout) {
    draw_status_bar(frame, layout.status_bar);
    draw_sidebar(frame, layout.sidebar);
    draw_chat_view(frame, layout.chat_view);
    draw_input_area(frame, layout.input_area);
}

fn draw_status_bar(frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
    let text = vec![ratatui::text::Line::from(vec![
        ratatui::text::Span::styled(
            "Claw TUI",
            ratatui::style::Style::default().fg(ratatui::style::Color::Green),
        ),
        ratatui::text::Span::raw(" | "),
        ratatui::text::Span::raw("Press Ctrl+Q to quit"),
    ])];

    frame.render_widget(
        ratatui::widgets::Paragraph::new(text)
            .style(ratatui::style::Style::default().bg(ratatui::style::Color::DarkGray))
            .block(
                ratatui::widgets::Block::default()
                    .borders(ratatui::widgets::Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded),
            ),
        area,
    );
}

fn draw_sidebar(frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
    let text = vec![
        ratatui::text::Line::from("Tasks"),
        ratatui::text::Line::from("  [ ] Task 1"),
        ratatui::text::Line::from("  [x] Task 2"),
        ratatui::text::Line::from(""),
        ratatui::text::Line::from("Files"),
        ratatui::text::Line::from("  main.rs"),
        ratatui::text::Line::from("  app.rs"),
    ];

    frame.render_widget(
        ratatui::widgets::Paragraph::new(text).block(
            ratatui::widgets::Block::default()
                .title("Sidebar")
                .borders(ratatui::widgets::Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded),
        ),
        area,
    );
}

fn draw_chat_view(frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
    let text = vec![
        ratatui::text::Line::from("Welcome to Claw TUI"),
        ratatui::text::Line::from(""),
        ratatui::text::Line::from("This is a modern terminal interface for the Claw CLI."),
        ratatui::text::Line::from(""),
        ratatui::text::Line::from(vec![ratatui::text::Span::styled(
            "Features:",
            ratatui::style::Style::default().fg(ratatui::style::Color::Cyan),
        )]),
        ratatui::text::Line::from("• Split-pane layout"),
        ratatui::text::Line::from("• Rich markdown rendering"),
        ratatui::text::Line::from("• Persistent sidebar"),
        ratatui::text::Line::from("• Multi-line input"),
    ];

    frame.render_widget(
        ratatui::widgets::Paragraph::new(text)
            .block(
                ratatui::widgets::Block::default()
                    .title("Chat")
                    .borders(ratatui::widgets::Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded),
            )
            .wrap(ratatui::widgets::Wrap { trim: true }),
        area,
    );
}

fn draw_input_area(frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
    frame.render_widget(
        ratatui::widgets::Paragraph::new("> ").block(
            ratatui::widgets::Block::default()
                .borders(ratatui::widgets::Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded),
        ),
        area,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tui_config_default() {
        let config = TuiConfig::default();
        assert!(config.mouse_support);
        assert!(config.bracketed_paste);
        assert_eq!(config.tick_rate_ms, 250);
    }
}
