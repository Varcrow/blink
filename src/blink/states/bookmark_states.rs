use crate::blink::{
    app::App,
    rendering::{render_app, render_bookmark_list, render_input_popup, render_input_prompt_popup},
    states::{main_state::MainState, state_trait::State},
};
use ratatui::{Frame, crossterm::event::KeyCode, widgets::ListState};

pub struct NewBookmarkState {
    pub input: String,
}
pub struct BookmarkListState {
    pub list_state: ListState,
}
pub struct DeleteBookmarkState {
    index: usize,
}

impl State for NewBookmarkState {
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
            let _ = app.create_bookmark(self.input);
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
            "New bookmark".to_string(),
            format!("Name: {}", self.input),
        )
    }
}

impl State for BookmarkListState {
    fn handle_input(mut self: Box<Self>, key: KeyCode, app: &mut App) -> Box<dyn State> {
        let kb = &app.config.keybindings;

        if kb.matches(key, &vec!["enter".to_string()]) {
            app.jump_to_bookmark(self.list_state.selected().unwrap_or_default());
            return Box::new(MainState);
        }
        if kb.matches(key, &vec!["esc".to_string()]) {
            return Box::new(MainState);
        }
        if kb.matches(key, &kb.delete) {
            return Box::new(DeleteBookmarkState {
                index: self.list_state.selected().unwrap_or_default(),
            });
        }
        if kb.matches(key, &kb.move_up) {
            let i = match self.list_state.selected() {
                Some(i) => {
                    if i == 0 {
                        app.bookmarks.list().len() - 1
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
                    if i >= app.bookmarks.list().len() - 1 {
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

    fn render(&self, app: &App, frame: &mut Frame) {
        render_app(app, frame);
        render_bookmark_list(app, frame, &mut self.list_state.clone());
    }
}

impl State for DeleteBookmarkState {
    fn handle_input(self: Box<Self>, key: KeyCode, app: &mut App) -> Box<dyn State> {
        let kb = &app.config.keybindings;

        if kb.matches(key, &vec!["esc".to_string(), "n".to_string()]) {
            return Box::new(MainState);
        }
        if kb.matches(key, &vec!["enter".to_string(), "y".to_string()]) {
                let _ = app.delete_bookmark(self.index);
            return Box::new(MainState);
        }

        return self;
    }

    fn render(&self, app: &App, frame: &mut Frame) {
        render_app(app, frame);
        render_input_prompt_popup(
            app,
            frame,
            "Delete bookmark?".to_string(),
            "y / n".to_string(),
        );
    }
}
