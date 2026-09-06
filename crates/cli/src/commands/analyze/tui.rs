mod focus;
mod params;

use std::path::PathBuf;

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    text::Span,
};

use wwa::{Enemy, Player};

use crate::commands::shared::{
    enemies::load_enemies,
    output_list::OutputList,
    status_bar::StatusBar,
    tui::{self, TuiApp},
};

use {focus::Focus, params::Params};

pub(super) struct Config {
    enemies_json5: PathBuf,
    atk: usize,
    def: usize,
}

impl Config {
    pub(super) fn new(enemies_json5: PathBuf, atk: usize, def: usize) -> Self {
        Self {
            enemies_json5,
            atk,
            def,
        }
    }
}

pub(super) fn run(config: Config) -> Result<()> {
    let enemies = load_enemies(&config.enemies_json5)?;
    let params = Params {
        atk: config.atk,
        def: config.def,
        param_index: 0,
    };
    let output_list = OutputList::new(App::compute_output(params.atk, params.def, &enemies));
    let mut app = App {
        enemies_json5: config.enemies_json5,
        enemies,
        params,
        focus: Focus::Params,
        status: StatusBar::new(),
        output_list,
    };
    tui::run(&mut app)
}

struct App {
    enemies_json5: PathBuf,
    enemies: Vec<Enemy>,
    params: Params,
    focus: Focus,
    status: StatusBar,
    output_list: OutputList,
}

impl TuiApp for App {
    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::vertical([
            Constraint::Length(4),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(area);

        let title = format!(" analyze: {} ", self.enemies_json5.display());
        self.params
            .draw(frame, chunks[0], title, self.focus == Focus::Params);
        self.output_list
            .draw(frame, chunks[1], self.focus == Focus::Output);
        self.status.draw(frame, chunks[2], &self.status_hints());
    }

    fn handle_key(&mut self, key: KeyEvent) -> bool {
        match self.focus {
            Focus::Params => match key.code {
                KeyCode::Char('q') => return true,
                KeyCode::Char('r') => self.reload(),
                KeyCode::Tab => self.next_focus(),
                KeyCode::Up => self.adjust(Params::adjustment_step(key.modifiers)),
                KeyCode::Down => self.adjust(-Params::adjustment_step(key.modifiers)),
                KeyCode::Left => self.params.cycle(-1),
                KeyCode::Right => self.params.cycle(1),
                _ => {}
            },
            Focus::Output => match key.code {
                KeyCode::Char('q') => return true,
                KeyCode::Char('r') => self.reload(),
                KeyCode::Tab => self.next_focus(),
                KeyCode::Up => self.output_list.move_cursor(-1),
                KeyCode::Down => self.output_list.move_cursor(1),
                _ => {}
            },
        }

        false
    }
}

impl App {
    fn compute_output(atk: usize, def: usize, enemies: &[Enemy]) -> Result<Vec<String>, String> {
        let player = Player::new(atk, def);
        let analyses = enemies
            .iter()
            .map(|enemy| player.analyze(enemy))
            .collect::<Vec<_>>();

        let mut buf = Vec::new();
        super::args::Format::Plain
            .writeln(&mut buf, &analyses, enemies)
            .map_err(|e| e.to_string())?;

        String::from_utf8(buf)
            .map(|s| s.lines().map(str::to_owned).collect())
            .map_err(|e| e.to_string())
    }

    fn status_hints(&self) -> Vec<Span<'_>> {
        let mut hints = match self.focus {
            Focus::Params => vec![
                Span::raw("[↑↓] ±1"),
                Span::raw("[Shift+↑↓] ±10"),
                Span::raw("[Alt+↑↓] ±100"),
                Span::raw("[←→] move"),
            ],
            Focus::Output => vec![
                Span::raw("[↑↓] move"), //
            ],
        };
        hints.push(Span::raw("[Tab] focus next"));
        hints.push(Span::raw("[r] reload"));
        hints.push(Span::raw("[q] quit"));
        hints
    }

    fn reload(&mut self) {
        match self.load() {
            Ok(()) => {
                self.status.set_message(None);
                self.refresh_output();
            }
            Err(e) => self.status.set_message(Some(format!("{e:#}"))),
        }
    }

