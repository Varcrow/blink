use crate::blink::{
    app::{App, RunningState},
    rendering::render_app,
    states::{
        bookmark_states::{BookmarkListState, NewBookmarkState},
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
        let kb = &app.config.keybindings;

        if kb.matches(key, &kb.quit) {
            app.running_state = RunningState::Done;
            return self;
        }
        if kb.matches(key, &kb.move_down) {
            app.move_forward_in_cwd_list();
            return self;
        }
        if kb.matches(key, &kb.move_up) {
            app.move_back_in_cwd_list();
            return self;
        }
        if kb.matches(key, &kb.go_back) {
            app.go_up_one_directory_level();
            return self;
        }
        if kb.matches(key, &kb.go_forward) {
            app.enter_current_path_selection();
            return self;
        }
        if kb.matches(key, &kb.undo) {
            app.undo_last_operation();
            return self;
        }
        if kb.matches(key, &kb.yank) {
            app.yank_current_selection(false);
            return self;
        }
        if kb.matches(key, &kb.cut) {
            app.yank_current_selection(true);
            return self;
        }
        if kb.matches(key, &kb.paste) {
            app.paste_yanked_path();
            return self;
        }
        if kb.matches(key, &kb.delete) {
            app.delete_current_selection();
            return self;
        }
        if kb.matches(key, &kb.open_default) {
            _ = app.open_in_default_app();
            return self;
        }
        if kb.matches(key, &kb.open_editor) {
            _ = app.open_in_editor();
            return self;
        }
        if kb.matches(key, &kb.visual_mode) {
            app.toggle_visual_mode();
            return Box::new(VisualSelectionState);
        }
        if kb.matches(key, &kb.rename) {
            return Box::new(RenamePathState {
                input: String::new(),
            });
        }
        if kb.matches(key, &kb.new_entry) {
            return Box::new(NewPathState {
                input: String::new(),
            });
        }
        if kb.matches(key, &kb.bookmark_new) {
            return Box::new(NewBookmarkState {
                input: String::new(),
            });
        }
        if kb.matches(key, &kb.bookmark_list) {
            return Box::new(BookmarkListState {
                list_state: ListState::default(),
            });
        }

        self
    }

    fn render(&self, app: &App, frame: &mut Frame) {
        render_app(app, frame);
    }
}
