use crate::stfm::{
    bookmarks::Bookmarks,
    config::Config,
    entries::{FileEntry, get_entries},
    rendering::render,
};
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    widgets::ListState,
};
use std::{fs, path::PathBuf, time::Duration};
use std::{io, path::Path};

#[derive(Debug, Default, PartialEq, Eq)]
enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Debug)]
pub enum Preview {
    File { contents: String },
    Directory { entries: Vec<FileEntry> },
}

// wtf is this
impl Default for Preview {
    fn default() -> Self {
        Self::File {
            contents: String::new(),
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub enum InputMode {
    #[default]
    None,
    Delete,
    NewBookmark {
        input: String,
    },
    ListBookmarks {
        input: String,
    },
    Rename {
        input: String,
    },
    NewEntry {
        input: String,
    },
}

#[derive(Debug, Default)]
pub struct App {
    running_state: RunningState,
    pub list_state: ListState,
    pub bookmark_list_state: ListState,
    pub cwd: PathBuf,
    pub yanked_entry_path: Option<PathBuf>,
    pub is_cut: bool,
    pub parent_dir_entries: Vec<FileEntry>,
    pub cwd_entries: Vec<FileEntry>,
    pub preview_contents: Preview,
    pub input_mode: InputMode,
    pub bookmarks: Bookmarks,
    pub config: Config,
}

// pub functions
impl App {
    pub fn new(path: PathBuf) -> color_eyre::Result<App> {
        let config = Config::load()?;
        let bookmarks = Bookmarks::load()?;

        let mut app = App {
            running_state: RunningState::Running,
            list_state: ListState::default(),
            bookmark_list_state: ListState::default(),
            cwd: path.clone(),
            yanked_entry_path: None,
            is_cut: false,
            parent_dir_entries: Vec::new(),
            cwd_entries: Vec::new(),
            preview_contents: Preview::File {
                contents: String::new(),
            },
            input_mode: InputMode::None,
            bookmarks,
            config,
        };
        app.update_all_entries();
        Ok(app)
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
        self.cwd = path;
        self.list_state.select(Some(0));
        self.update_all_entries();
    }

    fn update_cwd_entries(&mut self) {
        self.cwd_entries =
            get_entries(self.config.ui.show_hidden, self.cwd.as_path()).unwrap_or_default();
    }

    fn update_parent_dir_entries(&mut self) {
        if let Some(parent) = self.cwd.parent() {
            self.parent_dir_entries =
                get_entries(self.config.ui.show_hidden, parent).unwrap_or_default();
        } else {
            self.parent_dir_entries.clear();
        }
    }

    fn update_preview_contents(&mut self) {
        if let Some(selected_idx) = self.list_state.selected() {
            if let Some(entry) = self.cwd_entries.get(selected_idx) {
                if entry.is_dir {
                    let entries =
                        get_entries(self.config.ui.show_hidden, &entry.path).unwrap_or_default();
                    self.preview_contents = Preview::Directory { entries }
                } else {
                    let contents = fs::read_to_string(&entry.path)
                        .unwrap_or_else(|_| "[Binary file or non-UTF-8 content]".to_string());
                    self.preview_contents = Preview::File { contents }
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
                if self.input_mode != InputMode::None {
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
                        KeyCode::Right | KeyCode::Char('l') => {
                            self.enter_selected();
                        }
                        // file operations
                        KeyCode::Char('r') => self.open_rename_popup(),
                        KeyCode::Char('m') => self.open_new_entry_popup(),
                        KeyCode::Char('b') => self.open_bookmark_list_popup(),
                        KeyCode::Char('B') => self.open_new_bookmark_popup(),
                        KeyCode::Char('y') => self.yank(false),
                        KeyCode::Char('x') => self.yank(true),
                        KeyCode::Char('p') => self.paste(),
                        KeyCode::Char('d') => {
                            if self.config.behavior.confirm_delete == false {
                                self.input_mode = InputMode::Delete;
                                self.execute_popup_action()?;
                            } else {
                                self.open_delete_popup()
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn handle_popup_input(&mut self, key_code: KeyCode) -> color_eyre::Result<()> {
        match &mut self.input_mode {
            InputMode::None => {}
            InputMode::Rename { input }
            | InputMode::NewEntry { input }
            | InputMode::NewBookmark { input } => match key_code {
                KeyCode::Esc => {
                    self.input_mode = InputMode::None;
                }
                KeyCode::Enter => {
                    self.execute_popup_action()?;
                }
                KeyCode::Char(c) => {
                    input.push(c);
                }
                KeyCode::Backspace => {
                    input.pop();
                }
                _ => {}
            },
            InputMode::ListBookmarks { input } => match key_code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    self.input_mode = InputMode::None;
                }
                KeyCode::Enter | KeyCode::Char('l') | KeyCode::Right => {
                    // Jump to selected bookmark
                    if let Some(i) = self.bookmark_list_state.selected() {
                        let bookmarks: Vec<_> = self.bookmarks.list();
                        if let Some((tag, bookmark)) = bookmarks.get(i) {
                            if bookmark.path.exists() {
                                self.update_cwd(bookmark.path.clone());
                                self.input_mode = InputMode::None;
                            }
                        }
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    let bookmarks_count = self.bookmarks.list().len();
                    if bookmarks_count > 0 {
                        let i = match self.bookmark_list_state.selected() {
                            Some(i) => {
                                if i >= bookmarks_count - 1 {
                                    0
                                } else {
                                    i + 1
                                }
                            }
                            None => 0,
                        };
                        self.bookmark_list_state.select(Some(i));
                    }
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    let bookmarks_count = self.bookmarks.list().len();
                    if bookmarks_count > 0 {
                        let i = match self.bookmark_list_state.selected() {
                            Some(i) => {
                                if i == 0 {
                                    bookmarks_count - 1
                                } else {
                                    i - 1
                                }
                            }
                            None => 0,
                        };
                        self.bookmark_list_state.select(Some(i));
                    }
                }
                KeyCode::Char('d') => {
                    // Delete selected bookmark
                    if let Some(i) = self.bookmark_list_state.selected() {
                        let bookmarks: Vec<_> = self.bookmarks.list();
                        if let Some((tag, _)) = bookmarks.get(i) {
                            self.bookmarks.remove(tag);
                            let _ = self.bookmarks.save();

                            // Adjust selection
                            let new_count = self.bookmarks.list().len();
                            if new_count == 0 {
                                self.bookmark_list_state.select(None);
                            } else if i >= new_count {
                                self.bookmark_list_state.select(Some(new_count - 1));
                            }
                        }
                    }
                }
                _ => {}
            },
            InputMode::Delete => match key_code {
                KeyCode::Esc | KeyCode::Char('n') => {
                    self.input_mode = InputMode::None;
                }
                KeyCode::Char('y') | KeyCode::Enter => {
                    self.execute_popup_action()?;
                }
                _ => {}
            },
        }
        Ok(())
    }

    fn execute_popup_action(&mut self) -> color_eyre::Result<()> {
        match &self.input_mode {
            InputMode::Rename { input } => {
                if let Some(i) = self.list_state.selected() {
                    if let Some(entry) = self.cwd_entries.get(i) {
                        let new_path = self.cwd.join(input);
                        fs::rename(&entry.path, &new_path)?;
                        self.update_all_entries();
                    }
                }
            }
            InputMode::NewEntry { input } => {
                let new_path = self.cwd.join(input);
                if input.contains('.') {
                    fs::File::create(&new_path)?;
                } else {
                    fs::create_dir_all(&new_path)?;
                }
                self.update_all_entries();
            }
            InputMode::Delete => {
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
            InputMode::ListBookmarks { input } => {
                if let Some(i) = self.bookmark_list_state.selected() {
                    let bookmarks: Vec<_> = self.bookmarks.list();
                    if let Some((tag, bookmark)) = bookmarks.get(i) {
                        if bookmark.path.exists() {
                            self.update_cwd(bookmark.path.clone());
                            self.input_mode = InputMode::None;
                        }
                    }
                }
            }
            InputMode::NewBookmark { input } => {
                self.bookmarks.add(input.clone(), self.cwd.clone());
                let _ = self.bookmarks.save();
            }
            InputMode::None => {}
        }
        self.input_mode = InputMode::None;
        Ok(())
    }

    fn open_rename_popup(&mut self) {
        if let Some(i) = self.list_state.selected() {
            if let Some(entry) = self.cwd_entries.get(i) {
                self.input_mode = InputMode::Rename {
                    input: entry.name.clone(),
                };
            }
        }
    }

    fn open_delete_popup(&mut self) {
        self.input_mode = InputMode::Delete;
    }

    fn open_new_entry_popup(&mut self) {
        self.input_mode = InputMode::NewEntry {
            input: String::new(),
        };
    }

    fn open_bookmark_list_popup(&mut self) {
        self.input_mode = InputMode::ListBookmarks {
            input: String::new(),
        };
        self.bookmark_list_state.select(Some(0));
    }

    fn open_new_bookmark_popup(&mut self) {
        self.input_mode = InputMode::NewBookmark {
            input: String::new(),
        };
    }

    fn yank(&mut self, cut: bool) {
        if let Some(i) = self.list_state.selected() {
            if let Some(entry) = self.cwd_entries.get(i) {
                self.yanked_entry_path = Some(entry.path.clone());
                self.is_cut = cut;
            }
        }
    }

    fn paste(&mut self) {
        if let Some(source) = &self.yanked_entry_path {
            if let Some(filename) = source.file_name() {
                let mut destination = self.cwd.join(filename);

                if destination.exists() {
                    let stem = destination
                        .file_stem()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .into_owned();
                    let ext = destination
                        .extension()
                        .map(|e| format!(".{}", e.to_string_lossy()))
                        .unwrap_or_default();

                    let mut counter = 1;
                    loop {
                        destination = self.cwd.join(format!("{}_copy{}{}", stem, counter, ext));
                        if !destination.exists() {
                            break;
                        }
                        counter += 1;
                    }
                }

                let result = if source.is_file() {
                    fs::copy(source, &destination).map(|_| ())
                } else if source.is_dir() {
                    copy_dir_recursively(source, &destination)
                } else {
                    return;
                };

                if result.is_ok() {
                    if self.is_cut {
                        if source.is_file() {
                            let _ = fs::remove_file(source);
                        } else if source.is_dir() {
                            let _ = fs::remove_dir_all(source);
                        }
                    }

                    self.yanked_entry_path = None;
                    self.is_cut = false;
                    self.update_all_entries();
                }
            }
        }
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
                    self.list_state.select(Some(0));
                    self.update_cwd(entry.path.clone());
                }
            }
        }
    }

    fn up_dir_level(&mut self) {
        if let Some(parent) = self.cwd.parent() {
            self.update_cwd(parent.to_path_buf());
        }
    }
}

fn copy_dir_recursively(src: &Path, dst: &Path) -> io::Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_recursively(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}
