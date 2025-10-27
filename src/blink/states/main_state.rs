use crate::blink::{
    app::{App, RunningState},
    rendering::render_app,
    states::{
        bookmark_states::{BookmarkListState, NewBookmarkState},
        delete_path_state::DeletePathState,
        new_path_state::NewPathState,
        rename_path_state::RenamePathState,
        state_trait::State,
        visual_selection_states::VisualSelectionState,
    },
};
use ratatui::{Frame, crossterm::event::KeyCode, widgets::ListState};

pub struct MainState;

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
            KeyCode::Char('o') => {
                let _ = app.open_in_default_app();
                self
            }
            KeyCode::Char('e') => {
                let _ = app.open_in_editor();
                self
            }
            KeyCode::Char('t') => {
                // TODO: Create new tab
                self
            }
            KeyCode::Char('T') => {
                // TODO: Close tab
                self
            }
            KeyCode::Char('V') => {
                app.toggle_visual_mode();
                Box::new(VisualSelectionState)
            }
            KeyCode::Char('r') => Box::new(RenamePathState {
                input: String::new(),
            }),
            KeyCode::Char('m') => Box::new(NewPathState {
                input: String::new(),
            }),
            KeyCode::Char('b') => Box::new(BookmarkListState {
                list_state: ListState::default(),
            }),
            KeyCode::Char('B') => Box::new(NewBookmarkState {
                input: String::new(),
            }),
            KeyCode::Char('d') => Box::new(DeletePathState),
            _ => self,
        }
    }

    fn render(&self, app: &App, frame: &mut Frame) {
        render_app(app, frame);
    }
}
