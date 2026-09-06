use itertools::Itertools;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

pub(crate) struct StatusBar {
    message: Option<String>,
}

impl StatusBar {
    pub(crate) fn new() -> Self {
        Self { message: None }
    }

    pub(crate) fn set_message(&mut self, message: Option<String>) {
        self.message = message;
    }

    pub(crate) fn draw(&self, frame: &mut Frame, area: Rect, hints: &[Span]) {
        let mut spans = Vec::new();

        if let Some(msg) = &self.message {
            spans.push(Span::styled(
                msg.clone(),
                Style::default().fg(Color::Yellow),
            ));
        }

        spans.extend(hints.iter().cloned());

        let spans: Vec<Span> = Itertools::intersperse(spans.into_iter(), Span::raw("  ")).collect();
        let bar = Paragraph::new(Line::from(spans));

        frame.render_widget(bar, area);
    }
}

#[cfg(test)]
impl StatusBar {
    pub(crate) fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
