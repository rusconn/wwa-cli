use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use crate::commands::shared::params::ParamModel;

#[derive(Clone, Copy)]
pub(super) struct TogglableParam {
    pub(super) enabled: bool,
    pub(super) value: usize,
}

pub(super) struct Params {
    pub(super) min: TogglableParam,
    pub(super) max: TogglableParam,
    pub(super) param_index: usize,
}

impl ParamModel for Params {
    fn draw(&self, frame: &mut Frame, area: Rect, title: &str, focused: bool) {
        Params::draw(self, frame, area, title, focused);
    }

    fn handle_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('t') => {
                self.toggle_selected();
                true
            }
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
            Span::raw("[t] toggle"),
        ]
    }
}

impl Params {
    fn draw(&self, frame: &mut Frame, area: Rect, title: &str, focused: bool) {
        let mut spans = Vec::new();

        for (i, (name, param)) in [
            ("MIN", self.min), //
            ("MAX", self.max),
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
                format!("[{}]", param.value),
                if param.enabled {
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default() //
                        .fg(Color::Gray)
                        .add_modifier(Modifier::DIM)
                },
            ));
        }

        let params = Paragraph::new(Line::from(spans)).block(Block::bordered().title(title));

        frame.render_widget(params, area);
    }

    fn toggle_selected(&mut self) {
        let param = self.selected();
        param.enabled = !param.enabled;
    }

    fn adjust(&mut self, delta: isize) {
        let param = self.selected();
        param.value = param.value.saturating_add_signed(delta);
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

    fn selected(&mut self) -> &mut TogglableParam {
        match self.param_index {
            0 => &mut self.min,
            _ => &mut self.max,
        }
    }

    const PARAM_COUNT: usize = 2;

    fn cycle(&mut self, delta: isize) {
        let n = Self::PARAM_COUNT as isize;
        self.param_index = ((self.param_index as isize + delta).rem_euclid(n)) as usize;
    }
}
