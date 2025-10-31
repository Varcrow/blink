use crate::blink::{app::App, rendering::render_app, states::{main_state::MainState, state_trait::State}};
use ratatui::{crossterm::event::KeyCode, widgets::ListState, Frame};

pub struct LogState {
    list_state: ListState,
}

impl State for LogState {
    fn handle_input(self: Box<Self>, key: KeyCode, app: &mut App) -> Box<dyn State> {
        let kb = &app.config.keybindings;

        if kb.matches(key, &kb.quit) {
            return Box::new(MainState);
        }

        // scroll log list
        if kb.matches(key, &kb.move_up) {
            return self;
        }

        if kb.matches(key, &kb.move_down) {
            return self;
        }

        self
    }

    fn render(&self, app: &crate::blink::app::App, frame: &mut Frame) {
        render_app(app, frame);
    }
}
