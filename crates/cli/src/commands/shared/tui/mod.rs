pub(crate) mod app;
pub(crate) mod focus;
pub(crate) mod output_list;
pub(crate) mod output_view;
pub(crate) mod params;
pub(crate) mod status_bar;

use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{DefaultTerminal, Frame, layout::Rect};

pub(crate) trait TuiApp {
    /// Draw the whole frame. `area` is the full terminal area.
    fn draw(&mut self, frame: &mut Frame, area: Rect);

    /// Handle one key event (press or repeat). Return `true` to quit the session.
    fn handle_key(&mut self, key: KeyEvent) -> bool;
}

pub(crate) fn run(app: &mut impl TuiApp) -> Result<()> {
    let _guard = TerminalGuard;
    let mut terminal = ratatui::try_init()?;
    run_loop(&mut terminal, app)
}

struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        ratatui::restore();
    }
}

fn run_loop(terminal: &mut DefaultTerminal, app: &mut impl TuiApp) -> Result<()> {
    loop {
        terminal.draw(|frame| app.draw(frame, frame.area()))?;
        let event = crossterm::event::read()?;
        let Some(key) = press_or_repeat(&event) else {
            continue;
        };
        if is_ctrl_c(key) || app.handle_key(key) {
            break Ok(());
        }
    }
}

fn press_or_repeat(event: &Event) -> Option<KeyEvent> {
    match event {
        Event::Key(key) if matches!(key.kind, KeyEventKind::Press | KeyEventKind::Repeat) => {
            Some(*key)
        }
        _ => None,
    }
}

fn is_ctrl_c(key: KeyEvent) -> bool {
    matches!(
        key,
        KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    #[test]
    fn ctrl_c_detects_control_c() {
        let key = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
        assert!(is_ctrl_c(key));
    }

    #[test]
    fn ctrl_c_ignores_plain_c() {
        let key = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE);
        assert!(!is_ctrl_c(key));
    }

    #[test]
    fn ctrl_c_ignores_other_keys() {
        let key = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::CONTROL);
        assert!(!is_ctrl_c(key));
    }

    #[test]
    fn press_or_repeat_accepts_press_and_repeat() {
        let press = Event::Key(KeyEvent::new_with_kind(
            KeyCode::Up,
            KeyModifiers::NONE,
            KeyEventKind::Press,
        ));
        assert_eq!(
            press_or_repeat(&press),
            Some(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE))
        );

        let repeat = Event::Key(KeyEvent::new_with_kind(
            KeyCode::Up,
            KeyModifiers::NONE,
            KeyEventKind::Repeat,
        ));
        assert_eq!(
            press_or_repeat(&repeat),
            Some(KeyEvent::new_with_kind(
                KeyCode::Up,
                KeyModifiers::NONE,
                KeyEventKind::Repeat,
            ))
        );

        let release = Event::Key(KeyEvent::new_with_kind(
            KeyCode::Up,
            KeyModifiers::NONE,
            KeyEventKind::Release,
        ));
        assert_eq!(press_or_repeat(&release), None);

        assert_eq!(press_or_repeat(&Event::Resize(80, 24)), None);
    }
}
