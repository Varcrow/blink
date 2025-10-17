use crate::stfm::entries::{FileEntry, get_entries};
use ratatui::widgets::ListState;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct Model {
    pub running_state: RunningState,
    pub list_state: ListState,
    pub current_dir: PathBuf,
    pub parent_dir_entries: Vec<FileEntry>,
    pub cwd_entries: Vec<FileEntry>,
}

impl Model {
    pub fn new(path: PathBuf) -> Model {
        let mut model = Model {
            running_state: RunningState::Running,
            list_state: ListState::default(),
            current_dir: path.clone(),
            parent_dir_entries: Vec::new(),
            cwd_entries: Vec::new(),
        };

        model.update_all_entries();

        model
    }

    pub fn update_current_dir(&mut self, path: PathBuf) {
        self.current_dir = path;
        self.update_all_entries();
    }

    pub fn update_cwd_entries(&mut self) {
        self.cwd_entries = get_entries(self.current_dir.as_path()).unwrap();
    }

    pub fn update_parent_dir_entries(&mut self) {
        self.parent_dir_entries =
            get_entries(self.current_dir.as_path().parent().unwrap()).unwrap();
    }

    pub fn update_all_entries(&mut self) {
        self.update_cwd_entries();
        self.update_parent_dir_entries();
    }

    fn next(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.cwd_entries.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.cwd_entries.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn enter_selected(&mut self) {
        if let Some(i) = self.list_state.selected() {
            if let Some(entry) = self.cwd_entries.get(i) {
                if entry.is_dir {
                    self.update_current_dir(entry.path.clone());
                    self.list_state.select(Some(0));
                }
            }
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}
