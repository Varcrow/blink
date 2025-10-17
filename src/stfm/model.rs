use std::path::PathBuf;

use crate::stfm::entries::{get_entries, FileEntry};

#[derive(Debug, Default)]
pub struct Model {
    pub running_state: RunningState,
    pub current_dir: PathBuf,
    pub parent_dir_entries: Vec<FileEntry>,
    pub cwd_entries: Vec<FileEntry>,
}

impl Model {
    pub fn new(path: PathBuf) -> Model {
        let mut model = Model {
            running_state: RunningState::Running,
            current_dir: path.clone(),
            parent_dir_entries: Vec::new(),
            cwd_entries: Vec::new(),
        };
        
        model.update_all_entries();

        model
    }

    pub fn update_current_dir(&mut self, path: PathBuf) {
        self.current_dir = path;
    }

    pub fn update_cwd_entries(&mut self) {
        self.cwd_entries = get_entries(self.current_dir.as_path()).unwrap();
    }

    pub fn update_parent_dir_entries(&mut self) {
        self.parent_dir_entries = get_entries(self.current_dir.as_path().parent().unwrap()).unwrap();
    }

    pub fn update_all_entries(&mut self) {
        self.update_cwd_entries(); 
        self.update_parent_dir_entries();
    }

}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}
