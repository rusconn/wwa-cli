mod params;

use std::path::PathBuf;

use anyhow::Result;

use wwa::{BreakpointOptions, EnemiesBreakpointExt, Enemy};

use crate::commands::shared::{app::App, output_list::OutputList, tui};

use params::{Params, TogglableParam};

pub(super) struct Config {
    enemies_json5: PathBuf,
    min: Option<usize>,
    max: Option<usize>,
}

impl Config {
    pub(super) fn new(enemies_json5: PathBuf, min: Option<usize>, max: Option<usize>) -> Self {
        Self {
            enemies_json5,
            min,
            max,
        }
    }
}

pub(super) fn run(config: Config) -> Result<()> {
    let mut app: App<Params, OutputList> = App::init(
        "breakpoints",
        config.enemies_json5,
        Params {
            min: TogglableParam {
                enabled: config.min.is_some(),
                value: config.min.unwrap_or(1),
            },
            max: TogglableParam {
                enabled: config.max.is_some(),
                value: config.max.unwrap_or(1),
            },
            param_index: 0,
        },
        compute_output,
    )?;
    tui::run(&mut app)
}

fn compute_output(params: &Params, enemies: &[Enemy]) -> Result<Vec<String>, String> {
    let options = BreakpointOptions::new(
        params.min.enabled.then_some(params.min.value),
        params.max.enabled.then_some(params.max.value),
    );
    let map = enemies.breakpoints(&options);

    let mut buf = Vec::new();
    super::args::Format::Plain
        .writeln(&mut buf, &map)
        .map_err(|e| e.to_string())?;

    String::from_utf8(buf)
        .map(|s| s.lines().map(str::to_owned).collect())
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    use crate::commands::shared::focus::Focus;
    use crate::commands::shared::tui::TuiApp;

    fn app() -> App<Params, OutputList> {
        let mut app = App::new(
            PathBuf::new(),
            Vec::new(),
            Params {
                min: TogglableParam {
                    enabled: true,
                    value: 10,
                },
                max: TogglableParam {
                    enabled: false,
                    value: 5,
                },
                param_index: 0,
            },
            OutputList::new(Ok(vec!["a".to_string(), "b".to_string(), "c".to_string()])),
            compute_output,
        );
        app.output_mut()
            .set_output(Ok(vec!["a".to_string(), "b".to_string(), "c".to_string()]));
        app
    }

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    #[test]
    fn output_state_stops_at_last_item() {
        let mut app = app();
        app.set_focus(Focus::Output);

        for _ in 0..10 {
            assert!(!app.handle_key(key(KeyCode::Down)));
        }

        assert_eq!(app.output().selected(), Some(2));
    }

    #[test]
    fn t_toggles_selected_param() {
        let mut app = app();

        app.handle_key(key(KeyCode::Char('t')));
        assert!(!app.params().min.enabled);

        app.handle_key(key(KeyCode::Char('t')));
        assert!(app.params().min.enabled);
    }

    #[test]
    fn up_down_adjusts_disabled_param() {
        let mut app = app();
        app.params_mut().param_index = 1;

        app.handle_key(key(KeyCode::Up));
        assert_eq!(app.params().max.value, 6);
    }

    #[test]
    fn shift_alt_adjusts_by_step() {
        let mut app = app();

        app.handle_key(KeyEvent::new(KeyCode::Up, KeyModifiers::SHIFT));
        assert_eq!(app.params().min.value, 20);

        app.handle_key(KeyEvent::new(KeyCode::Up, KeyModifiers::ALT));
        assert_eq!(app.params().min.value, 120);

        app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::ALT));
        assert_eq!(app.params().min.value, 20);
    }

    #[test]
    fn q_quits_from_both_focuses() {
        for focus in [Focus::Params, Focus::Output] {
            let mut app = app();
            app.set_focus(focus);
            assert!(app.handle_key(key(KeyCode::Char('q'))));
        }
    }

    #[test]
    fn tab_switches_focus_and_move() {
        let mut app = app();

        app.handle_key(key(KeyCode::Tab));
        assert_eq!(app.focus(), Focus::Output);

        app.handle_key(key(KeyCode::Down));
        assert_eq!(app.output().selected(), Some(1));

        app.handle_key(key(KeyCode::Tab));
        assert_eq!(app.focus(), Focus::Params);

        app.handle_key(key(KeyCode::Up));
        assert_eq!(app.params().min.value, 11);
    }

    #[test]
    fn left_right_wraps_param_index() {
        let mut app = app();

        app.handle_key(key(KeyCode::Left));
        assert_eq!(app.params().param_index, 1);

        app.handle_key(key(KeyCode::Right));
        assert_eq!(app.params().param_index, 0);

        app.handle_key(key(KeyCode::Right));
        assert_eq!(app.params().param_index, 1);

        app.handle_key(key(KeyCode::Left));
        assert_eq!(app.params().param_index, 0);
    }

    #[test]
    fn adjust_refreshes_output() {
        let mut app = app();
        app.output_mut().set_output(Ok(vec!["dummy".to_string()]));

        app.handle_key(key(KeyCode::Up));

        assert_eq!(app.output().output(), &Ok(vec![String::new()]));
    }

    #[test]
    fn toggle_refreshes_output() {
        let mut app = app();
        app.output_mut().set_output(Ok(vec!["dummy".to_string()]));

        app.handle_key(key(KeyCode::Char('t')));

        assert_eq!(app.output().output(), &Ok(vec![String::new()]));
    }

    #[test]
    fn focus_switch_does_not_refresh_output() {
        let mut app = app();
        app.output_mut().set_output(Ok(vec!["dummy".to_string()]));

        app.handle_key(key(KeyCode::Tab));
        app.handle_key(key(KeyCode::Tab));

        assert_eq!(app.output().output(), &Ok(vec!["dummy".to_string()]));
    }
}
