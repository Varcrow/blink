use crate::blink::{
    app::App,
    rendering::{render_app, render_log_list},
    states::{main_state::MainState, state_trait::State},
};
use ratatui::{Frame, crossterm::event::KeyCode, widgets::ListState};

pub struct LogState {
    pub list_state: ListState,
}

impl State for LogState {
    fn handle_input(mut self: Box<Self>, key: KeyCode, app: &mut App) -> Box<dyn State> {
        let kb = &app.config.keybindings;

        if kb.matches(key, &kb.quit) {
            return Box::new(MainState);
        }

        if kb.matches(key, &kb.move_up) {
            let i = match self.list_state.selected() {
                Some(i) => {
                    if i == 0 {
                        app.log_manager.session_logs.len() - 1
                    } else {
                        i - 1
                    }
                }
                _ => 0,
            };
            self.list_state.select(Some(i));
            return self;
        }

        if kb.matches(key, &kb.move_down) {
            let i = match self.list_state.selected() {
                Some(i) => {
                    if i >= app.log_manager.session_logs.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                _ => 0,
            };
            self.list_state.select(Some(i));
            return self;
        }

        self
    }

    fn render(&self, app: &crate::blink::app::App, frame: &mut Frame) {
        render_app(app, frame);
        render_log_list(app, frame, &mut self.list_state.clone());
    }
}
