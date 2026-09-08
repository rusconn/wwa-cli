use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect, text::Span};

pub(crate) trait ParamModel {
    /// Draw the params pane.
    fn draw(&self, frame: &mut Frame, area: Rect, title: &str, focused: bool);

    /// Handle a key event. Returns `true` if a value changed (output needs refresh).
    fn handle_key(&mut self, key: KeyEvent) -> bool;

    /// Hints specific to this param model (shown in the params focus).
    fn hints(&self) -> Vec<Span<'static>>;
}
