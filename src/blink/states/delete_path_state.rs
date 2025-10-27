use crate::blink::{
    app::App,
    rendering::{render_app, render_input_prompt_popup},
    states::{main_state::MainState, state_trait::State},
};
use ratatui::{Frame, crossterm::event::KeyCode};

pub struct DeletePathState;

impl State for DeletePathState {
    fn handle_input(self: Box<Self>, key: KeyCode, app: &mut App) -> Box<dyn State> {
        match key {
            KeyCode::Esc | KeyCode::Char('n') => Box::new(MainState),
            KeyCode::Enter | KeyCode::Char('y') | KeyCode::Char('d') => {
                app.delete_current_selection();
                Box::new(MainState)
            }
            _ => self,
        }
    }

    fn render(&self, app: &App, frame: &mut Frame) {
        render_app(app, frame);
        render_input_prompt_popup(app, frame, "Delete?".to_string(), "y / n".to_string());
    }
}
