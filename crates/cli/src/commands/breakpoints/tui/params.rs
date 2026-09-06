use crossterm::event::KeyModifiers;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

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

impl Params {
    pub(super) fn draw(&self, frame: &mut Frame, area: Rect, title: String, focused: bool) {
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

    pub(super) fn toggle_selected(&mut self) {
        let param = self.selected();
        param.enabled = !param.enabled;
    }

    pub(super) fn adjust(&mut self, delta: isize) {
        let param = self.selected();
        param.value = param.value.saturating_add_signed(delta);
    }

    fn selected(&mut self) -> &mut TogglableParam {
        match self.param_index {
            0 => &mut self.min,
            _ => &mut self.max,
        }
    }

    pub(super) fn adjustment_step(modifiers: KeyModifiers) -> isize {
        if modifiers.contains(KeyModifiers::ALT) {
            100
        } else if modifiers.contains(KeyModifiers::SHIFT) {
            10
        } else {
            1
        }
    }

    const PARAM_COUNT: usize = 2;

    pub(super) fn cycle(&mut self, delta: isize) {
        let n = Self::PARAM_COUNT as isize;
        self.param_index = ((self.param_index as isize + delta).rem_euclid(n)) as usize;
    }
}
