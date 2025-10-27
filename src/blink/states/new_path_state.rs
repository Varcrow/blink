use crate::blink::{
    app::App,
    rendering::{render_app, render_input_popup},
    states::{main_state::MainState, state_trait::State},
};
use ratatui::{Frame, crossterm::event::KeyCode};

pub struct NewPathState {
    pub(crate) input: String,
}

impl State for NewPathState {
    fn handle_input(mut self: Box<Self>, key: KeyCode, app: &mut App) -> Box<dyn State> {
        let kb = &app.config.keybindings;

        if let KeyCode::Char(c) = key {
            self.input.push(c);
            return self;
        }
        if kb.matches(key, &vec!["backspace".to_string()]) {
            self.input.pop();
            return self;
        }
        if kb.matches(key, &vec!["enter".to_string()]) {
            app.create_file(&*self.input);
            return Box::new(MainState);
        }
        if kb.matches(key, &vec!["esc".to_string()]) {
            return Box::new(MainState);
        }

        self
    }

    fn render(&self, app: &App, frame: &mut Frame) {
        render_app(app, frame);
        render_input_popup(
            app,
            frame,
            "New entry".to_string(),
            format!("Name: {}", self.input),
        )
    }
}
