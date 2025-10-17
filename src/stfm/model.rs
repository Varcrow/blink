use crate::stfm::entries::{FileEntry, get_entries};
use ratatui::widgets::ListState;
use std::{fs, path::PathBuf};

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Debug)]
pub enum DirPreview {
    File { contents: String },
    Directory { entries: Vec<FileEntry> },
}

impl Default for DirPreview {
    fn default() -> Self {
        Self::File {
            contents: String::new(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Model {
    pub running_state: RunningState,
    pub list_state: ListState,
    pub current_dir: PathBuf,
    pub parent_dir_entries: Vec<FileEntry>,
    pub cwd_entries: Vec<FileEntry>,
    pub dir_preview: DirPreview,
}

impl Model {
    pub fn new(path: PathBuf) -> Model {
        let mut model = Model {
            running_state: RunningState::Running,
            list_state: ListState::default(),
            current_dir: path.clone(),
            parent_dir_entries: Vec::new(),
            cwd_entries: Vec::new(),
            dir_preview: DirPreview::File {
                contents: String::new(),
            },
        };
        model.update_all_entries();
        model
    }

    pub fn update_cwd(&mut self, path: PathBuf) {
        self.current_dir = path;
        self.update_all_entries();
    }

    pub fn update_cwd_entries(&mut self) {
        self.cwd_entries = get_entries(self.current_dir.as_path()).unwrap();
    }

    pub fn update_parent_dir_entries(&mut self) {
        if let Some(parent) = self.current_dir.parent() {
            self.parent_dir_entries = get_entries(parent).unwrap();
        } else {
            self.parent_dir_entries.clear();
        }
    }

    pub fn update_preview_contents(&mut self) {
        if let Some(selected_idx) = self.list_state.selected() {
            if let Some(entry) = self.cwd_entries.get(selected_idx) {
                if entry.is_dir {
                    self.dir_preview = DirPreview::Directory {
                        entries: get_entries(&entry.path).unwrap(),
                    }
                } else {
                    self.dir_preview = DirPreview::File {
                        contents: fs::read_to_string(&entry.path).unwrap(),
                    }
                }
            }
        }
    }

    pub fn update_all_entries(&mut self) {
        self.update_cwd_entries();
        self.update_parent_dir_entries();
        self.update_preview_contents();
    }

    pub fn next(&mut self) {
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
        self.update_preview_contents();
    }

    pub fn previous(&mut self) {
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
        self.update_preview_contents();
    }

    pub fn enter_selected(&mut self) {
        if let Some(i) = self.list_state.selected() {
            if let Some(entry) = self.cwd_entries.get(i) {
                if entry.is_dir {
                    self.update_cwd(entry.path.clone());
                    self.list_state.select(Some(0));
                }
            }
        }
    }

    pub fn up_dir_level(&mut self) {
        if self.current_dir.parent().is_some() {
            self.update_cwd(self.current_dir.parent().unwrap().to_path_buf());
        }
    }
}
