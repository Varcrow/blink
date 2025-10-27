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
        match key {
            KeyCode::Esc => Box::new(MainState),
            KeyCode::Enter => {
                app.new_path(&*self.input);
                Box::new(MainState)
            }
            KeyCode::Char(c) => {
                self.input.push(c);
                self
            }
            KeyCode::Backspace => {
                self.input.pop();
                self
            }
            _ => self,
        }
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
