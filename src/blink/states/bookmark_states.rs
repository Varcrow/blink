use crate::blink::{
    app::App,
    rendering::{render_app, render_bookmark_list, render_input_popup, render_input_prompt_popup},
    states::{main_state::MainState, state_trait::State},
};
use ratatui::{Frame, crossterm::event::KeyCode, widgets::ListState};

pub struct NewBookmarkState {
    pub(crate) input: String,
}
pub struct BookmarkListState {
    pub(crate) list_state: ListState,
}
pub struct DeleteBookmarkState {
    index: usize,
}

impl State for NewBookmarkState {
    fn handle_input(mut self: Box<Self>, key: KeyCode, app: &mut App) -> Box<dyn State> {
        match key {
            KeyCode::Esc => Box::new(MainState),
            KeyCode::Enter => {
                let _ = app.create_bookmark(self.input);
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
            "New bookmark".to_string(),
            format!("Name: {}", self.input),
        )
    }
}

impl State for BookmarkListState {
    fn handle_input(mut self: Box<Self>, key: KeyCode, app: &mut App) -> Box<dyn State> {
        match key {
            KeyCode::Esc | KeyCode::Char('q') => Box::new(MainState),
            KeyCode::Enter => {
                app.jump_to_bookmark(self.list_state.selected().unwrap_or_default());
                Box::new(MainState)
            }
            KeyCode::Char('d') => Box::new(DeleteBookmarkState {
                index: self.list_state.selected().unwrap_or_default(),
            }),
            KeyCode::Char('k') => {
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
                self
            }
            KeyCode::Char('j') => {
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
                self
            }
            _ => self,
        }
    }

    fn render(&self, app: &App, frame: &mut Frame) {
        render_app(app, frame);
        render_bookmark_list(app, frame, &mut self.list_state.clone());
    }
}

impl State for DeleteBookmarkState {
    fn handle_input(self: Box<Self>, key: KeyCode, app: &mut App) -> Box<dyn State> {
        match key {
            KeyCode::Esc | KeyCode::Char('n') => Box::new(MainState),
            KeyCode::Enter | KeyCode::Char('y') | KeyCode::Char('d') => {
                let _ = app.delete_bookmark(self.index);
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
            "Delete bookmark?".to_string(),
            "y / n".to_string(),
        );
    }
}
