use std::mem;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, List, ListItem, ListState},
};

use super::output_view::OutputView;

pub(crate) struct OutputList {
    output: Result<Vec<String>, String>,
    state: ListState,
}

impl OutputView for OutputList {
    fn new(output: Result<Vec<String>, String>) -> Self {
        OutputList::new(output)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect, focused: bool) {
        OutputList::draw(self, frame, area, focused);
    }

    fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.move_cursor(-1);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.move_cursor(1);
            }
            _ => {}
        }
    }

    fn on_focus(&mut self) {
        OutputList::select_first(self);
    }

    fn set_output(&mut self, output: Result<Vec<String>, String>) {
        OutputList::set_output(self, output);
    }
}

impl OutputList {
    pub(crate) fn new(output: Result<Vec<String>, String>) -> Self {
        Self {
            output,
            state: ListState::default(),
        }
    }

    pub(crate) fn draw(&mut self, frame: &mut Frame, area: Rect, focused: bool) {
        let items: Vec<ListItem> = match &self.output {
            Ok(lines) => lines
                .iter()
                .map(|l| ListItem::new(Line::raw(l.as_str())))
                .collect(),
            Err(e) => vec![ListItem::new(Line::raw(format!("Error: {e}")))],
        };

        let mut state = mem::take(&mut self.state);
        if focused {
            let pos = state.selected().unwrap_or(0);
            state.select(Some(pos.min(items.len().saturating_sub(1))));
        }

        let list = if focused {
            List::new(items).highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        } else {
            List::new(items)
        }
        .block(Block::bordered().title(" Output "));

        frame.render_stateful_widget(list, area, &mut state);

        self.state = state;
    }

    pub(crate) fn move_cursor(&mut self, delta: isize) {
        let count = self.output.as_ref().map(Vec::len).unwrap_or(0);
        if count == 0 {
            return;
        }
        let pos = self.state.selected().unwrap_or(0) as isize;
        let pos = (pos + delta).clamp(0, count as isize - 1) as usize;
        self.state.select(Some(pos));
    }

    pub(crate) fn select_first(&mut self) {
        self.state.select(Some(0));
    }

    pub(crate) fn set_output(&mut self, output: Result<Vec<String>, String>) {
        self.output = output;
    }
}

#[cfg(test)]
impl OutputList {
    pub(crate) fn output(&self) -> &Result<Vec<String>, String> {
        &self.output
    }

    pub(crate) fn selected(&self) -> Option<usize> {
        self.state.selected()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crossterm::event::KeyModifiers;

    fn list() -> OutputList {
        OutputList::new(Ok(vec!["a".to_string(), "b".to_string(), "c".to_string()]))
    }

    #[test]
    fn new_holds_output() {
        assert_eq!(
            list().output(),
            &Ok(vec!["a".to_string(), "b".to_string(), "c".to_string()]),
        );
    }

    #[test]
    fn set_output_replaces_content() {
        let mut list = list();
        list.set_output(Ok(vec!["x".to_string()]));
        assert_eq!(list.output(), &Ok(vec!["x".to_string()]));
    }

    #[test]
    fn move_cursor_stops_at_bounds() {
        let mut list = list();

        for _ in 0..10 {
            list.move_cursor(1);
        }
        assert_eq!(list.selected(), Some(2));

        for _ in 0..10 {
            list.move_cursor(-1);
        }
        assert_eq!(list.selected(), Some(0));
    }

    #[test]
    fn select_first_resets_position() {
        let mut list = list();
        list.move_cursor(5);
        list.select_first();
        assert_eq!(list.selected(), Some(0));
    }

    #[test]
    fn move_cursor_does_nothing_when_empty() {
        let mut list = OutputList::new(Ok(Vec::new()));
        list.move_cursor(1);
        assert_eq!(list.selected(), None);
    }

    #[test]
    fn j_moves_cursor_down() {
        let mut list = list();
        list.handle_key(KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE));
        assert_eq!(list.selected(), Some(1));
    }

    #[test]
    fn k_moves_cursor_up() {
        let mut list = list();
        list.move_cursor(2);
        list.handle_key(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
        assert_eq!(list.selected(), Some(1));
    }

    #[test]
    fn uppercase_jk_are_ignored() {
        let mut list = list();
        list.handle_key(KeyEvent::new(KeyCode::Char('J'), KeyModifiers::NONE));
        list.handle_key(KeyEvent::new(KeyCode::Char('K'), KeyModifiers::NONE));
        assert_eq!(list.selected(), None);
    }
}
