use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};

pub(crate) trait OutputView: Sized {
    fn new(output: Result<Vec<String>, String>) -> Self;

    fn draw(&mut self, frame: &mut Frame, area: Rect, focused: bool);

    fn handle_key(&mut self, key: KeyEvent);

    fn on_focus(&mut self) {}

    fn set_output(&mut self, output: Result<Vec<String>, String>);
}
