use crate::stfm::{
    app::{App, RunningState},
    bookmarks::Bookmarks,
    config::Config,
    entries::{FileEntry, get_entries},
    rendering::render,
};
use color_eyre::config::Frame;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    widgets::ListState,
};
use std::{fs, path::PathBuf, time::Duration};
use std::{io, path::Path};

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

impl State for MainState {
    fn handle_input(self: Box<Self>, key: KeyCode, app: &mut App) -> Box<dyn State> {
        match key {
            KeyCode::Esc | KeyCode::Char('q') => app.running_state = RunningState::Done,
            KeyCode::Down | KeyCode::Char('j') => app.move_forward_in_cwd_list(),
            KeyCode::Up | KeyCode::Char('k') => app.move_back_in_cwd_list(),
            KeyCode::Left | KeyCode::Char('h') => app.go_up_one_directory_level(),
            KeyCode::Right | KeyCode::Char('l') => {
                app.enter_current_path_selection();
            }
            _ => {}
        }
        self
    }

    fn render(&self, app: &App, frame: &mut Frame) {
        todo!()
    }
}

impl State for DeletePathState {
    fn handle_input(self: Box<Self>, key: KeyCode, app: &mut App) -> Box<dyn State> {
        match key {
            KeyCode::Esc | KeyCode::Char('q') => app.running_state = RunningState::Done,
            KeyCode::Enter => app.delete_current_selection(),
            _ => {}
        }
        self
    }

    fn render(&self, app: &App, frame: &mut Frame) {
        todo!()
    }
}