    fn load(&mut self) -> Result<()> {
        self.enemies = load_enemies(&self.enemies_json5)?;
        Ok(())
    }

    fn next_focus(&mut self) {
        self.focus = self.focus.cycle(1);
        if self.focus == Focus::Output {
            self.output_list.select_first();
        }
    }

    fn adjust(&mut self, delta: isize) {
        self.params.adjust(delta);
        self.refresh_output();
    }

    fn refresh_output(&mut self) {
        self.output_list.set_output(Self::compute_output(
            self.params.atk,
            self.params.def,
            &self.enemies,
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crossterm::event::KeyModifiers;

    fn app() -> App {
        App {
            enemies_json5: PathBuf::new(),
            enemies: Vec::new(),
            params: Params {
                atk: 10,
                def: 5,
                param_index: 0,
            },
            focus: Focus::Params,
            status: StatusBar::new(),
            output_list: OutputList::new(Ok(vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
            ])),
        }
    }

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    #[test]
    fn output_state_stops_at_last_item() {
        let mut app = app();
        app.focus = Focus::Output;

        for _ in 0..10 {
            assert!(!app.handle_key(key(KeyCode::Down)));
        }

        assert_eq!(app.output_list.selected(), Some(2));
    }

    #[test]
    fn up_down_adjusts_selected_value() {
        let mut app = app();

        app.handle_key(key(KeyCode::Up));
        assert_eq!(app.params.atk, 11);

        app.handle_key(key(KeyCode::Down));
        assert_eq!(app.params.atk, 10);

        app.handle_key(key(KeyCode::Right));
        assert_eq!(app.params.param_index, 1);

        app.handle_key(key(KeyCode::Up));
        assert_eq!(app.params.def, 6);
    }

    #[test]
    fn shift_alt_adjusts_by_step() {
        let mut app = app();

        app.handle_key(KeyEvent::new(KeyCode::Up, KeyModifiers::SHIFT));
        assert_eq!(app.params.atk, 20);

        app.handle_key(KeyEvent::new(KeyCode::Up, KeyModifiers::ALT));
        assert_eq!(app.params.atk, 120);

        app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::ALT));
        assert_eq!(app.params.atk, 20);
    }

    #[test]
    fn q_quits_from_both_focuses() {
        for focus in [Focus::Params, Focus::Output] {
            let mut app = app();
            app.focus = focus;
            assert!(app.handle_key(key(KeyCode::Char('q'))));
        }
    }

    #[test]
    fn tab_switches_focus_and_move() {
        let mut app = app();

        app.handle_key(key(KeyCode::Tab));
        assert_eq!(app.focus, Focus::Output);

        app.handle_key(key(KeyCode::Down));
        assert_eq!(app.output_list.selected(), Some(1));

        app.handle_key(key(KeyCode::Tab));
        assert_eq!(app.focus, Focus::Params);

        app.handle_key(key(KeyCode::Up));
        assert_eq!(app.params.atk, 11);
    }

    #[test]
    fn left_right_wraps_param_index() {
        let mut app = app();

        app.handle_key(key(KeyCode::Left));
        assert_eq!(app.params.param_index, 1);

        app.handle_key(key(KeyCode::Right));
        assert_eq!(app.params.param_index, 0);

        app.handle_key(key(KeyCode::Right));
        assert_eq!(app.params.param_index, 1);

        app.handle_key(key(KeyCode::Left));
        assert_eq!(app.params.param_index, 0);
    }

    #[test]
    fn reload_failure_sets_message() {
        let mut app = app();

        assert!(!app.handle_key(key(KeyCode::Char('r'))));
        assert!(app.status.message().is_some());
    }

    #[test]
    fn adjust_refreshes_output() {
        let mut app = app();
        app.output_list.set_output(Ok(vec!["dummy".to_string()]));

        app.handle_key(key(KeyCode::Up));

        assert_eq!(app.output_list.output(), &Ok(vec![String::new()]));
    }

    #[test]
    fn focus_switch_does_not_refresh_output() {
        let mut app = app();
        app.output_list.set_output(Ok(vec!["dummy".to_string()]));

        app.handle_key(key(KeyCode::Tab));
        app.handle_key(key(KeyCode::Tab));

        assert_eq!(app.output_list.output(), &Ok(vec!["dummy".to_string()]),);
    }
}
