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
        let kb = &app.config.keybindings;

        if kb.matches(key, &vec!["esc".to_string()]) {
            app.toggle_visual_mode();
            return Box::new(MainState);
        }
        if kb.matches(key, &kb.move_down) {
            app.move_forward_in_cwd_list();
            return self;
        }
        if kb.matches(key, &kb.move_up) {
            app.move_back_in_cwd_list();
            return self;
        }
        if kb.matches(key, &kb.yank) {
            app.yank_current_selection(false);
            app.toggle_visual_mode();
            return Box::new(MainState);
        }
        if kb.matches(key, &kb.cut) {
            app.yank_current_selection(true);
            app.toggle_visual_mode();
            return Box::new(MainState);
        }
        if kb.matches(key, &kb.delete) {
            return Box::new(DeleteVisualSelectionState);
        }

        self
    }

    fn render(&self, app: &App, frame: &mut Frame) {
        render_app(app, frame);
    }
}

impl State for DeleteVisualSelectionState {
    fn handle_input(self: Box<Self>, key: KeyCode, app: &mut App) -> Box<dyn State> {
        let kb = &app.config.keybindings;

        if kb.matches(key, &vec!["esc".to_string(), "n".to_string()]) {
            app.toggle_visual_mode();
            return Box::new(MainState);
        }
        if kb.matches(key, &vec!["enter".to_string(), "y".to_string()]) {
            app.delete_current_selection();
            app.toggle_visual_mode();
            return Box::new(MainState);
        }

        self
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
