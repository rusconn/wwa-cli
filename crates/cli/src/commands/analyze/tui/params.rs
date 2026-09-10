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
            KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => {
                self.adjust(Self::adjustment_step(&key));
                true
            }
            KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => {
                self.adjust(-Self::adjustment_step(&key));
                true
            }
            KeyCode::Left | KeyCode::Char('h') => {
                self.cycle(-1);
                false
            }
            KeyCode::Right | KeyCode::Char('l') => {
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

    fn adjustment_step(key: &KeyEvent) -> isize {
        if match key.code {
            KeyCode::Char(c) => c.is_uppercase(),
            _ => key.modifiers.contains(KeyModifiers::SHIFT),
        } {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    fn key_mod(code: KeyCode, modifiers: KeyModifiers) -> KeyEvent {
        KeyEvent::new(code, modifiers)
    }

    #[test]
    fn k_increases_value() {
        let mut p = Params {
            atk: 5,
            def: 5,
            param_index: 0,
        };
        assert!(p.handle_key(key(KeyCode::Char('k'))));
        assert_eq!(p.atk, 6);
    }

    #[test]
    fn j_decreases_value() {
        let mut p = Params {
            atk: 5,
            def: 5,
            param_index: 0,
        };
        assert!(p.handle_key(key(KeyCode::Char('j'))));
        assert_eq!(p.atk, 4);
    }

    #[test]
    fn h_cycles_selection_left() {
        let mut p = Params {
            atk: 5,
            def: 5,
            param_index: 0,
        };
        assert!(!p.handle_key(key(KeyCode::Char('h'))));
        assert_eq!(p.param_index, 1);
    }

    #[test]
    fn l_cycles_selection_right() {
        let mut p = Params {
            atk: 5,
            def: 5,
            param_index: 0,
        };
        assert!(!p.handle_key(key(KeyCode::Char('l'))));
        assert_eq!(p.param_index, 1);
    }

    #[test]
    fn k_increases_by_ten() {
        let mut p = Params {
            atk: 5,
            def: 5,
            param_index: 0,
        };
        assert!(p.handle_key(key(KeyCode::Char('K'))));
        assert_eq!(p.atk, 15);
    }

    #[test]
    fn j_decreases_by_ten() {
        let mut p = Params {
            atk: 5,
            def: 5,
            param_index: 0,
        };
        assert!(p.handle_key(key(KeyCode::Char('J'))));
        assert_eq!(p.atk, 0);
    }

    #[test]
    fn char_case_decides_step_not_modifiers() {
        let mut p = Params {
            atk: 5,
            def: 5,
            param_index: 0,
        };
        assert!(p.handle_key(key_mod(KeyCode::Char('k'), KeyModifiers::SHIFT)));
        assert_eq!(p.atk, 6);
        assert!(p.handle_key(key_mod(KeyCode::Char('K'), KeyModifiers::CONTROL)));
        assert_eq!(p.atk, 16);
    }

    #[test]
    fn uppercase_h_l_are_ignored() {
        let mut p = Params {
            atk: 5,
            def: 5,
            param_index: 0,
        };
        assert!(!p.handle_key(key(KeyCode::Char('H'))));
        assert_eq!(p.param_index, 0);
        assert!(!p.handle_key(key(KeyCode::Char('L'))));
        assert_eq!(p.param_index, 0);
    }

    #[test]
    fn arrow_keys_accept_modifiers() {
        let mut p = Params {
            atk: 5,
            def: 5,
            param_index: 0,
        };
        assert!(p.handle_key(key_mod(KeyCode::Up, KeyModifiers::SHIFT)));
        assert_eq!(p.atk, 15);
    }

    #[test]
    fn arrow_keys_plain_adjust_by_one() {
        let mut p = Params {
            atk: 5,
            def: 5,
            param_index: 0,
        };
        assert!(p.handle_key(key(KeyCode::Up)));
        assert_eq!(p.atk, 6);
    }
}
