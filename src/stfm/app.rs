use crate::stfm::{
    entries::{FileEntry, get_entries},
    rendering::render,
};
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    widgets::ListState,
};
use std::result::Result::Ok;
use std::{fs, path::PathBuf, time::Duration};

#[derive(Debug, Default, PartialEq, Eq)]
enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Debug)]
pub enum DirPreview {
    File { contents: String },
    Directory { entries: Vec<FileEntry> },
}

// wtf is this
impl Default for DirPreview {
    fn default() -> Self {
        Self::File {
            contents: String::new(),
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub enum PopupMode {
    #[default]
    None,
    Rename {
        input: String,
    },
    Delete {
        confirm: bool,
    },
    NewEntry {
        input: String,
    },
}

#[derive(Debug, Default)]
pub struct App {
    running_state: RunningState,
    pub list_state: ListState,
    pub current_dir: PathBuf,
    pub yanked_entry_path: PathBuf,
    pub parent_dir_entries: Vec<FileEntry>,
    pub cwd_entries: Vec<FileEntry>,
    pub dir_preview: DirPreview,
    pub popup_mode: PopupMode,
}

// pub functions
impl App {
    pub fn new(path: PathBuf) -> App {
        let mut app = App {
            running_state: RunningState::Running,
            list_state: ListState::default(),
            current_dir: path.clone(),
            yanked_entry_path: PathBuf::new(),
            parent_dir_entries: Vec::new(),
            cwd_entries: Vec::new(),
            dir_preview: DirPreview::File {
                contents: String::new(),
            },
            popup_mode: PopupMode::None,
        };
        app.update_all_entries();
        app
    }

    pub fn run(&mut self) -> color_eyre::Result<()> {
        let mut terminal = ratatui::init();
        while self.running_state != RunningState::Done {
            terminal.draw(|frame| render(self, frame))?;

            if event::poll(Duration::from_millis(100))? {
                self.handle_input()?;
            }
        }
        ratatui::restore();
        Ok(())
    }
}

// App path manip functions
impl App {
    fn update_cwd(&mut self, path: PathBuf) {
        self.current_dir = path;
        self.update_all_entries();
    }

    fn update_cwd_entries(&mut self) {
        self.cwd_entries = get_entries(self.current_dir.as_path()).unwrap();
    }

    fn update_parent_dir_entries(&mut self) {
        if let Some(parent) = self.current_dir.parent() {
            self.parent_dir_entries = get_entries(parent).unwrap();
        } else {
            self.parent_dir_entries.clear();
        }
    }

    fn update_preview_contents(&mut self) {
        if let Some(selected_idx) = self.list_state.selected() {
            if let Some(entry) = self.cwd_entries.get(selected_idx) {
                if entry.is_dir {
                    self.dir_preview = DirPreview::Directory {
                        entries: get_entries(&entry.path).unwrap_or_default(),
                    }
                } else {
                    // this is basically the solution for files that are not utf8 since i can't
                    // figure out how to filter the entries out
                    // read as utf8 or fallback message!
                    let contents = match fs::read_to_string(&entry.path) {
                        Ok(text) => text,
                        Err(_) => "[Binary file or non-UTF-8 content]".to_string(),
                    };
                    self.dir_preview = DirPreview::File { contents }
                }
            }
        }
    }

    fn update_all_entries(&mut self) {
        self.update_cwd_entries();
        self.update_parent_dir_entries();
        self.update_preview_contents();
    }
}

// App input
impl App {
    fn handle_input(&mut self) -> color_eyre::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                if self.popup_mode != PopupMode::None {
                    self.handle_popup_input(key.code)?;
                } else {
                    match key.code {
                        // normal navigation
                        KeyCode::Esc | KeyCode::Char('q') => {
                            self.running_state = RunningState::Done
                        }
                        KeyCode::Down | KeyCode::Char('j') => self.next(),
                        KeyCode::Up | KeyCode::Char('k') => self.previous(),
                        KeyCode::Left | KeyCode::Char('h') => self.up_dir_level(),
                        KeyCode::Enter | KeyCode::Right | KeyCode::Char('l') => {
                            self.enter_selected();
                        }
                        // file operations
                        KeyCode::Char('r') => self.open_rename_popup(),
                        KeyCode::Char('d') => self.open_delete_popup(),
                        KeyCode::Char('m') => self.open_new_entry_popup(),
                        KeyCode::Char('y') => self.yank(),
                        KeyCode::Char('p') => self.paste(),
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }
    fn handle_popup_input(&mut self, key_code: KeyCode) -> color_eyre::Result<()> {
        match &mut self.popup_mode {
            PopupMode::None => {}
            PopupMode::Rename { input } | PopupMode::NewEntry { input } => match key_code {
                KeyCode::Esc => {
                    self.popup_mode = PopupMode::None;
                }
                KeyCode::Enter => {
                    self.execute_popup_action()?;
                    self.popup_mode = PopupMode::None;
                }
                KeyCode::Char(c) => {
                    input.push(c);
                }
                KeyCode::Backspace => {
                    input.pop();
                }
                _ => {}
            },
            PopupMode::Delete { confirm } => match key_code {
                KeyCode::Esc | KeyCode::Char('n') => {
                    self.popup_mode = PopupMode::None;
                }
                KeyCode::Char('y') | KeyCode::Enter | KeyCode::Char('d') => {
                    self.execute_popup_action()?;
                    self.popup_mode = PopupMode::None;
                }
                _ => {}
            },
        }
        Ok(())
    }

    fn execute_popup_action(&mut self) -> color_eyre::Result<()> {
        match &self.popup_mode {
            PopupMode::Rename { input } => {
                if let Some(i) = self.list_state.selected() {
                    if let Some(entry) = self.cwd_entries.get(i) {
                        let new_path = self.current_dir.join(input);
                        fs::rename(&entry.path, &new_path)?;
                        self.update_all_entries();
                    }
                }
            }
            // SO if the input contains a . somewhere it's a folder :P
            // this does not support creating hidden folders yet, so stuff like .config
            PopupMode::NewEntry { input } => {
                if input.contains('.') {
                    let new_path = self.current_dir.join(input);
                    fs::File::create(&new_path)?;
                    self.update_all_entries();
                } else {
                    let new_path = self.current_dir.join(input);
                    fs::create_dir(&new_path)?;
                    self.update_all_entries();
                }
            }
            PopupMode::Delete { .. } => {
                if let Some(i) = self.list_state.selected() {
                    if let Some(entry) = self.cwd_entries.get(i) {
                        if entry.is_dir {
                            fs::remove_dir_all(&entry.path)?;
                        } else {
                            fs::remove_file(&entry.path)?;
                        }
                        self.update_all_entries();
                    }
                }
            }
            PopupMode::None => {}
        }
        Ok(())
    }

    fn open_rename_popup(&mut self) {
        if let Some(i) = self.list_state.selected() {
            if let Some(entry) = self.cwd_entries.get(i) {
                self.popup_mode = PopupMode::Rename {
                    input: entry.name.clone(),
                };
            }
        }
    }

    fn open_delete_popup(&mut self) {
        self.popup_mode = PopupMode::Delete { confirm: false };
    }

    fn open_new_entry_popup(&mut self) {
        self.popup_mode = PopupMode::NewEntry {
            input: String::new(),
        };
    }

    fn yank(&mut self) {
        if let Some(i) = self.list_state.selected() {
            if let Some(entry) = self.cwd_entries.get(i) {
                self.yanked_entry_path = entry.path.clone();
            }
        }
    }

    fn paste(&self) {
        _ = fs::copy(self.yanked_entry_path.clone(), self.current_dir.clone());
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
        self.update_preview_contents();
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
        self.update_preview_contents();
    }

    fn enter_selected(&mut self) {
        if let Some(i) = self.list_state.selected() {
            if let Some(entry) = self.cwd_entries.get(i) {
                if entry.is_dir {
                    self.update_cwd(entry.path.clone());
                    self.list_state.select(Some(0));
                }
            }
        }
    }

    fn up_dir_level(&mut self) {
        if self.current_dir.parent().is_some() {
            self.update_cwd(self.current_dir.parent().unwrap().to_path_buf());
        }
    }
}
