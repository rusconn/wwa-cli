use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use crate::commands::shared::tui::params::ParamModel;

pub(super) struct Params {
    pub(super) atk: usize,
    pub(super) def: usize,
    pub(super) param_index: usize,
}

impl ParamModel for Params {
    fn draw(&self, frame: &mut Frame, area: Rect, title: &str, focused: bool) {
        Params::draw(self, frame, area, title, focused);
    }

    fn handle_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Up => {
                self.adjust(Self::adjustment_step(key.modifiers));
                true
            }
            KeyCode::Down => {
                self.adjust(-Self::adjustment_step(key.modifiers));
                true
            }
            KeyCode::Left => {
                self.cycle(-1);
                false
            }
            KeyCode::Right => {
                self.cycle(1);
                false
            }
            _ => false,
        }
    }

    fn hints(&self) -> Vec<Span<'static>> {
        vec![
            Span::raw("[↑↓] ±1"),
            Span::raw("[Shift+↑↓] ±10"),
            Span::raw("[Alt+↑↓] ±100"),
            Span::raw("[←→] move"),
        ]
    }
}

impl Params {
    fn draw(&self, frame: &mut Frame, area: Rect, title: &str, focused: bool) {
        let mut spans = Vec::new();

        for (i, (name, value)) in [
            ("ATK", self.atk), //
            ("DEF", self.def),
        ]
        .iter()
        .enumerate()
        {
            if i > 0 {
                spans.push(Span::raw("  "));
            }

            let selected = focused && self.param_index == i;

            spans.push(Span::styled(
                format!("{}{name}: ", if selected { "> " } else { "  " }),
                if selected {
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                },
            ));

            spans.push(Span::styled(
                format!("[{value}]"),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ));
        }

        let params = Paragraph::new(Line::from(spans)).block(Block::bordered().title(title));

        frame.render_widget(params, area);
    }

    fn adjust(&mut self, delta: isize) {
        let value = self.selected();
        *value = value.saturating_add_signed(delta);
    }

    fn adjustment_step(modifiers: KeyModifiers) -> isize {
        if modifiers.contains(KeyModifiers::ALT) {
            100
        } else if modifiers.contains(KeyModifiers::SHIFT) {
            10
        } else {
            1
        }
    }

    fn selected(&mut self) -> &mut usize {
        match self.param_index {
            0 => &mut self.atk,
            _ => &mut self.def,
        }
    }

    const PARAM_COUNT: usize = 2;

    fn cycle(&mut self, delta: isize) {
        let n = Self::PARAM_COUNT as isize;
        self.param_index = ((self.param_index as isize + delta).rem_euclid(n)) as usize;
    }
}
