use crate::blink::{
    app::App,
    rendering::{render_app, render_input_prompt_popup},
    states::{main_state::MainState, state_trait::State},
};
use ratatui::{Frame, crossterm::event::KeyCode};

pub struct VisualSelectionState;
pub struct DeleteVisualSelectionState;

impl State for VisualSelectionState {
    fn handle_input(self: Box<Self>, key: KeyCode, app: &mut App) -> Box<dyn State> {
        match key {
            KeyCode::Esc | KeyCode::Char('q') => {
                app.toggle_visual_mode();
                Box::new(MainState)
            }
            KeyCode::Char('k') => {
                app.move_back_in_cwd_list();
                self
            }
            KeyCode::Char('j') => {
                app.move_forward_in_cwd_list();
                self
            }
            KeyCode::Char('y') => {
                app.yank_current_selection(false);
                app.toggle_visual_mode();
                Box::new(MainState)
            }
            KeyCode::Char('x') => {
                app.yank_current_selection(true);
                app.toggle_visual_mode();
                Box::new(MainState)
            }
            KeyCode::Char('d') => Box::new(DeleteVisualSelectionState),
            _ => self,
        }
    }

    fn render(&self, app: &App, frame: &mut Frame) {
        render_app(app, frame);
    }
}

impl State for DeleteVisualSelectionState {
    fn handle_input(self: Box<Self>, key: KeyCode, app: &mut App) -> Box<dyn State> {
        match key {
            KeyCode::Esc | KeyCode::Char('n') => {
                app.toggle_visual_mode();
                Box::new(MainState)
            }
            KeyCode::Enter | KeyCode::Char('y') | KeyCode::Char('d') => {
                app.delete_current_selection();
                app.toggle_visual_mode();
                Box::new(MainState)
            }
            _ => self,
        }
    }

    fn render(&self, app: &App, frame: &mut Frame) {
        render_app(app, frame);
        render_input_prompt_popup(
            app,
            frame,
            "Delete selection?".to_string(),
            "y / n".to_string(),
        );
    }
}
