use std::path::PathBuf;

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    text::Span,
};

use wwa::Enemy;

use super::{
    enemies::load_enemies, focus::Focus, output_view::OutputView, params::ParamModel,
    status_bar::StatusBar, tui::TuiApp,
};

pub(crate) struct App<P: ParamModel, O: OutputView> {
    enemies_json5: PathBuf,
    enemies: Vec<Enemy>,
    params: P,
    output: O,
    focus: Focus,
    status: StatusBar,
    compute_output: fn(&P, &[Enemy]) -> Result<Vec<String>, String>,
    title: String,
}

impl<P: ParamModel, O: OutputView> TuiApp for App<P, O> {
    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::vertical([
            Constraint::Length(4),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(area);

        self.params
            .draw(frame, chunks[0], &self.title, self.focus == Focus::Params);
        self.output
            .draw(frame, chunks[1], self.focus == Focus::Output);

        let mut hints = match self.focus {
            Focus::Params => self.params.hints(),
            Focus::Output => vec![Span::raw("[↑↓] move")],
        };
        hints.push(Span::raw("[Tab] focus next"));
        hints.push(Span::raw("[r] reload"));
        hints.push(Span::raw("[q] quit"));

        self.status.draw(frame, chunks[2], &hints);
    }

    fn handle_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('q') => return true,
            KeyCode::Char('r') => self.reload(),
            KeyCode::Tab => self.next_focus(),
            _ => match self.focus {
                Focus::Params => {
                    if self.params.handle_key(key) {
                        self.refresh_output();
                    }
                }
                Focus::Output => self.output.handle_key(key),
            },
        }
        false
    }
}

impl<P: ParamModel, O: OutputView> App<P, O> {
    pub(crate) fn init(
        title_prefix: &str,
        enemies_json5: PathBuf,
        params: P,
        compute_output: fn(&P, &[Enemy]) -> Result<Vec<String>, String>,
    ) -> Result<Self> {
        let enemies = load_enemies(&enemies_json5)?;
        let output = O::new(compute_output(&params, &enemies));
        Ok(Self {
            title: format!(" {title_prefix}: {} ", enemies_json5.display()),
            enemies_json5,
            enemies,
            params,
            output,
            focus: Focus::Params,
            status: StatusBar::new(),
            compute_output,
        })
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
            self.output.on_focus();
        }
    }

    fn refresh_output(&mut self) {
        self.output
            .set_output((self.compute_output)(&self.params, &self.enemies));
    }
}

#[cfg(test)]
impl<P: ParamModel, O: OutputView> App<P, O> {
    pub(crate) fn new(
        enemies_json5: PathBuf,
        enemies: Vec<Enemy>,
        params: P,
        output: O,
        compute_output: fn(&P, &[Enemy]) -> Result<Vec<String>, String>,
    ) -> Self {
        Self {
            title: " test ".to_string(),
            enemies_json5,
            enemies,
            params,
            output,
            focus: Focus::Params,
            status: StatusBar::new(),
            compute_output,
        }
    }

    pub(crate) fn params(&self) -> &P {
        &self.params
    }

    pub(crate) fn params_mut(&mut self) -> &mut P {
        &mut self.params
    }

    pub(crate) fn output(&self) -> &O {
        &self.output
    }

    pub(crate) fn output_mut(&mut self) -> &mut O {
        &mut self.output
    }

    pub(crate) fn focus(&self) -> Focus {
        self.focus
    }

    pub(crate) fn set_focus(&mut self, focus: Focus) {
        self.focus = focus;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crossterm::event::{KeyCode, KeyModifiers};

    use super::super::tui::TuiApp;

    struct DummyParams {
        value: usize,
    }
    impl ParamModel for DummyParams {
        fn handle_key(&mut self, key: KeyEvent) -> bool {
            match key.code {
                KeyCode::Up => {
                    self.value += 1;
                    true
                }
                _ => false,
            }
        }
        fn hints(&self) -> Vec<Span<'static>> {
            vec![]
        }
        fn draw(&self, _frame: &mut Frame, _area: Rect, _title: &str, _focused: bool) {}
    }

    struct DummyOutput {
        selected: Option<usize>,
        output: Result<Vec<String>, String>,
    }
    impl OutputView for DummyOutput {
        fn new(output: Result<Vec<String>, String>) -> Self {
            Self {
                selected: None,
                output,
            }
        }
        fn set_output(&mut self, output: Result<Vec<String>, String>) {
            self.output = output;
        }
        fn handle_key(&mut self, _key: KeyEvent) {}
        fn draw(&mut self, _frame: &mut Frame, _area: Rect, _focused: bool) {}
        fn on_focus(&mut self) {
            self.selected = Some(0);
        }
    }

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    fn app() -> App<DummyParams, DummyOutput> {
        App {
            enemies_json5: PathBuf::new(),
            enemies: Vec::new(),
            params: DummyParams { value: 0 },
            output: DummyOutput {
                selected: None,
                output: Ok(vec!["dummy".to_string()]),
            },
            focus: Focus::Params,
            status: StatusBar::new(),
            compute_output: |_, _| Ok(Vec::new()),
            title: " test ".to_string(),
        }
    }

    #[test]
    fn q_quits() {
        let mut app = app();
        assert!(app.handle_key(key(KeyCode::Char('q'))));
    }

    #[test]
    fn q_quits_from_output_focus() {
        let mut app = app();
        app.handle_key(key(KeyCode::Tab));
        assert!(app.handle_key(key(KeyCode::Char('q'))));
    }

    #[test]
    fn tab_cycles_focus() {
        let mut app = app();
        assert_eq!(app.focus(), Focus::Params);

        app.handle_key(key(KeyCode::Tab));
        assert_eq!(app.focus(), Focus::Output);

        app.handle_key(key(KeyCode::Tab));
        assert_eq!(app.focus(), Focus::Params);
    }

    #[test]
    fn on_focus_on_focus_acquisition() {
        let mut app = app();
        app.output_mut().selected = Some(5);

        app.handle_key(key(KeyCode::Tab));

        assert_eq!(app.focus(), Focus::Output);
        assert_eq!(app.output().selected, Some(0));
    }

    #[test]
    fn params_change_refreshes_output() {
        let mut app = app();
        app.output_mut().output = Ok(vec!["dummy".to_string()]);

        app.handle_key(key(KeyCode::Up));

        assert_eq!(app.params().value, 1);
        assert_eq!(app.output().output, Ok(Vec::new()));
    }

    #[test]
    fn params_noop_keeps_output() {
        let mut app = app();
        app.output_mut().output = Ok(vec!["dummy".to_string()]);

        app.handle_key(key(KeyCode::Right));

        assert_eq!(app.output().output, Ok(vec!["dummy".to_string()]));
    }

    #[test]
    fn reload_error_sets_message() {
        let mut app = app();

        assert!(!app.handle_key(key(KeyCode::Char('r'))));
        assert!(app.status.message().is_some());
    }
}
