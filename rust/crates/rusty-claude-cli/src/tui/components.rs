use ratatui::Frame;

pub trait Component {
    fn render(&self, frame: &mut Frame, area: ratatui::layout::Rect);

    fn handle_key_event(&mut self, event: &crossterm::event::KeyEvent) -> bool {
        let _ = event;
        false
    }

    fn handle_mouse_event(&mut self, event: &crossterm::event::MouseEvent) -> bool {
        let _ = event;
        false
    }

    fn on_tick(&mut self) {}
}

#[derive(Debug, Default, Clone)]
pub struct EmptyComponent;

impl Component for EmptyComponent {
    fn render(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        frame.render_widget(ratatui::widgets::Clear, area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    #[test]
    fn test_empty_component() {
        let mut component = EmptyComponent::default();
        let key_event = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);
        assert!(!component.handle_key_event(&key_event));
    }

    #[test]
    fn test_component_default_implementations() {
        let mut component = EmptyComponent::default();
        component.on_tick();
    }
}
