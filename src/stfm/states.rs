use crate::stfm::{
    app::{App, RunningState},
    rendering::{render_bookmark_list, render_confirm_delete_popup, render_input_popup, render_main_state},
};
use ratatui::{Frame, crossterm::event::KeyCode, widgets::ListState};

// State trait
pub trait State {
    fn handle_input(self: Box<Self>, key: KeyCode, app: &mut App) -> Box<dyn State>;
    fn render(&self, app: &App, frame: &mut Frame);
}

// App states
pub struct MainState;
pub struct DeletePathState;
pub struct NewPathState {
    input: String,
}
pub struct RenamePathState {
    input: String,
}
pub struct NewBookmarkState {
    input: String,
}
pub struct BookmarkListState {
    list_state: ListState,
}

impl State for MainState {
    fn handle_input(self: Box<Self>, key: KeyCode, app: &mut App) -> Box<dyn State> {
        match key {
            KeyCode::Esc | KeyCode::Char('q') => {
                app.running_state = RunningState::Done;
                self
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.move_forward_in_cwd_list();
                self
            }
            KeyCode::Up | KeyCode::Char('k') => {
                app.move_back_in_cwd_list();
                self
            }
            KeyCode::Left | KeyCode::Char('h') => {
                app.go_up_one_directory_level();
                self
            }
            KeyCode::Right | KeyCode::Char('l') => {
                app.enter_current_path_selection();
                self
            }
            KeyCode::Char('y') => {
                app.yank_current_selection(false);
                self
            }
            KeyCode::Char('x') => {
                app.yank_current_selection(true);
                self
            }
            KeyCode::Char('p') => {
                app.paste_yanked_path();
                self
            }
            KeyCode::Char('r') => Box::new(RenamePathState {
                input: String::new(),
            }),
            KeyCode::Char('m') => Box::new(NewPathState {
                input: String::new(),
            }),
            KeyCode::Char('B') => Box::new(NewBookmarkState {
                input: String::new(),
            }),
            KeyCode::Char('b') => Box::new(BookmarkListState {
                list_state: ListState::default(),
            }),
            KeyCode::Char('d') => Box::new(DeletePathState),
            _ => self,
        }
    }

    fn render(&self, app: &App, frame: &mut Frame) {
        render_main_state(app, frame);
    }
}

impl State for DeletePathState {
    fn handle_input(self: Box<Self>, key: KeyCode, app: &mut App) -> Box<dyn State> {
        match key {
            KeyCode::Esc | KeyCode::Char('n') => Box::new(MainState),
            KeyCode::Enter | KeyCode::Char('y') => {
                app.delete_current_selection();
                Box::new(MainState)
            }
            _ => self,
        }
    }

    fn render(&self, app: &App, frame: &mut Frame) {
        render_main_state(app, frame);
        render_confirm_delete_popup(frame);
    }
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
        render_main_state(app, frame);
        render_input_popup(
            frame,
            "New entry".to_string(),
            format!("Name: {}", self.input),
        )
    }
}

impl State for RenamePathState {
    fn handle_input(mut self: Box<Self>, key: KeyCode, app: &mut App) -> Box<dyn State> {
        match key {
            KeyCode::Esc => Box::new(MainState),
            KeyCode::Enter => {
                app.rename_current_selected_path(&*self.input);
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
        render_main_state(app, frame);
        render_input_popup(frame, "Rename".to_string(), format!("Name: {}", self.input))
    }
}

impl State for NewBookmarkState {
    fn handle_input(mut self: Box<Self>, key: KeyCode, app: &mut App) -> Box<dyn State> {
        match key {
            KeyCode::Esc => Box::new(MainState),
            KeyCode::Enter => {
                app.create_bookmark(self.input);
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
        render_main_state(app, frame);
        render_input_popup(
            frame,
            "New bookmark".to_string(),
            format!("Name: {}", self.input),
        )
    }
}

impl State for BookmarkListState {
    fn handle_input(mut self: Box<Self>, key: KeyCode, app: &mut App) -> Box<dyn State> {
        match key {
            KeyCode::Esc => Box::new(MainState),
            KeyCode::Enter => {
                app.jump_to_bookmark(self.list_state.selected().unwrap_or_default());
                Box::new(MainState)
            }
            KeyCode::Char('k') => {
                let i = match self.list_state.selected() {
                    Some(i) => {
                        if i == 0 {
                            app.bookmarks.list().len() - 1
                        } else {
                            i - 1
                        }
                    }
                    None => 0,
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
                    None => 0,
                };
                self.list_state.select(Some(i));
                self
            }
            _ => self,
        }
    }

    fn render(&self, app: &App, frame: &mut Frame) {
        render_main_state(app, frame);
        render_bookmark_list(app, frame, &mut self.list_state.clone());
    }
}
